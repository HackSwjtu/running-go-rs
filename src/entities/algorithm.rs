use std::f64::consts::PI;

use crate::utils::*;
use crate::config::*;

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
