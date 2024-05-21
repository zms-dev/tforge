use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod client;
mod server;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Client {
        #[arg(short, long)]
        config: PathBuf,
        #[arg(short, long)]
        torrent: PathBuf,
    },
    Server {
        #[arg(short, long)]
        config: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Client { config, torrent } => client::main(config, torrent).await,
        Commands::Server { config } => server::main(config).await,
    }
}
