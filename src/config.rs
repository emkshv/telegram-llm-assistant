use clap::Parser;
use std::env;

use crate::llm::LLMServiceKind;

#[derive(Clone, Debug)]
pub struct Config {
    pub llm_service: LLMServiceKind,
    pub telegram_token: String,
}

fn assert_env_var(env_var_name: &str) -> String {
    env::var(env_var_name)
        .unwrap_or_else(|_| {
            eprintln!(
                "Error: {} environment variable is not set. Please, set {} and try again.",
                env_var_name, env_var_name
            );
            std::process::exit(1);
        })
        .into()
}

impl Default for Config {
    fn default() -> Config {
        Config {
            llm_service: LLMServiceKind::Mock,
            telegram_token: assert_env_var("TELEGRAM_TOKEN"),
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
    let mut cfg = Config::default();

    let openai_key = env::var("OPENAI_API_KEY");

    match cli.llm_service {
        Some(LLMServiceKind::OpenAI) => {
            assert_env_var("OPENAI_API_KEY");
            cfg.llm_service = LLMServiceKind::OpenAI
        }
        Some(LLMServiceKind::Mock) => cfg.llm_service = LLMServiceKind::Mock,
        None => match openai_key {
            Err(_) => cfg.llm_service = LLMServiceKind::Mock,
            Ok(_) => cfg.llm_service = LLMServiceKind::OpenAI,
        },
    }

    cfg
}
