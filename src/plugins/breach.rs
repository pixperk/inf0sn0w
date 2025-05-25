use super::OsintPlugin;
use async_trait::async_trait;

pub struct BreachPlugin;

#[async_trait]
impl OsintPlugin for BreachPlugin {
    async fn run_username_scan(&self, _username: &str) -> Option<String> {
        None // Breach plugin doesn’t use username
    }

    async fn run_email_scan(&self, email: &str) -> Option<String> {
        Some(format!("Email {} found in breached DBs", email))
    }

    fn name(&self) -> &'static str {
        "BreachChecker"
    }
}
