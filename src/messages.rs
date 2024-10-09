use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct SystemPrompt {
    pub id: Uuid,
    pub model: String,
    pub prompt: String,
}

#[derive(Deserialize)]
pub struct SystemPromptRequest {
    pub model: String,
    pub prompt: String,
}

#[derive(Deserialize)]
pub struct QueryRequest {
    pub query: String,
}

impl SystemPrompt {
    pub fn new(model: String, prompt: String) -> Self {
        Self {
            id: Uuid::new(),
            model,
            prompt,
        }
    }
}

impl SystemPromptRequest {
    pub fn new(model: String, prompt: String) -> Self {
        Self {
            model,
            prompt,
        }
    }
}