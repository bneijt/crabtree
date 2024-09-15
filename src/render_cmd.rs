use crate::models::TomlFile;
use murmur3::murmur3_32;
use std::io::Cursor;

pub async fn render_database(database: TomlFile) -> String {
    let mut members = database.member;
    members.sort_by(|a, b| a.date_of_birth.cmp(&b.date_of_birth));

    let header = [String::from("flowchart TD")];
    let member_lines = members
        .iter()
        .map(|member| format!("{}({})", member.id.clone().unwrap(), member.display));

    // For each parent combination, create a combine node: ⚤
    let mut parent_combination_nodes: Vec<String> = members.iter().flat_map(|member| {
        let combination_nodes: Vec<String> = member
            .parents
            .clone()
            .map(|parents| {
                let mut joined_node_value = parents.join("|");
                let hash_result = murmur3_32(&mut Cursor::new(joined_node_value), 0).unwrap();
                let mut nds: Vec<String> = Vec::new();
                nds.push(format!("{:x}(⚤)", hash_result));
                nds.extend(parents.iter().map(|parent_id| format!("{} --- {:x}", parent_id, hash_result)));
                nds.push(format!("{:x} --- {}", hash_result, member.id.clone().unwrap()));
                nds
            }).unwrap_or(Vec::new());
            combination_nodes
    }).collect();
    parent_combination_nodes.sort_unstable();
    parent_combination_nodes.dedup();

    //Join lines into a single newline split String
    let content: Vec<String> = header.into_iter().chain(member_lines).chain(parent_combination_nodes).collect();
    content.join("\n").into()
}
