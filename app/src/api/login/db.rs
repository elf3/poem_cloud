use anyhow::Result;
use sea_orm::DatabaseConnection;

use super::structures::LoginRequest;
pub async fn login(db: &DatabaseConnection, req: LoginRequest) -> Result<String> {
    Ok("success".to_string())
}
