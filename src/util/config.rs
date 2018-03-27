use std::fs::File;
use std::fmt;
use std::net::IpAddr;
use std::error::Error;

use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct DbConfig {
    pub host: IpAddr,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub schema: String,
}

impl fmt::Display for DbConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let connect = format!("mysql://{}:{}@{}:{}/{}", self.user, self.password, self.host, self.port, self.schema);
        write!(f, "{}", connect)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogConfig {
    pub file: String,
    pub rotate_count: u16,
    pub rotate_type: String,
    pub rotate_value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerConfig {
    pub port: u16,
    pub addr: String,
    pub charset: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub home: String,
    pub server: ServerConfig,
    pub database: DbConfig,
    pub log: LogConfig,
}

impl Config {
    pub fn from_file(filepath: String) -> Result<Config, Box<Error>> {
        let file = File::open(filepath)?;
        Ok(serde_json::from_reader(file)?)
    }
}