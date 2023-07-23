use crate::llm::LLMService;
use crate::llm::LLMThreadMessage;
use async_trait::async_trait;

pub struct Mock;

#[async_trait]
impl LLMService for Mock {
    async fn get_answer(&self, thread_messages: Vec<LLMThreadMessage>) -> anyhow::Result<String> {
        println!("Mocked request:");

        for msg in thread_messages {
            println!("Message: {:?}", msg)
        }

        async move {
            tokio::time::sleep(tokio::time::Duration::from_micros(1)).await;
            Ok("Mocked answer".to_string())
        }
        .await
    }
}
