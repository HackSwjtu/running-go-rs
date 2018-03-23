use json::JsonValue;

use crate::utils::*;
use crate::constant::*;
use crate::entities::*;

#[derive(Debug)]
pub struct GPSRecord {
    pub time: u64,
    pub id: u64,
    pub speed: f64,
    pub avg_speed: f64,
    pub pos: GeoPoint,
    pub sum_dis: f64,
    pub sum_time: f64,
    pub point_type: u64,
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

            let this_point = vectors[curr_point_idx];
            let next_point = vectors[curr_point_idx+  1];
            let target_point = if curr_pos.distance_to(this_point) < CONFIRM_DISTANCE {
                next_point
            } else {
                this_point
            };

            curr_pos = curr_pos
                .step_toward(target_point, distance)
                .fuzz(FUZZLE_ERR);
                
            if curr_pos.distance_to(target_point) < CONFIRM_DISTANCE {
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

        records.first_mut().unwrap().point_type = 5;
        records.last_mut().unwrap().point_type = 6;

        records
    }

    pub fn to_json(&self, flag: u64) -> JsonValue {
        object! {
            "id" => self.id.to_string(),
            "pointid" => self.id.to_string(),
            "flag" => flag.to_string(),
            "radius" => 180.to_string(),
            "lat" => self.pos.lat.to_string(),
            "lng" => self.pos.lon.to_string(),
            "totaldis" => self.sum_dis.to_string(),
            "totaltime" => self.sum_time.round() as u64,
            "speed" => self.speed.to_string(),
            "avgspeed" => self.avg_speed.to_string(),
            "gaintime" => self.time.to_string(),
            "type" => self.point_type.to_string(),
            "locationtype" => 0.to_string(),
        }
    }
}
