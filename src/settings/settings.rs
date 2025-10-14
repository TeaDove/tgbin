use serde::Deserialize;
use config::{Config, Environment, File};

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct Settings {
    pub url: String,
    pub db_path: String,
    pub tg_token: String,
}

impl Settings {
    pub fn must_new() -> Self {
        let s = Config::builder()
            .add_source(File::with_name(".env.yaml"))
            .add_source(Environment::with_prefix("APP_")).
            build().unwrap();

        s.try_deserialize().unwrap()
    }
}

