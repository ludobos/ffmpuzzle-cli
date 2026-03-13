use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ffmpuzzle", about = "The daily FFmpeg encoding puzzle")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Force offline mode (embedded puzzles)
    #[arg(long)]
    pub offline: bool,

    /// API base URL override
    #[arg(long, default_value = "https://ffmpuzzle.com")]
    pub api_url: String,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Submit a guess via flags
    Guess {
        #[arg(long)]
        codec: Option<String>,
        #[arg(long)]
        crf: Option<String>,
        #[arg(long)]
        bitrate: Option<String>,
        #[arg(long, alias = "res")]
        resolution: Option<String>,
        #[arg(long)]
        audio: Option<String>,
        #[arg(long)]
        container: Option<String>,
        #[arg(long)]
        preset: Option<String>,
    },
    /// Show current game status
    Status,
    /// Show/copy share text
    Share {
        /// Copy to clipboard
        #[arg(long)]
        copy: bool,
    },
    /// Show personal stats
    Stats,
    /// Show today's leaderboard
    Leaderboard,
}
