use std::sync::Arc;

use argon2::Argon2;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db_conn: DatabaseConnection,
    pub argon2: Arc<Argon2<'static>>
}