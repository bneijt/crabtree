use crate::models::TomlFile;
use chrono::naive::NaiveDate;
use chrono::Datelike;

use murmur3::murmur3_32;
use serde::Serialize;
use std::collections::HashSet;
use std::{collections::HashMap, io::Cursor};
use tera::{Context, Tera};

const TEMPLATE: &str = r#"
flowchart TD
{% for member in members %}
    {{member.id}}("
{{member.name}}
{% if member.date_of_birth %}🎂 {{ member.date_of_birth }}{% endif %}
{% if member.date_of_death %}🪦 {{ member.date_of_death }}{% endif %}
{% if member.date_of_birth %}age {{ ages[member.id] }}{% endif %}
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

    let ages: HashMap<String, u32> = members
        .iter()
        .filter(|m| m.date_of_birth.is_some())
        .map(|member| {
            let dob = member.date_of_birth.unwrap();
            let mut age = today.year() - dob.year();
            if (today.month(), today.day()) < (dob.month(), dob.day()) {
                age -= 1;
            }
            (member.id.clone().unwrap(), age as u32)
        })
        .collect();

    let mut context = Context::new();
    context.insert("members", &members);
    context.insert(
        "joining_nodes",
        &joining_nodes.values().collect::<Vec<&JoiningNode>>(),
    );
    context.insert("ages", &ages);
    // add stuff to context
    let result = Tera::one_off(TEMPLATE, &context, false);
    result.unwrap().replace("\n\n", "\n")
}
