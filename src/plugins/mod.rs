use async_trait::async_trait;

mod github;
mod breach;
mod reddit;

pub use github::*;
pub use breach::*;
pub use reddit::*;


#[async_trait]
pub trait OsintPlugin: Send + Sync {
    async fn run_username_scan(&self, username: &str) -> Option<String>;
    async fn run_email_scan(&self, email: &str) -> Option<String>;
    
    fn name(&self) -> &'static str;
}
