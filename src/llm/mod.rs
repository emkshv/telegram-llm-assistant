pub mod llm_thread_message;
pub mod mock;
pub mod openai;
use async_trait::async_trait;

use llm_thread_message::LLMThreadMessage;

#[derive(Clone)]
pub enum LLMProvider {
    OpenAI,
    Mock,
}

impl Default for LLMProvider {
    fn default() -> Self {
        LLMProvider::OpenAI
    }
}

#[async_trait]
pub trait LLMService: Send {
    async fn get_answer(&self, thread_messages: Vec<LLMThreadMessage>) -> anyhow::Result<String>;
}
