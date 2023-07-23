pub mod llm_thread_message;
pub mod mock;
pub mod openai;
use async_trait::async_trait;
use clap::ValueEnum;

use llm_thread_message::LLMThreadMessage;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum LLMServiceKind {
    OpenAI,
    Mock,
}

impl Default for LLMServiceKind {
    fn default() -> Self {
        LLMServiceKind::OpenAI
    }
}

#[async_trait]
pub trait LLMServiceModel: Sync + Send {}

#[async_trait]
pub trait LLMService: Send {
    async fn get_answer(&self, thread_messages: Vec<LLMThreadMessage>) -> anyhow::Result<String>;
}
