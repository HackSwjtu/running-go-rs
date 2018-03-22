use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use serde_ini;

use crate::error::Error;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub start_pos_lat: f64,
    pub start_pos_lon: f64,
    pub min_distance_meter: u64,
    pub device_imei: String,
    pub device_model: String,
    pub device_mac: String,
    pub device_os_version: String,
}

impl Config {
    pub fn from_path(path:&Path) -> Result<Self, Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(serde_ini::from_str(&contents)?)
    }

    pub fn save(&self,path:&Path) -> Result<(), Error> {
        let mut file = File::create(path)?;
        serde_ini::to_writer(&mut file, self)?;
        Ok(())
    }

    pub fn rand_device_info(&mut self) {
        
    }
}
