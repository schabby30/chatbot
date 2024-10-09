use std::{collections::HashMap, fs, sync::{Arc, Mutex}};
use mongodb::bson::Uuid;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use crate::{db_model::ChatModel, mongo::{add_chat, get_system_prompts}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroqMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct GroqRequest {
    pub messages: Vec<GroqMessage>,
    pub model: String,
    pub temperature: Option<f64>,
    max_tokens: i32,
}

#[derive(Deserialize)]
pub struct GroqResponse {
    pub choices: Vec<GroqResponseChoice>,
    pub created: u64,
    pub id: String,
    pub model: String,
    pub object: String,
    pub system_fingerprint: String,
    pub usage: Usage,
    pub x_groq: XGroq,
}

#[derive(Deserialize)]
pub struct GroqResponseChoice {
    pub message: GroqMessage,
}

#[derive(Deserialize)]
pub struct Usage {
    pub completion_time: f64,
    pub completion_tokens: u64,
    pub prompt_time: f64,
    pub prompt_tokens: u64,
    pub total_time: f64,
    pub total_tokens: u64,
}

#[derive(Deserialize)]
pub struct XGroq {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroqConversation {
    pub messages: Vec<GroqMessage>,
    pub document_context: String,
}

#[derive(Debug, Default)]
pub struct GroqConversationCache {
    pub cache: HashMap<String, GroqConversation>,
}

impl GroqConversationCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn get_conversation(&self, id: &str) -> Option<GroqConversation> {
        self.cache.get(id).cloned()
    }

    pub fn update_conversation(&mut self, id: String, conversation: GroqConversation) {
        self.cache.insert(id, conversation);
    }
}

pub struct GroqChatbot {
    pub client: reqwest::Client,
    pub api_key: String,
    pub cache: Arc<Mutex<GroqConversationCache>>,
    pub knowledge_base: String,
}

impl GroqChatbot {
    pub fn new(api_key: String, knowledge_base_path: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to create HTTP client");

        let knowledge_base = fs::read_to_string(knowledge_base_path)
            .expect("Failed to read knowledge base file");

        Self {
            client,
            api_key,
            cache: Arc::new(Mutex::new(GroqConversationCache::new())),
            knowledge_base,
        }
    }

    pub async fn process_query(&self, conversation_id: &str, user_input: &str) -> Result<String, String> {
        let mut cache = self.cache.lock().expect("Lock should have been acquired.");
        
        // Get or create conversation
        let mut conversation = cache
            .get_conversation(conversation_id)
            .unwrap_or_else(|| GroqConversation {
                messages: Vec::new(),
                document_context: self.knowledge_base.clone(),
            });

        // Add user message
        conversation.messages.push(GroqMessage {
            role: "user".to_string(),
            content: user_input.to_string(),
        });

        // Collect system prompts if available
        let mut system_prompt = "".to_string();
        if let Ok(system_prompts) = get_system_prompts().await {
            system_prompt = system_prompts.iter()
                                .fold("".to_string(), 
                                    |mut acc, prompt| {
                                        acc.push_str(prompt.prompt.as_str());
                                        acc
                                    }
                                );
        }

        // Add a system message (optional for instructions)
        if !system_prompt.is_empty() {
            conversation.messages.insert(0, GroqMessage {
                role: "system".to_string(),
                content: system_prompt,
            });
        }

        // Prepare the API request for Groq
        let request = GroqRequest {
            max_tokens: 1024,
            messages: conversation.messages.clone(),
            model: "llama3-8b-8192".to_string(),
            temperature: Some(0.5),
        };

        // Call Groq API
        let groq_url = "https://api.groq.com/openai/v1/chat/completions";
        let response = self
            .client
            .post(groq_url)
            .header("Authorization", format!("Bearer {}", &self.api_key))
            .json(&request)
            .send()
            .await
            .expect("Failed to send request to Groq API");

        let response_body = response.text().await.expect("Failed to get response text");

        let groq_response: GroqResponse = serde_json::from_str(&response_body)
                                                .expect("Failed to parse Groq API response");

        let content = &groq_response.choices[0].message.content;

        let chat_model = ChatModel::new(Uuid::new(), user_input.to_string(), content.to_string());

        match add_chat(&chat_model).await {
            Ok(_) => {},
            Err(err) => return Err(err.to_string()),
        };

        // Update conversation history
        conversation.messages.push(GroqMessage {
            role: "assistant".to_string(),
            content: content.clone(),
        });

        // Update cache
        cache.update_conversation(conversation_id.to_string(), conversation);

        Ok(content.clone())
    }
}
