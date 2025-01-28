use std::env;

use migration::MigratorTrait;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::sync::OnceCell;

static DB_CONN: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn get_db() -> &'static DatabaseConnection {
    DB_CONN.get_or_init(configure_database).await
}

async fn configure_database() -> DatabaseConnection {
    let db_url = env::var("DB_URL").expect("DB_URL not found at .env file");
    let db_max_connections = env::var("DB_MAX_CONNECTIONS")
        .expect("DB_MAX_CONNECTIONS not found at .env file")
        .parse::<u32>()
        .unwrap();
    let db_min_connections = env::var("DB_MIN_CONNECTIONS")
        .expect("DB_MIN_CONNECTIONS not found at .env file")
        .parse::<u32>()
        .unwrap();
    let db_schema = env::var("DB_SCHEMA").expect("DB_SCHEMA not found at .env file");

    let db_opt = ConnectOptions::new(db_url)
        .max_connections(db_max_connections)
        .min_connections(db_min_connections)
        .set_schema_search_path(db_schema)
        .to_owned();

    let db = Database::connect(db_opt)
        .await
        .expect("Failed to connect on database");

    migration::Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");

    db
}