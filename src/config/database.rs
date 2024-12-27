use dotenvy::var;
use once_cell::sync::Lazy;
use sea_orm::ConnectOptions;

pub static DB_OPTIONS: Lazy<ConnectOptions> = Lazy::new(set_db_options);

fn set_db_options() -> ConnectOptions {
    let url: String = var("DATABASE_URL").expect("DATABASE_URL not found");
    let max_connections: String = var("DATABASE_MAX_CONNECTIONS").expect("DATABASE_MAX_CONNECTIONS not found");
    let min_connections: String = var("DATABASE_MIN_CONNECTIONS").expect("DATABASE_MIN_CONNECTIONS not found");
    let schema: String = var("DATABASE_SCHEMA").expect("DATABASE_SCHEMA not found");

    ConnectOptions::new(url)
        .max_connections(max_connections.parse().unwrap())
        .min_connections(min_connections.parse().unwrap())
        .set_schema_search_path(schema)
        .to_owned()
}