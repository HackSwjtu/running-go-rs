use std::path::Path;
use std::fs::{self, File};
use std::io::prelude::*;
use serde_ini;
use rand::{self, Rng};
use itertools::Itertools;

use crate::error::Error;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub start_pos_lat: f64,
    pub start_pos_lon: f64,
    pub device_imei: String,
    pub device_model: String,
    pub device_mac: String,
    pub device_os_version: String,
    pub device_user_agent: String,
}

impl Config {
    pub fn from_path(path: &str) -> Result<Self, Error> {
        let mut file = File::open(Path::new(path))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(serde_ini::from_str(&contents)?)
    }

    pub fn output(&self, path: &str) -> Result<(), Error> {
        let path = Path::new(path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = File::create(Path::new(path))?;
        serde_ini::to_writer(&mut file, self)?;
        Ok(())
    }

    pub fn build(&mut self) {
        let mut rng = rand::thread_rng();

        let models = [
            "Xiaomi MI 2",
            "Xiaomi MI 2S",
            "Xiaomi MI 2A",
            "Xiaomi MI 3",
            "Xiaomi MI 4",
            "Xiaomi MI 5",
            "Xiaomi MI 6",
            "Meizu MX4",
            "Meizu MX5",
            "Meizu MX6",
            "Meizu Pro5",
            "Meizu Pro6",
            "Meizu Pro6 Plus",
            "Meizu Pro7",
            "ONEPLUS A3000",
            "ONEPLUS A3010",
            "ONEPLUS A5000",
            "ONEPLUS A5010",
        ];
        let os_versions = [
            "6.0.0", "6.0.1", "6.0.2", "6.1.0", "6.1.1", "7.0.0", "7.0.1", "7.1.0", "7.1.1",
            "7.1.1", "7.1.2",
        ];

        let mut imei = rng.next_u64().to_string();
        imei.truncate(15);
        self.device_imei = imei;

        self.device_mac = (0..6)
            .map(|_| format!("{:02x}", rng.next_u32() % 256))
            .join(":");

        self.device_os_version =
            os_versions[rng.next_u32() as usize % os_versions.len()].to_string();

        self.device_model = models[rng.next_u32() as usize % models.len()].to_string();

        self.device_user_agent = format!(
            "Dalvik/2.1.0 (Linux; Android {}; {})",
            self.device_os_version, self.device_model
        );
    }
}
