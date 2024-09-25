use crate::models;
use std::path::Path;
use tokio::fs;
use uuid::Uuid;

pub async fn assign_ids(
    config_file: &Path,
) -> Result<models::TomlFile, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(config_file).await?;

    // Deserialize the TOML string into a MyConfig struct
    let mut config: models::TomlFile = toml::from_str(&contents)?;

    for member in config.member.iter_mut() {
        if member.id.is_none() {
            member.id = Some(Uuid::new_v4().to_string());
        }
        //Parents are sorted to allow for hashing for join nodes.
        member.parents.sort();
        member.parents.dedup();
    }
    // Write back the toml file to update the contents
    let toml_string = toml::to_string(&config)?;
    fs::write(config_file, toml_string).await?;


    Ok(config)

}
