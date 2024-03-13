use vulcan::chat::ChatMessage;
use std::io::Write;

pub fn read_user_input() -> String {
    print!("\n[user]\n> ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn write_message(message: &ChatMessage) {
    println!("\n[{}]\n{}", message.role, message.content);
    if let Some(tool_calls) = &message.tool_calls {
        for tool_call in tool_calls {
            println!("{}({:?})", tool_call.name, tool_call.args);
        }
    }
}
