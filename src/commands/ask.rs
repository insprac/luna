use std::path::PathBuf;

use crate::{io::write_message, result::Result, tools::ask_user::AskUser};
use clap::Parser;
use vulcan::{chat::{providers::GPTChat, ChatMessage, ChatProvider}, tools::{Tool, ToolCall}};

const BASE_PROMPT: &str = r#"
You are an intelligent and creative assistant.
You are working in a local directory.
You'll be provided with all files available in the directory.
Then you will answer the user's questions about the files.
Answer concisely and informatively.
"#;

#[derive(Parser)]
pub struct Ask {
    // pub prompt: String,
    #[clap(skip)]
    messages: Vec<ChatMessage>,
}

impl Ask {
    pub async fn start(&mut self) -> Result<()> {
        self.messages = self.build_messages().unwrap();

        loop {
            let input = crate::io::read_user_input();
            if input == "exit" {
                break;
            }
            self.messages.push(ChatMessage::user(input));
            self.run().await?;
        }

        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        let model = "gpt-4-turbo-preview".to_string();
        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
        let gpt = GPTChat::new(model, api_key, 0.1);
        let tools = self.build_tools();
        let response = gpt.chat(&self.messages, tools).await?;
        self.handle_response(response).await?;
        Ok(())
    }

    fn build_messages(&self) -> Result<Vec<ChatMessage>> {
        let mut messages = vec![ChatMessage::system(BASE_PROMPT.to_string())];
        let current_dir = std::env::current_dir().unwrap();
        messages.extend(self.build_file_messages(current_dir)?);
        // messages.push(ChatMessage::user(self.prompt.clone()));
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

    fn build_tools(&self) -> Vec<Tool> {
        vec![
            AskUser::tool(),
        ]
    }

    async fn handle_response(&mut self, message: ChatMessage) -> Result<()> {
        write_message(&message);
        self.messages.push(message.clone());
        match message.tool_calls {
            Some(tool_calls) => {
                for tool_call in tool_calls {
                    self.handle_tool_call(tool_call).await?;
                }
            }
            None => ()
        }
        Ok(())
    }

    async fn handle_tool_call(&mut self, tool_call: ToolCall) -> Result<()> {
        println!("{}({:?})", tool_call.name, tool_call.args);
        match tool_call.name.as_str() {
            "ask_user" => {
                let ask_user = AskUser::from_json(tool_call.args)?;
                let user_response = ask_user.run()?;
                self.messages.extend(user_response);
                Ok(())
            }
            _ => {
                panic!("Unknown tool: {}", tool_call.name);
            }
        }
    }
}
