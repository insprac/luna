use std::path::PathBuf;

use crate::result::Result;
use clap::Parser;
use vulcan::chat::{providers::GPTChat, ChatMessage, ChatProvider};

const BASE_PROMPT: &str = r#"
You are an intelligent and creative assistant.
You are working in a local directory.
You'll be provided with all files available in the directory.
Then you will answer the user's questions about the files.
Answer concisely and informatively.
"#;

#[derive(Parser)]
pub struct Ask {
    pub prompt: String,
}

impl Ask {
    pub async fn run(&self) -> Result<()> {
        let model = "gpt-4-turbo-preview".to_string();
        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
        let gpt = GPTChat::new(model, api_key, 0.1);
        let messages = self.build_messages()?;
        let response = gpt.chat(&messages, vec![]).await?;
        println!("{}", response.content);
        Ok(())
    }

    fn build_messages(&self) -> Result<Vec<ChatMessage>> {
        let mut messages = vec![ChatMessage::system(BASE_PROMPT.to_string())];
        let current_dir = std::env::current_dir().unwrap();
        messages.extend(self.build_file_messages(current_dir)?);
        messages.push(ChatMessage::user(self.prompt.clone()));
        Ok(messages)
    }

    fn build_file_messages(&self, dir: PathBuf) -> Result<Vec<ChatMessage>> {
        let mut messages: Vec<ChatMessage> = vec![];
        for entry in ignore::Walk::new(dir) {
            let file = entry?.path().to_path_buf();
            if file.is_dir() || self.is_file_ignored(&file) {
                continue;
            }
            match self.read_file(file) {
                Ok(message) => {
                    messages.push(message);
                }
                Err(e) => {
                    println!("Error reading file: {:?}", e);
                }
            }
        }
        Ok(messages)
    }

    fn is_file_ignored(&self, file: &PathBuf) -> bool {
        let ignore_files = vec!["Cargo.lock", "package-lock.json", "yarn.lock", ".DS_Store"];
        if let Some(file_name) = file.file_name() {
            if let Some(file_name) = file_name.to_str() {
                if ignore_files.contains(&file_name) {
                    return true;
                }
            }
        }
        false
    }

    fn read_file(&self, file: PathBuf) -> Result<ChatMessage> {
        let content = std::fs::read_to_string(&file)?;
        let content = format!("File: {}\n\n```{}```", file.to_str().unwrap(), content);
        Ok(ChatMessage::system(content))
    }
}
