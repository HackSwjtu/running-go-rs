use chrono::{FixedOffset, TimeZone};
use json::JsonValue;

use crate::utils::*;
use crate::config::*;
use crate::entities::*;

#[derive(Debug)]
pub struct GPSRecord {
    pub time: u64,
    pub id: u32,
    pub speed: f64,
    pub avg_speed: f64,
    pub pos: GeoPoint,
    pub sum_dis: f64,
    pub sum_time: f64,
    pub point_type: u32,
}

impl GPSRecord {
    pub fn plan(start_time: u64, route_plan: &RoutePlan) -> Vec<Self> {
        let mut records = Vec::new();
        let start_pos = route_plan.route_points.first().unwrap().clone();
        let vectors: Vec<Vector> = route_plan
            .route_points
            .iter()
            .map(|p| p.get_offset_of(start_pos))
            .collect();
        let mut curr_id = 0;
        let mut curr_point_idx = 0;
        let mut curr_time = start_time;
        let mut curr_pos = Vector { x: 0.0, y: 0.0 };
        let mut sum_time = 0.0;
        let mut sum_dis = 0.0;

        while sum_dis < route_plan.min_distance as f64
            || curr_point_idx < route_plan.min_points as usize
        {
            let speed = rand_near_f64(AVG_SPEED, SPEED_ERR);
            let duration = rand_near_f64(SPAMLE_TIME * 1000.0, SPAMLE_TIME_ERR * 1000.0);
            let distance = speed * duration / 1000.0;

            let target_point = vectors[curr_point_idx];
            curr_pos = curr_pos.step_toward(target_point, distance).fuzzle();
            if curr_pos.distance_to(target_point) < 5.0 {
                curr_point_idx += 1;
            }

            let speed_weird = (50.0 * sum_time) / (3.0 * sum_dis);
            let avg_speed_weird = rand_near_f64(speed_weird, 0.2);
            records.push(GPSRecord {
                id: curr_id,
                time: curr_time,
                speed: speed_weird,
                avg_speed: avg_speed_weird,
                pos: start_pos.offset(curr_pos),
                sum_dis: sum_dis,
                sum_time: sum_time,
                point_type: 0,
            });

            curr_id += 1;
            curr_time += duration.round() as u64;
            sum_dis += distance;
            sum_time += duration / 1000.0;
        }

        records
    }

    pub fn to_json(&self, flag: u64) -> JsonValue {
        let time_zone = FixedOffset::east(8 * 3600);
        let time_format = time_zone
            .timestamp(self.time as i64 / 1000, 0)
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        object! {
            "id" => self.id,
            "flag" => flag,
            "lat" => self.pos.lat,
            "lng" => self.pos.lon,
            "totalDis" => self.sum_dis / 1000.0,
            "totalTime" => self.sum_time.round() as u32,
            "speed" => self.speed,
            "avgSpeed" => self.avg_speed,
            "gainTime" => time_format,
            "type" => self.point_type,
            "locType" => 61,
            "radius" => 180,
        }
    }
}
