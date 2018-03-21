use json::JsonValue;

use crate::config::*;
use crate::utils::*;

#[derive(Debug)]
pub struct SpeedRecord {
    pub id: u32,
    pub begin: u64,
    pub end: u64,
    pub distance: f64,
}

impl SpeedRecord {
    pub fn rand(start_time: u64, end_time: u64) -> Vec<Self> {
        let mut records = Vec::new();
        let mut curr_id = 0;
        let mut curr_time = start_time;

        while curr_time < end_time {
            let prev_time = curr_time;
            let duration = rand_near_f64(10.0 * 1000.0, SPAMLE_TIME_ERR * 1000.0) as u64;
            curr_time += duration;
            let speed = rand_near_f64(AVG_SPEED, SPEED_ERR);
            let distance = speed * (duration / 1000) as f64;

            records.push(SpeedRecord {
                id: curr_id,
                begin: prev_time,
                end: curr_time,
                distance: distance,
            });

            curr_id += 1
        }

        records
    }

    pub fn to_json(&self, flag: u64) -> JsonValue {
        object! {
            "id" => self.id,
            "beginTime" => self.begin,
            "endTime" => self.end,
            "flag" => flag,
            "distance" => self.distance,
        }
    }
}
