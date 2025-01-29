use sea_orm::{Database, DatabaseConnection};
use migration::MigratorTrait;

use crate::config::database::get_database_options;

pub async fn get_db_connection() -> DatabaseConnection {
    let db = Database::connect(
        get_database_options()
    )
    .await
    .expect("Failed to connect on database");

    migration::Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");

    db
}