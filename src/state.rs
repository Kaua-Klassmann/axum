use std::sync::Arc;

use argon2::Argon2;
use sea_orm::DatabaseConnection;

use crate::database::database::get_db_connection;

pub struct AppState {
    pub db_conn: DatabaseConnection,
    pub argon2: Arc<Argon2<'static>>
}

pub async fn get_state() -> AppState {
    let db_conn = get_db_connection().await;
    let argon2 = Arc::new(Argon2::default());

    AppState {
        db_conn,
        argon2
    }
}