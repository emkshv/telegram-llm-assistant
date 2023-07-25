use crate::llm::LLMService;
use crate::llm::LLMServiceModel;
use crate::llm::LLMThreadMessage;
use async_trait::async_trait;
use std::str::FromStr;

pub struct Mock {
    pub completion_model: MockCompletionModel,
}

#[derive(Copy, Clone, Debug)]
pub enum MockCompletionModel {
    Bright,
    Brighter,
}

impl Default for MockCompletionModel {
    fn default() -> Self {
        MockCompletionModel::Bright
    }
}

pub fn all_completions() -> [MockCompletionModel; 2] {
    [MockCompletionModel::Bright, MockCompletionModel::Brighter]
}

impl LLMServiceModel for MockCompletionModel {}

impl MockCompletionModel {
    pub fn as_str(&self) -> &'static str {
        match *self {
            MockCompletionModel::Bright => "bright",
            MockCompletionModel::Brighter => "brighter",
        }
    }

    pub fn default_string() -> String {
        let mock_completion_model: MockCompletionModel = Default::default();
        mock_completion_model.as_str().to_string()
    }
}

impl FromStr for MockCompletionModel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bright" => Ok(MockCompletionModel::Bright),
            "brighter" => Ok(MockCompletionModel::Brighter),
            _ => Err(()),
        }
    }
}

#[async_trait]
impl LLMService for Mock {
    fn bot_info(&self) -> String {
        format!("Bot uses Mock with {}", self.completion_model.as_str())
    }

    async fn get_answer(&self, thread_messages: Vec<LLMThreadMessage>) -> anyhow::Result<String> {
        println!(
            "Mocked request using {:?} completion model",
            self.completion_model
        );

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
