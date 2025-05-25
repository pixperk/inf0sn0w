use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "inf0sn0w", about = "The villain-tier OSINT engine.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run OSINT by username
    Username {
        #[arg(short, long)]
        username: String,
    },
    /// Run OSINT by email
    Email {
        #[arg(short, long)]
        email: String,
    },
    /// Run both email + username
    Full {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        email: String,
    },
}
