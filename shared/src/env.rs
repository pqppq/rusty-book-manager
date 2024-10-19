use std::env;
use strum::EnumString;

#[derive(Default, EnumString)]
pub enum Environment {
    #[default]
    Development,
    Production,
}

pub fn which() -> Environment {
    // debug buildではこちらが実行される
    #[cfg(debug_assertions)]
    let default_env = Environment::Development;

    // release buildではこちらが実行される
    #[cfg(not(debug_assertions))]
    let default_env = Environment::Production;

    match env::var("ENV") {
        Err(_) => default_env,
        Ok(v) => v.parse().unwrap_or(default_env)
    }
}
