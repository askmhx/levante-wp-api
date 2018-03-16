use std::fs::File;
use std::io::Read;
use std::fmt;
use std::net::IpAddr;

use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct DbConfig {
    pub host: IpAddr,
    pub port: u16,
    pub user: String,
    pub password: Option<String>,
    pub database: String,
}

impl fmt::Display for DbConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let connect = format!("mysql://{}:{}@{}:{}/{}", self.user, self.password, self.host, self.port, self.database);
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
    pub db: DbConfig,
    pub log: LogConfig,
}

impl Config {
    pub fn from_file(filepath: &str) -> Option<Config> {
        let mut file = File::open(filepath)?;
        let mut raw = String::new();
        file.read_to_string(&mut raw)?;
        let value = serde_json::from_str(&raw).unwrap()?;
        Ok(value)
    }
}