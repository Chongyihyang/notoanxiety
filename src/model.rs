use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PostInfo {
    pub title: String,
    pub date: String,
    pub description: String
}

impl PostInfo {
    pub fn new(title: String, date: String, description: String) -> Self {
        Self {title, date, description}
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub password: String
}
