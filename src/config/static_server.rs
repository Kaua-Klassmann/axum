use std::{env, sync::OnceLock};

#[derive(Clone)]
pub struct StaticServer {
    pub url: String,
}

static STATIC_SERVER: OnceLock<StaticServer> = OnceLock::new();

pub fn get_static_server() -> StaticServer {
    STATIC_SERVER
        .get_or_init(|| {
            let url = env::var("STATIC_SERVER").expect("STATIC_SERVER not found at .env file");

            StaticServer { url }
        })
        .clone()
}
