mod gps;
mod run;
mod speed;
mod step;

use std::f64::consts::PI;
use md5;
use rand::{self, Rng};
use json::JsonValue;

use crate::utils::*;
use crate::config::*;

pub use self::gps::GPSRecord;
pub use self::run::RunRecord;
pub use self::speed::SpeedRecord;
pub use self::step::StepRecord;

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
        self.custom_id = format!("{:X}", md5::compute(rand::thread_rng().gen::<[u8; 16]>()));
    }
}

#[derive(Default, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub campus_name: String,
    pub uid: u32,
    pub unid: u32,
    pub token: String,
}

#[derive(Debug, Clone)]
pub struct FivePoint {
    pub id: u32,
    pub pos: GeoPoint,
    pub name: String,
    pub fixed: u32,
}

impl FivePoint {
    pub fn to_json(&self, flag: u64) -> JsonValue {
        object! {
            "id" => self.id,
            "flag" => flag,
            "hasReward" => false,
            "isFixed" => self.fixed,
            "isPass" => true,
            "lon" => self.pos.lon,
            "lat" => self.pos.lat,
            "pointName" => self.name.clone(),
            "position" => 999,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn distance_to(&self, v: Vector) -> f64 {
        ((v.x - self.x).powf(2.0) + (v.y - self.y).powf(2.0)).sqrt()
    }

    pub fn step_toward(&self, v: Vector, distance: f64) -> Vector {
        let delta = Vector {
            x: v.x - self.x,
            y: v.y - self.y,
        };
        let delta_distance = delta.distance_to(Vector { x: 0.0, y: 0.0 });
        let factor = (distance / delta_distance).min(1.0);

        Vector {
            x: self.x + delta.x * factor,
            y: self.y + delta.y * factor,
        }
    }

    pub fn fuzzle(&self) -> Vector {
        Vector {
            x: rand_near_f64(self.x, FUZZLE_ERR),
            y: rand_near_f64(self.y, FUZZLE_ERR),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GeoPoint {
    pub lon: f64,
    pub lat: f64,
}

impl GeoPoint {
    pub fn get_offset_of(&self, origin: Self) -> Vector {
        let dx = self.lon - origin.lon;
        let dy = self.lat - origin.lat;
        let lat_middle = (self.lat + origin.lat) / 2.0;
        let x = (dx * PI / 180.0) * 6367000.0 * (lat_middle * PI / 180.0).cos();
        let y = 6367000.0 * dy * PI / 180.0;
        Vector { x, y }
    }

    pub fn offset(&self, vector: Vector) -> Self {
        let dlat = vector.y * 180.0 / PI / 6367000.0;
        let lat_middle = (self.lat * 2.0 + dlat) / 2.0;
        let dlon = vector.x * 180.0 / PI / (lat_middle * PI / 180.0).cos() / 6367000.0;
        GeoPoint {
            lon: self.lon + dlon,
            lat: self.lat + dlat,
        }
    }
}

#[derive(Debug)]
pub struct Captcha {
    pub challenge: String,
    pub gt: String,
}

#[derive(Debug)]
pub struct CaptchaResult {
    pub challenge: String,
    pub validate: String,
}

#[derive(Debug)]
pub struct RoutePlan {
    pub route_points: Vec<GeoPoint>,
    pub min_distance: u32,
    pub min_points: u32,
}
