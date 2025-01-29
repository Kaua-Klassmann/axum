use sea_orm::DatabaseConnection;

use crate::database::database::get_db_connection;

pub struct AppState {
    pub db_conn: DatabaseConnection
}

pub async fn get_state() -> AppState {
    let db_conn = get_db_connection().await;

    AppState {
        db_conn
    }
}