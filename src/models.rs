use serde::{Deserialize, Serialize};
use chrono::naive::NaiveDate;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TomlFile {
    pub member: Vec<Member>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Member {
    pub id: Option<String>,
    pub display: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub date_of_death: Option<NaiveDate>,
    pub parents: Option<Vec<String>>,
    pub sex: Option<String>,
}
