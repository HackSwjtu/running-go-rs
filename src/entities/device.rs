use rand::{self, Rng};
use md5;

#[derive(Default, Debug)]
pub struct Device {
    pub imei: String,
    pub model: String,
    pub mac: String,
    pub os_version: String,
    pub user_agent: String,
    pub id: String,
    pub custom_id: String,
}

impl Device {
    pub fn build(&mut self) {
        self.id = format!(
            "{:x}",
            md5::compute(String::new() + &self.imei + &self.model + &self.mac)
        );
        self.custom_id = format!("{:X}", md5::compute(&self.id));
    }
}
