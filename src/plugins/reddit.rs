use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

use super::OsintPlugin;

pub struct RedditPlugin;

#[derive(Deserialize)]
struct RedditUserData {
    data: RedditUser,
}

#[derive(Deserialize)]
struct RedditUser {
    name: String,
    link_karma: i32,
    comment_karma: i32,
    created_utc: f64,
    is_gold: bool,
    icon_img: String,
}

#[async_trait]
impl OsintPlugin for RedditPlugin {
  fn name (&self) -> &'static str {
        "Reddit"
    }

    async fn run_username_scan(&self, username: &str) -> Option<String> {
        let url = format!("https://www.reddit.com/user/{}/about.json", username);
        let client = Client::new();

        let res = client
            .get(&url)
            .header("User-Agent", "Inf0Sn0w/1.0")
            .send()
            .await
            .ok()?;

        if !res.status().is_success() {
            return None;
        }

        let user: RedditUserData = res.json().await.ok()?;

        let created = chrono::DateTime::<chrono::Utc>::from_timestamp(user.data.created_utc as i64, 0)?
            .format("%Y-%m-%d");

        Some(format!(
            "👤 Reddit: {}\n✨ Link Karma: {}\n💬 Comment Karma: {}\n🎂 Created: {}\n💎 Premium: {}\n🖼 Avatar: {}\n",
            user.data.name,
            user.data.link_karma,
            user.data.comment_karma,
            created,
            if user.data.is_gold { "Yes" } else { "No" },
            user.data.icon_img
        ))
    }

    async fn run_email_scan(&self, _: &str) -> Option<String> {
        None
    }

}
