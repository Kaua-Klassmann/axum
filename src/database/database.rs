use migration::MigratorTrait;
use sea_orm::{Database, DatabaseConnection};
use tokio::sync::OnceCell;

use crate::config::database::get_database_options;

static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn get_db_connection() ->  DatabaseConnection {
    DB.get_or_init(|| async {
        let db = Database::connect(
            get_database_options()
        ).await
        .expect("Failed to connect on database");

        migration::Migrator::up(&db, None)
            .await
            .expect("Failed to run migrations");
        
        db
    })
    .await
    .clone()
}
