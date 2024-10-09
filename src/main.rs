#[allow(dead_code)]

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use chatbot::{actix_base::{add_system_prompt, ask_groq, get_groq_chat}, get_mongo_client, init_mongo_client, init_system_prompt};
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load environment variables
    init_mongo_client().await; // Initialize MongoDB Client
    init_system_prompt().await; // Initialize system prompts

    HttpServer::new(move || {
        let cors = Cors::permissive(); // Allow all CORS for simplicity
        App::new()
            .app_data(web::Data::new(get_mongo_client().clone()))
            .wrap(cors)
            .service(ask_groq)
            .service(get_groq_chat)
            .service(add_system_prompt)
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
