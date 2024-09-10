use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TomlFile {
    member: Vec<Member>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Member {
    id: Option<String>,
    display: String,
    first_name: String,
    last_name: String,
}
