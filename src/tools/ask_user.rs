use serde::Deserialize;
use serde_json::{json, Value};
use vulcan::chat::ChatMessage;
use vulcan::tools::Tool;
use std::io::Write;

use crate::result::Result;

#[derive(Deserialize, Debug, Clone)]
pub struct AskUser {
    pub question: String,
}

impl AskUser {
    pub fn from_json(args: Value) -> Result<Self> {
        let tool: Self = serde_json::from_value(args)?;
        Ok(tool)
    }

    pub fn tool() -> Tool {
        Tool {
            name: "ask_user".to_string(),
            description: "Ask the user a question when you need more information to fulfill their request".to_string(),
            params: json!({
                "type": "object",
                "properties": {
                    "question": {
                        "type": "string",
                        "description": "The question to ask the user"
                    }
                },
                "required": ["question"]
            }),
        }
    }

    pub fn run(&self) -> Result<Vec<ChatMessage>> {
        println!("\n[assistant]\n{}", self.question);
        print!("\n[user]\n> ");
        std::io::stdout().flush()?;
        let mut response = String::new();
        std::io::stdin().read_line(&mut response)?;
        let message = ChatMessage::user(response.trim().to_string());
        Ok(vec![message])
    }
}
