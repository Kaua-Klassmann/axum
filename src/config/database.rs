use std::env;

use sea_orm::ConnectOptions;

pub fn get_database_options() -> ConnectOptions {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found at .env file");
    let db_schema = env::var("DATABASE_SCHEMA").expect("DATABASE_SCHEMA not found at .env file");
    let db_max_connections = env::var("DATABASE_MAX_CONNECTIONS")
        .expect("DATABASE_MAX_CONNECTIONS not found at .env file")
        .parse::<u32>()
        .unwrap();
    let db_min_connections = env::var("DATABASE_MIN_CONNECTIONS")
        .expect("DATABASE_MIN_CONNECTIONS not found at .env file")
        .parse::<u32>()
        .unwrap();

    ConnectOptions::new(db_url)
        .max_connections(db_max_connections)
        .min_connections(db_min_connections)
        .set_schema_search_path(db_schema)
        .to_owned()
}