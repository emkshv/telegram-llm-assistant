use clap::Parser;
use std::env;

use crate::llm::LLMServiceKind;

#[derive(Clone, Debug)]
pub struct Config {
    pub llm_service: LLMServiceKind,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            llm_service: LLMServiceKind::Mock,
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    #[arg(value_enum)]
    llm_service: Option<LLMServiceKind>,
}

pub fn create_config() -> Config {
    let cli = Cli::parse();
    let mut cfg = Config {
        llm_service: LLMServiceKind::Mock,
    };

    let openai_key = env::var("OPENAI_API_KEY");

    match cli.llm_service {
        Some(LLMServiceKind::OpenAI) => match openai_key {
            Err(e) => {
                panic!("Error getting OPENAI_API_KEY {:?}", e);
            }
            Ok(_) => cfg.llm_service = LLMServiceKind::OpenAI,
        },
        Some(LLMServiceKind::Mock) => cfg.llm_service = LLMServiceKind::Mock,
        None => match openai_key {
            Err(_) => cfg.llm_service = LLMServiceKind::Mock,
            Ok(_) => cfg.llm_service = LLMServiceKind::OpenAI,
        },
    }

    cfg
}
