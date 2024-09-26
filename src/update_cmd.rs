use crate::models::{self, EventType};
use std::path::Path;
use tokio::fs;
use toml_edit::{value, DocumentMut, Value, Array};
use uuid::Uuid;

pub async fn assign_ids(config_file: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(config_file).await?;

    let mut document: DocumentMut = contents.parse().unwrap();

    for member in document["member"]
        .as_array_of_tables_mut()
        .unwrap()
        .iter_mut()
    {
        if !member.contains_key("id") {
            member["id"] = value(Uuid::new_v4().to_string());
        }
        //Parents are sorted to allow for hashing for join nodes.
        if member.contains_key("parents") {
            let mut parents_vec: Vec<String> = member
                .get_mut("parents")
                .map(|a| a.as_array_mut().unwrap().clone())
                .map(|ar| ar.iter().map(|v| v.as_str().unwrap().to_string()).collect())
                .unwrap_or_default();
            parents_vec.sort();
            parents_vec.dedup();

            let parents_array: toml_edit::Array = parents_vec.into_iter().collect();
            member["parents"] = value(parents_array);
                   }
    }

    fs::write(config_file, document.to_string()).await?;

    Ok(())
}

pub async fn load_database(
    config_file: &Path,
) -> Result<models::TomlFile, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(config_file).await?;
    let config: models::TomlFile = toml::from_str(&contents)?;

    // check config
    for event in config.event.iter() {
        if event.participants.is_empty() {
            return Err("Event has no participants".into());
        }
        match event.event_type {
            EventType::Died => {
                if event.date.is_none() {
                    return Err("Event has no date".into());
                }
            }
            _ => (),
        }
    }

    Ok(config)
}
