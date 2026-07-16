use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    /// Run in interactive wizard mode
    Wizard,
}

pub fn run() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Wizard) => {
            println!("🚀 Starting Archiver Wizard...");
            // TODO: Implement interactive flow
        }
        None => {
            println!("Use --help for usage");
        }
    }
}
