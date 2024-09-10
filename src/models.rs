use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TomlFile {
    pub member: Vec<Member>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Member {
    pub id: Option<String>,
    pub display: String,
    pub first_name: String,
    pub last_name: String,
}
