use super::OsintPlugin;
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Deserialize)]
struct GitHubProfile {
    login: String,
    name: Option<String>,
    bio: Option<String>,
    public_repos: u32,
    followers: u32,
    created_at: String,
}

pub struct GitHubPlugin;

#[async_trait]
impl OsintPlugin for GitHubPlugin {
    async fn run_username_scan(&self, username: &str) -> Option<String> {
        let url = format!("https://api.github.com/users/{}", username);
        let client = reqwest::Client::new();

        let res = client
            .get(&url)
            .header("User-Agent", "inf0sn0w")
            .send()
            .await;

        match res
        {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<GitHubProfile>().await {
                        Ok(profile) => {
                            Some(format!(
                                "Username: {}\nName: {}\nBio: {}\nPublic Repos: {}\nFollowers: {}\nCreated At: {}",
                                profile.login,
                                profile.name.unwrap_or_else(|| "N/A".to_string()),
                                profile.bio.unwrap_or_else(|| "N/A".to_string()),
                                profile.public_repos,
                                profile.followers,
                                profile.created_at
                            ))
                        }
                        Err(_) => None, // JSON parsing error
                    }
                } else {
                    None // User not found or other error
                }
            }
            Err(_) => None, // Request failed
        }
    }

    async fn run_email_scan(&self, _email: &str) -> Option<String> {
        None // GitHub plugin doesn't do email lookups
    }

    fn name(&self) -> &'static str {
        "GitHub"
    }
}
