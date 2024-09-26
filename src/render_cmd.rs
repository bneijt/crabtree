use crate::models::TomlFile;
use chrono::naive::NaiveDate;
use chrono::Datelike;

use murmur3::murmur3_32;
use serde::Serialize;
use std::any::Any;
use crate::models::EventType;
use std::collections::HashSet;
use std::{collections::HashMap, io::Cursor};
use tera::{Context, Tera};

const TEMPLATE: &str = r#"
flowchart TD
{% for node in graph_nodes %}
    {{node.id}}("
{{node.name}}
{% if node.date_of_birth %}🎂 {{ node.date_of_birth }}{% endif %}
{% if node.date_of_death %}🪦 {{ node.date_of_death }}{% endif %}
{% if node.age %}age {{ node.age }}{% endif %}
")
{% endfor %}
{% for joining_node in joining_nodes %}
    {{joining_node.id}}(⚤)
    {% for input in joining_node.inputs %}{{input}} --- {{joining_node.id}}
    {% endfor %}
    {% for output in joining_node.outputs %}{{joining_node.id}} --- {{output}}
    {% endfor %}    
{% endfor %}
"#;

// #[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Clone, Serialize)]
#[derive(Serialize, Clone)]
struct JoiningNode {
    id: String,
    inputs: HashSet<String>,
    outputs: HashSet<String>,
}

#[derive(Serialize, Clone)]
struct GraphNode {
    id: String,
    name: String,
    date_of_birth: String,
    date_of_death: String,
    age: String,
}

pub async fn render_database(database: TomlFile) -> String {
    let mut members = database.member;
    members.sort_by(|a, b| a.date_of_birth.cmp(&b.date_of_birth));

    // // For each parent combination, create a combine node: ⚤
    let joining_node_duplicates: Vec<JoiningNode> = members
        .iter()
        .flat_map(|member| {
            let parents = member.parents.clone();
            let hash_result = murmur3_32(&mut Cursor::new(parents.join("|")), 0).unwrap();

            let combination_nodes: Vec<JoiningNode> = vec![JoiningNode {
                id: hash_result.to_string(),
                inputs: parents.into_iter().collect(),
                outputs: HashSet::from([member.id.clone().unwrap()]),
            }];
            combination_nodes
        })
        .collect();

    //Merge outputs that have the same JoiningNode id
    let mut joining_nodes: HashMap<String, JoiningNode> = HashMap::new();
    for joining_node in joining_node_duplicates {
        if joining_node.inputs.is_empty() {
            continue;
        }
        if joining_nodes.contains_key(&joining_node.id) {
            joining_nodes
                .get_mut(&joining_node.id)
                .unwrap()
                .outputs
                .extend(joining_node.outputs);
        } else {
            joining_nodes.insert(joining_node.id.clone(), joining_node);
        }
    }
    let today: NaiveDate = chrono::Local::now().naive_local().date();
    let deaths: HashMap<String, NaiveDate> = database
        .event
        .iter()
        .filter(|e| e.event_type == EventType::Died)
        .flat_map(|e| e.participants.iter().map(|p| (p.clone(), e.date.unwrap())))
        .collect();

    let graph_nodes: Vec<GraphNode> = members
        .iter()
        .map(|member| {
            let member_id = member.id.clone().unwrap();
            let age: String = match member.date_of_birth {
                Some(dob) => {
                    let end = deaths.get(&member_id).unwrap_or(&today);
                    let mut age = end.year() - dob.year();
                    if (end.month(), end.day()) < (dob.month(), dob.day()) {
                        age -= 1;
                    }
                    age.to_string()
                }
                None => String::new(),
            };
            GraphNode {
                id: member_id.clone(),
                name: member.name.clone(),
                date_of_birth: member
                    .date_of_birth
                    .map(|d| d.to_string())
                    .unwrap_or(String::from("")),
                date_of_death: deaths
                    .get(&member_id)
                    .map(|d| d.to_string())
                    .unwrap_or(String::from("")),
                age,
            }
        })
        .collect();

    let mut context = Context::new();
    context.insert("graph_nodes", &graph_nodes);
    context.insert(
        "joining_nodes",
        &joining_nodes.values().collect::<Vec<&JoiningNode>>(),
    );
    // add stuff to context
    let result = Tera::one_off(TEMPLATE, &context, false);
    result.unwrap().replace("\n\n", "\n")
}
