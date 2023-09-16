use anyhow::{Error, Result};
use chatgpt::config::{ModelConfiguration, ModelConfigurationBuilder};
use chatgpt::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // 1. Concatentate the arguments to form sentences
    let args: Vec<String> = std::env::args().collect();
    let words = &args[1..];
    let sentences = words.join(" ");

    // 2. Ask ChatGPT to translate whether the sentences in English or Japanese
    let key = dotenvy::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let max_tokens = dotenvy::var("OPENAI_MAX_TOKENS").expect("OPENAI_MAX_TOKENS must be set");
    let max_tokens = match max_tokens.parse::<u32>() {
        Ok(value) => value,
        Err(_) => {
            println!("Failed to parse OPENAI_MAX_TOKENS.  Setting it to 1000");
            1000
        }
    };

    let config: ModelConfiguration = ModelConfigurationBuilder::default()
        .temperature(0.0)
        .max_tokens(max_tokens as u32)
        .build()
        .unwrap_or_default();
    let client = ChatGPT::new_with_config(key, config)?;

    let conversation = r#"
            You are an expert at translating Japanese or English languages.  Please translate the 
            following sentences into Japanese or English, depending on whether the original sentences 
            are in English or Japanese.  Please respond with the translation only and do not specify
            whether the translated sentence is Japanese or English.
            "#;
    let conversation_string = conversation.to_string();
    let mut conversation: Conversation = client.new_conversation_directed(&conversation_string);

    let translation = conversation.send_message(&sentences).await?;
    println!("{}", translation.message_choices[0].message.content);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_01_args_to_words() {
        // Enter: "--nocapture This is a test." as command your line arguments"
        let args: Vec<String> = std::env::args().collect();
        let words = &args[1..];
        let sentences = words.join(" ");
        assert_eq!(sentences, "This is a test.".to_string());
    }

    #[test]
    fn test_02_env_vars() {
        let max_tokens = dotenvy::var("OPENAI_MAX_TOKENS").expect("OPENAI_MAX_TOKENS must be set");
        match max_tokens.parse::<u32>() {
            Ok(_value) => assert!(true),
            Err(_) => assert!(false),
        };
    }

    #[tokio::test]
    async fn test_03_translate() {
        let result = async {
            use chatgpt::config::{ModelConfiguration, ModelConfigurationBuilder};
            use chatgpt::prelude::*;

            let key = dotenvy::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
            let max_tokens =
                dotenvy::var("OPENAI_MAX_TOKENS").expect("OPENAI_MAX_TOKENS must be set");
            let max_tokens = match max_tokens.parse::<u32>() {
                Ok(value) => value,
                Err(_) => {
                    println!("Failed to parse OPENAI_MAX_TOKENS.  Setting it to 1000");
                    1000
                }
            };
            let config: ModelConfiguration = ModelConfigurationBuilder::default()
                .max_tokens(max_tokens as u32)
                .temperature(0.0)
                .build()
                .unwrap_or_default();
            let client = ChatGPT::new_with_config(key, config)?;

            let conversation = r#"
            You're an expert at translating Japanese or English languages.  Please translate the 
            following sentences into Japanese or English, depending on whether the original sentences 
            are in English or Japanese.  Please respond with the translation only and do not specify
            whether the translated sentence is Japanese or English.
            "#;
            let conversation_string = conversation.to_string();
            let mut conversation: Conversation =
                client.new_conversation_directed(&conversation_string);

            let sentences = "I am Alex Kelley. This is my brief but spectacular moment".to_string();
            conversation.send_message(&sentences).await
        };

        match result.await {
            Ok(cr) => {
                let translation = &cr.message_choices[0].message.content;
                assert_eq!(
                    translation,
                    "私はアレックス・ケリーです。これは私の短いけれども素晴らしい瞬間です。"
                );
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                assert!(false);
            }
        }
    }
}
