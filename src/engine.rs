
use std::sync::Arc;

use crate::plugins::{BreachPlugin, GitHubPlugin, OsintPlugin, RedditPlugin};

pub async fn run_username_scan(username: &str) {
    let plugins: Vec<Arc<dyn OsintPlugin>> = vec![
        Arc::new(GitHubPlugin),
        Arc ::new(RedditPlugin)
    ];

    for plugin in plugins {
        let name = plugin.name();
        match plugin.run_username_scan(username).await {
            Some(data) => println!("✅ [{}]: {}", name, data),
            None => println!("❌ [{}]: No data found.", name),
        }
    }
}

pub async fn run_email_scan(email: &str) {
    let plugins: Vec<Arc<dyn OsintPlugin>> = vec![
        Arc::new(BreachPlugin),
        // Add more email-based plugins here
    ];

    for plugin in plugins {
        let name = plugin.name();
        match plugin.run_email_scan(email).await {
            Some(data) => println!("✅ [{}]: {}", name, data),
            None => println!("❌ [{}]: No data found.", name),
        }
    }
}

pub async fn run_full_scan(username: &str, email: &str) {
    run_username_scan(username).await;
    run_email_scan(email).await;
}
