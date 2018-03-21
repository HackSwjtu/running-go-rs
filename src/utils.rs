use std::collections::BTreeMap;
use rand::{self, Rng};
use itertools::Itertools;
use md5;

pub fn rand_near(base: u32, err: u32) -> u32 {
    (base as f64 + (err as f64 * (rand::thread_rng().next_f64() * 2.0 - 1.0))) as u32
}

pub fn rand_near_f64(base: f64, err: f64) -> f64 {
    base + err * (rand::thread_rng().next_f64() * 2.0 - 1.0)
}

pub fn compute_sign(map: &BTreeMap<String, String>, salt: &str) -> String {
    let str = map.iter().map(|(k, v)| format!("{}={}", k, v)).join("&");
    format!("{:x}", md5::compute(str.clone() + salt))
}
