use super::OsintPlugin;
use async_trait::async_trait;
use serde::Deserialize;

pub struct BreachPlugin;

#[derive(Debug, Deserialize)]
struct LeakSource {
    name: String,
    date: String,
}

#[derive(Debug, Deserialize)]
struct LeakCheckResponse {
    success: bool,
    found: usize,
    sources: Vec<LeakSource>,
}


#[async_trait]
impl OsintPlugin for BreachPlugin {
    async fn run_username_scan(&self, _username: &str) -> Option<String> {
        None // Breach plugin doesn’t use username
    }

    async fn run_email_scan(&self, email: &str) -> Option<String> {
         let url = format!("https://leakcheck.io/api/public?check={}", email);
        let client = reqwest::Client::new();

        match client.get(&url).send().await {
            Ok(response) if response.status().is_success() => {
                let data = response.json::<LeakCheckResponse>().await.ok()?;
                if data.success && data.found > 0 {
                    let mut report = format!("{} leaks found for {}:\n", data.found, email);
                    for src in data.sources {
                        report.push_str(&format!(
                            "- {} (leaked on {})\n",
                            src.name, src.date
                        ));
                    }
                    Some(report)
                } else {
                    Some(format!("No leaks found for {}", email))
                }
            }
            Ok(response) => Some(format!("API error: HTTP {}", response.status())),
            Err(e) => Some(format!("Request failed: {}", e)),
    }
}

    fn name(&self) -> &'static str {
        "BreachChecker"
    }
}
