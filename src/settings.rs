use std::env;

use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Database {
    url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Grpc {
    pub scheme: String,
    pub server: String,
    pub port: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Http {
    pub scheme: String,
    pub server: String,
    pub port: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub(crate) struct Settings {
    debug: bool,
    database: Database,
    pub grpc: Grpc,
    pub http: Http,
}

#[allow(dead_code)]
impl Settings {
    pub(crate) fn new() -> Result<Self, ConfigError> {
        let conf_dir = env::var("CONF_DIR").unwrap_or_else(|_| "dev".into());
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());
        let grpc_server = env::var("GRPC_SERVER").unwrap_or_else(|_| "dev".into());

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name(&format!("{}/default", conf_dir.as_str())))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(
                File::with_name(&format!("{}/{}", conf_dir.as_str(), run_mode)).required(false),
            )
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name(&format!("{}/local", conf_dir.as_str())).required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("app"))
            .set_override("grpc.server", grpc_server)?
            // You may also programmatically change settings
            //.set_override("database.url", "postgres://")?
            .build()?;

        // Now that we're done, let's access our configuration
        println!("debug: {:?}", s.get_bool("debug"));

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}
