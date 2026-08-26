use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub port: u16,
    pub api_url: String,
    pub api_version: String,
    pub access_token: String
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        let port: u16 = env::var("PORT")
            .ok()
            .and_then(|val| val.parse().ok())
            .unwrap_or(3000);

        Config {
            port: port,
            api_url: env::var("API_URL").expect("API_URL environment variable must be set"),
            api_version: env::var("API_VERSION").expect("API_VERSION environment variable must be set"),
            access_token: env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN environment variable must be set"),
        }
    }
}
