use anyhow::Result;
use async_trait::async_trait;
use reqwest;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::llm::LLMService;
use crate::llm::LLMThreadMessage;

#[derive(Copy, Clone, Debug)]
pub enum GroqCompletionModel {
    Llama3_8b,
    Llama3_70b,
}

impl Default for GroqCompletionModel {
    fn default() -> Self {
        GroqCompletionModel::Llama3_70b
    }
}

impl FromStr for GroqCompletionModel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "llama3-8b-8192" => Ok(GroqCompletionModel::Llama3_8b),
            "llama3-70b-8192" => Ok(GroqCompletionModel::Llama3_70b),
            _ => Err(()),
        }
    }
}

impl GroqCompletionModel {
    pub fn as_str(&self) -> &'static str {
        match *self {
            GroqCompletionModel::Llama3_8b => "llama3-8b-8192",
            GroqCompletionModel::Llama3_70b => "llama3-70b-8192",
        }
    }

    pub fn default_string() -> String {
        let mock_completion_model: GroqCompletionModel = Default::default();
        mock_completion_model.as_str().to_string()
    }
}

pub struct Groq {
    pub completion_model: GroqCompletionModel,
    pub api_key: String,
}

pub fn all_completions() -> [GroqCompletionModel; 2] {
    [
        GroqCompletionModel::Llama3_8b,
        GroqCompletionModel::Llama3_70b,
    ]
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroqMessage {
    content: String,
    role: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroqRequest {
    messages: Vec<GroqMessage>,
    model: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GroqResponseMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GroqResponseChoice {
    index: i64,
    message: GroqResponseMessage,
    finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GroqResponseUsage {
    prompt_tokens: i64,
    prompt_time: f64,
    completion_tokens: i64,
    completion_time: f64,
    total_tokens: f64,
    total_time: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct GroqResponse {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<GroqResponseChoice>,
    usage: GroqResponseUsage,
}

#[async_trait]
impl LLMService for Groq {
    fn bot_info(&self) -> String {
        format!("Bot uses Groq with {}", self.completion_model.as_str())
    }

    async fn get_answer(&self, thread_messages: Vec<LLMThreadMessage>) -> anyhow::Result<String> {
        let mut chat_req_messages: Vec<GroqMessage> = vec![];

        for msg in thread_messages {
            let comp_req = GroqMessage {
                content: msg.message,
                role: msg.role,
            };

            chat_req_messages.push(comp_req);
        }

        let req_body = GroqRequest {
            messages: chat_req_messages,
            model: self.completion_model.as_str().to_string(),
        };

        let client = reqwest::Client::new();
        let body = serde_json::to_string(&req_body)?;

        let response: GroqResponse = client
            .post("https://api.groq.com/openai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?
            .json()
            .await?;

        match response.choices.first() {
            Some(choice) => Ok(choice.message.content.to_owned()),
            None => Ok("No response".to_string()),
        }
    }
}
