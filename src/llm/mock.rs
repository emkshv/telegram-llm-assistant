use crate::llm::LLMService;
use crate::llm::LLMThreadMessage;
use async_trait::async_trait;
use std::error::Error;

pub struct Mock;

#[async_trait]
impl LLMService for Mock {
    async fn get_answer(
        &self,
        thread_messages: Vec<LLMThreadMessage>,
    ) -> Result<String, Box<dyn Error>> {
        println!("Mock request:");

        for msg in thread_messages {
            println!("Message: {:?}", msg)
        }

        async move {
            tokio::time::sleep(tokio::time::Duration::from_micros(1)).await;
            Ok("Ok".to_string())
        }
        .await
    }
}
