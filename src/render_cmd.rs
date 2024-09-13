use crate::models::TomlFile;

pub async fn render_database(database: TomlFile) -> String {
    let mut members = database.member;
    members.sort_by(|a, b| a.date_of_birth.cmp(&b.date_of_birth));

    let header = [String::from("flowchart TD")];
    let member_lines = members
        .iter()
        .map(|member| format!("{}({})", member.id.clone().unwrap(), member.display));

    let parent_lines = members.iter().flat_map(|member| {
        member
            .parents
            .clone()
            .unwrap_or(Vec::new())
            .iter()
            .map(|parent_id| {
                format!(
                    "{} -- parent --- {}",
                    parent_id,
                    member.id.clone().unwrap()
                )
            })
            .collect::<Vec<String>>()
    });

    //Join lines into a single newline split String
    let content: Vec<String> = header.into_iter().chain(member_lines).chain(parent_lines).collect();
    content.join("\n").into()
}
