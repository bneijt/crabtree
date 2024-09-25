use chrono::naive::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TomlFile {
    pub member: Vec<Member>,
    
    #[serde(default)]
    pub event: Vec<Event>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Member {
    pub id: Option<String>,
    pub name: String,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    #[serde(default)]
    pub parents: Vec<String>,
    pub sex: Option<Sex>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Sex {
    Male,
    Female,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub participants: Vec<String>,
    pub date: Option<NaiveDate>,
    pub place: Option<String>,
    pub event_type: EventType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EventType {
    Died,
    Married,
    Baptized,
}
