use std::{env, fmt::Display, str::FromStr};

pub fn get_env_var<T: Display + FromStr>(key: &str, default: T) -> T {
    match env::var(key) {
        Ok(key) => match key.parse::<T>().ok() {
            Some(key) => key,
            None => {
                log::error!(
                    "Could not cast environment variable '{key}'! Defaulting to '{default}'...",
                );

                default
            },
        },
        Err(error) => {
            log::debug!(
                "Could not get environment variable \
                '{key}', defaulting to '{default}'. Error: {error}"
            );

            default
        },
    }
}