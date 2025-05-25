mod cli;
mod engine;
mod plugins;

use cli::{Cli, Commands};
use clap::Parser;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Username { username } => {
            println!("🔍 Looking up username: {username}");
            engine::run_username_scan(&username).await;
        }
        Commands::Email { email } => {
            println!("📧 Looking up email: {email}");
            engine::run_email_scan(&email).await;
        }
        Commands::Full { username, email } => {
            println!("🧠 Full scan: {username} | {email}");
            engine::run_full_scan(&username, &email).await;
        }
    }
}
