fn main() {
    // 1. Concatentate the arguments to form sentences
    // let args: Vec<String> = env::args().collect();
    // let sentences = &args[1..];
    // let concatenated = sentences.join(" ");

    // 2. Ask ChatGPT to determine whether the sentences in English or Japanese
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_01_args_to_words() {
        // Enter: "This is a test." as command your line arguments"
        let args: Vec<String> = std::env::args().collect();
        let words = &args[1..];
        let sentences = words.join(" ");
        assert_eq!(sentences, "This is a test.".to_string());
    }

    #[tokio::test]
    async fn test_02_detect_jpn() {
        let result = async {
            use chatgpt::prelude::*;

            let key = dotenvy::var("OPENAI_KEY").expect("OPEN_AI key must be set");
            let client = ChatGPT::new(key)?;

            let conversation = r#"
                You're an expert at detecting whether sentences are in the Japanese or English language.
                What is the language of the following sentence?  
                Respond by stating the just the language (English or Japanese)
            "#;
            let conversation_string = conversation.to_string();
            let mut conversation: Conversation =
                client.new_conversation_directed(&conversation_string);

            let sentences = "私はアレクスーケリーです。".to_string();
            conversation.send_message(&sentences).await
        };

        match result.await {
            Ok(cr) => {
                let language = &cr.message_choices[0].message.content;
                assert_eq!(language, "Japanese");
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                assert!(false);
            }
        }
    }

    #[tokio::test]
    async fn test_03_detect_eng() {
        let result = async {
            use chatgpt::prelude::*;

            let key = dotenvy::var("OPENAI_KEY").expect("OPEN_AI key must be set");
            let client = ChatGPT::new(key)?;

            let conversation = r#"
                You're an expert at detecting whether sentences are in the Japanese or English language.
                What is the language of the following sentence?  
                Respond by stating the just the language (English or Japanese)
            "#;
            let conversation_string = conversation.to_string();
            let mut conversation: Conversation =
                client.new_conversation_directed(&conversation_string);

            let sentences = "I am Alex Kelley".to_string();
            conversation.send_message(&sentences).await
        };

        match result.await {
            Ok(cr) => {
                let language = &cr.message_choices[0].message.content;
                assert_eq!(language, "English");
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                assert!(false);
            }
        }
    }
}
