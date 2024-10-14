use async_trait::async_trait;

#[async_trait]
// marker trait Send + Sync
pub trait HealthCheckRepository: Send + Sync {
    async fn check_db(&self) -> bool;
}
