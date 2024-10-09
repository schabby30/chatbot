pub mod actix_base;
pub mod db_model;
pub mod mongo;
pub mod groq_chatbot;
pub mod messages;

use std::{env, sync::OnceLock};
use mongo::get_system_prompts;
use reqwest::Client as ReqwestClient;
use mongodb::Client as MongoClient;

static ANTHROPIC_API_KEY: OnceLock<String> = OnceLock::new();
static GROQ_API_KEY: OnceLock<String> = OnceLock::new();
static MONGODB_URI: OnceLock<String> = OnceLock::new();
static ACTIX_CLIENT: OnceLock<ReqwestClient> = OnceLock::new();
static MONGO_CLIENT: OnceLock<MongoClient> = OnceLock::new();
static DEFAULT_SYSTEM_PROMPT: OnceLock<String> = OnceLock::new();

/// Retrieve API key from .env
pub fn get_openai_api_key() -> &'static String {
    ANTHROPIC_API_KEY.get_or_init(|| {
        env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found")
    })
}

/// Retrieve API key from .env
pub fn get_groq_api_key() -> &'static String {
    GROQ_API_KEY.get_or_init(|| {
        env::var("GROQ_API_KEY").expect("GROQ_API_KEY not found")
    })
}

/// Retrieve MONGODB URI from .env
pub fn get_mongodb_uri() -> &'static String {
    MONGODB_URI.get_or_init(|| {
        env::var("MONGODB_URI").expect("MONGODB URI not found")
    })
}

/// Get Actix-web client
pub fn get_reqwest_client() -> &'static ReqwestClient {
    ACTIX_CLIENT.get_or_init(|| {
        ReqwestClient::new()
    })
}

/// Asynchronously initialize MongoDB client (call this at startup)
pub async fn init_mongo_client() {
    let client = MongoClient::with_uri_str(get_mongodb_uri())
        .await
        .expect("Failed to connect to MongoDB");
    
    // Store the initialized MongoClient in the OnceLock
    MONGO_CLIENT.set(client).expect("Failed to set MongoClient");
}

/// Provide a static, synchronous getter for the MongoDB client
pub fn get_mongo_client() -> &'static MongoClient {
    MONGO_CLIENT.get().expect("MongoClient is not initialized")
}

/// Retrieve DEFAULT_SYSTEM_PROMPT from .env
pub async fn get_default_system_prompt() -> &'static String {
    DEFAULT_SYSTEM_PROMPT.get_or_init(|| {
        env::var("DEFAULT_SYSTEM_PROMPT").expect("DEFAULT_SYSTEM_PROMPT not found")
    })
}

pub async fn init_system_prompt() {
    match get_system_prompts().await {
        Ok(prompts) => if prompts.is_empty() {
            actix_base::add_new_system_prompt("claude".to_string(), get_default_system_prompt().await.clone()).await;
        },
        Err(_) => {},
    }
}