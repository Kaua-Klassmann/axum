use std::sync::Arc;

use argon2::Argon2;
use reqwest::Client as ReqwestClient;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db_conn: DatabaseConnection,
    pub argon2: Arc<Argon2<'static>>,
    pub reqwest_client: Arc<ReqwestClient>,
}
