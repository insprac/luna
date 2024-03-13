use serde_json::json;
use vulcan::tools::Tool;

pub struct WriteFileArgs {
    pub file_path: String,
    pub content: String,
}

pub fn tool() -> Tool {
    Tool {
        name: "write_file".to_string(),
        description: "Write to a new or existing file when modifications are needed".to_string(),
        params: json!({
            "type": "object",
            "properties": {
                "file_path": {
                    "type": "string",
                    "description": "The path to the file to write to"
                },
                "content": {
                    "type": "string",
                    "description": "The content to write to the file"
                }
            },
            "required": ["file_path", "content"]
        }),
    }
}

pub fn run(args: WriteFileArgs) -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()?;
    if !args.file_path.starts_with(&current_dir.to_str().unwrap()) {
        return Err("The file path must be within the current directory".into());
    }
    std::fs::write(args.file_path, args.content)?;
    Ok(())
}
