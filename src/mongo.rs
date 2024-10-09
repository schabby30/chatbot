use mongodb::{bson::{doc, Uuid}, Collection};
use crate::{db_model::ChatModel, get_mongo_client, messages::SystemPrompt};

const DB_NAME: &str = "chats_db";
const CHAT_COLL_NAME: &str = "chats";
const SYSTEM_COLL_NAME: &str = "system";

/// Adds a new chat to the "chats" collection in the database.
pub async fn add_chat(message: &ChatModel) -> Result<(), String> {
    let collection: Collection<ChatModel> = get_mongo_client().database(DB_NAME).collection(CHAT_COLL_NAME);
    let result = collection.insert_one(message).await;
    match result {
        Ok(_) => return Ok(()),
        Err(err) => return Err(err.to_string()),
    }
}

/// Gets the chat with the supplied id.
pub async fn get_chat(id: Uuid) -> Result<ChatModel, String> {
    let collection: Collection<ChatModel> = get_mongo_client().database(DB_NAME).collection(CHAT_COLL_NAME);
    match collection.find_one(doc! { "id": &id }).await {
        Ok(Some(chat)) => return Ok(chat.into()),
        Ok(None) => return Err("Not found".to_string()),
        Err(err) => return Err(err.to_string()),
    }
}

pub async fn insert_system_prompt_to_db(model: String, prompt: String) -> Result<(), String> {
    let system_prompt = SystemPrompt::new(model, prompt);
    let collection: Collection<SystemPrompt> = get_mongo_client().database(DB_NAME).collection(SYSTEM_COLL_NAME);
    let result = collection.insert_one(system_prompt).await;
    match result {
        Ok(_) => return Ok(()),
        Err(err) => return Err(err.to_string()),
    }
}

/// Gets the chat with the supplied id.
pub async fn get_system_prompts() -> Result<Vec<SystemPrompt>, String> {
    let collection: Collection<SystemPrompt> = get_mongo_client().database(DB_NAME).collection(SYSTEM_COLL_NAME);
    // Find all documents (empty filter means retrieve all)
    let mut cursor = match collection.find(doc! {}).await {
        Ok(cursor) => cursor,
        Err(e) => return Err(format!("Error finding documents: {}", e)),
    };

    let mut prompts = Vec::new();

    // Iterate over the cursor and collect each document
    loop {
        match cursor.advance().await {
            Ok(true) => {
                match cursor.deserialize_current() {
                    Ok(prompt) => prompts.push(prompt),
                    Err(err) => return Err(format!("Deserialization error: {}", err)),
                }
            },
            Ok(false) => break, // No more documents, exit the loop
            Err(err) => return Err(format!("DB error: {}", err)),
        }
    }

    Ok(prompts)
}