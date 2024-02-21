use crate::constants::{DEFAULT_CONFIG_FILENAME, DEFAULT_HOST, DEFAULT_PORT, HOST, PORT};
use ini::Ini;

/// A simple configuration struct.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    host: String,
    port: u16,
}

impl Config {
    pub fn host(&self) -> String {
        self.host.clone()
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn load() -> Self {
        let cfg = match Ini::load_from_file(DEFAULT_CONFIG_FILENAME) {
            Ok(config) => config,
            Err(_) => {
                // If the file doesn't exist, create it with default values.
                let mut config = Ini::new();
                let default = Config::default();

                config
                    .with_general_section()
                    .set(HOST, default.host())
                    .set(PORT, default.port().to_string());

                config.write_to_file(DEFAULT_CONFIG_FILENAME).unwrap();

                config
            }
        };

        let host = cfg.general_section().get(HOST).unwrap().to_string();
        let port = cfg.general_section().get(PORT).unwrap().parse().unwrap();

        Config { host, port }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: DEFAULT_HOST.to_owned(),
            port: DEFAULT_PORT,
        }
    }
}
