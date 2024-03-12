use clap::{Parser, Subcommand};

mod result;
mod commands;
mod tools;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Ask(commands::Ask),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Ask(ask_cmd)) => {
            ask_cmd.run().await?;
        }
        None => {
            panic!("No subcommand was used, use --help to see available subcommands");
        }
    }

    Ok(())
}
