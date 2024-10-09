use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ChatModel {
    pub id: Uuid,
    pub query: String,
    pub answer: String,
}

impl ChatModel {
    pub fn new(id: Uuid, query: String, answer: String) -> Self {
        Self {
            id,
            query,
            answer,
        }
    }
}