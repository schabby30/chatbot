use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;
use mongodb::bson::Uuid;
use crate::{get_groq_api_key, groq_chatbot::GroqChatbot, messages::{QueryRequest, SystemPromptRequest}, mongo::{get_chat, insert_system_prompt_to_db}};

/// Endpoint to handle POST request to /add_system_prompt
#[post("/add_system_prompt")]
pub async fn add_system_prompt(data: web::Json<SystemPromptRequest>) -> impl Responder { 
    add_new_system_prompt(data.model.clone(), data.prompt.clone()).await
}

#[post("/ask_groq")]
pub async fn ask_groq(data: web::Json<QueryRequest>) -> impl Responder {
    let query = &data.query;

    // Error handling if query is empty
    if query.is_empty() {
        return HttpResponse::BadRequest().json(json!({"error": "No query provided"}));
    }
    
    let chatbot = GroqChatbot::new(get_groq_api_key().to_string(), "arlista.csv");

    match chatbot.process_query("123", query.as_str()).await {
        Ok(content) => HttpResponse::Ok().json(json!({"response": content})),
        Err(err) => HttpResponse::InternalServerError().json(json!({"error": err.to_string()})),
    }
}

/// Endpoint to handle GET requests to /get_groq_chat
#[get("/get_groq_chat")]
pub async fn get_groq_chat(id: web::Query<Uuid>) -> impl Responder {
    match get_chat(id.into_inner()).await {
        Ok(chat) => HttpResponse::Ok().json(json!({"id": chat.id, "query": chat.query, "answer": chat.answer})),
        Err(err) => HttpResponse::Ok().body(err),
    }
}

pub async fn add_new_system_prompt(model: String, prompt: String) -> impl Responder { 
    match insert_system_prompt_to_db(model, prompt).await {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().json(json!({"error": err.to_string()})),
    }
}