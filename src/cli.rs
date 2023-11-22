use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(required = true)]
    pub csv_path: PathBuf,

    #[clap(subcommand)]
    pub command: Commands,

    #[clap(short, long)]
    pub output: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Commands {
    Chunks(ChunksCommand),
    Files(FilesCommand),
}

impl Commands {
    pub fn name(&self) -> String {
        match &self {
            Commands::Chunks(_) => "chunks".to_string(),
            Commands::Files(_) => "files".to_string(),
        }
    }
}

#[derive(Args)]
pub struct ChunksCommand {
    pub chunks: usize,
}

#[derive(Args)]
pub struct FilesCommand {
    pub files: usize,
}
