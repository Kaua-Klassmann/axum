use std::{env, sync::OnceLock};

#[derive(Clone)]
pub struct RedisConfig {
    pub url: String,
    pub max_connections: u32,
    pub ttl: u64,
}

static REDIS_CONFIG: OnceLock<RedisConfig> = OnceLock::new();

pub fn get_redis_config() -> RedisConfig {
    REDIS_CONFIG
        .get_or_init(|| {
            let url = env::var("REDIS_URL").expect("REDIS_URL not found at .env file");

            let max_connections = env::var("REDIS_MAX_CONNECTIONS")
                .ok()
                .and_then(|v| v.parse().ok())
                .expect("REDIS_MAX_CONNECTIONS not found at .env file");

            let ttl = env::var("REDIS_TTL")
                .expect("REDIS_TTL not found at .env file")
                .parse()
                .expect("REDIS_TTL not found at .env file");

            RedisConfig {
                url,
                max_connections,
                ttl,
            }
        })
        .clone()
}
