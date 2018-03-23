use std::collections::BTreeMap;
use json::JsonValue;

use crate::utils::*;
use crate::constant::*;
use crate::entities::*;

#[derive(Debug)]
pub struct RunRecord {
    pub flag: u64,
    pub uuid: String,
    pub distance: u64,
    pub five_points: Vec<FivePoint>,
    pub start_time: u64,
    pub end_time: u64,
    pub gps_records: Vec<GPSRecord>,
    pub step_records: Vec<StepRecord>,
    pub speed_records: Vec<SpeedRecord>,
}

impl RunRecord {
    pub fn plan(
        flag: u64,
        uuid: &str,
        route_plan: &RoutePlan,
        five_points: &Vec<FivePoint>,
        start_time: u64,
    ) -> Self {
        let gps_records = GPSRecord::plan(start_time, route_plan);
        let end_time = gps_records.last().unwrap().time + 5000;
        let step_records = StepRecord::rand(start_time, end_time);
        let speed_records = SpeedRecord::rand(start_time, end_time);

        RunRecord {
            flag,
            uuid: uuid.to_string(),
            distance: gps_records.last().unwrap().sum_dis as u64,
            five_points: five_points.to_vec(),
            start_time,
            end_time,
            gps_records,
            step_records,
            speed_records,
        }
    }

    pub fn to_json(&self, uid: u64, unid: u64) -> JsonValue {
        let all_loc_json = JsonValue::Array(
            self.gps_records
                .iter()
                .map(|r| r.to_json(self.flag))
                .collect(),
        ).to_string();

        let five_point_json = JsonValue::Array(
            self.five_points
                .iter()
                .map(|p| p.to_json(self.flag))
                .collect(),
        ).to_string();

        let speed_records = JsonValue::Array(
            self.speed_records
                .iter()
                .map(|r| r.to_json(self.flag))
                .collect(),
        );
        let step_records = JsonValue::Array(
            self.step_records
                .iter()
                .map(|r| r.to_json(self.flag))
                .collect(),
        );

        let sum_time = (self.end_time - self.start_time) / 1000;
        let calorie = rand_near(
            CALORIE_PER_KM * self.distance / 1000,
            CALORIE_PER_KM_ERR * self.distance / 1000,
        );

        let mut json = object! {
            "avgStepFreq" => rand_near(STEP_CNT_PER_MIN, STEP_CNT_PER_MIN_ERR),
            "calorie" => calorie,
            "complete" => true,
            "getPrize" => false,
            "selDistance" => SEL_DISTANCE,
            "selectedUnid" => unid,
            "speed" => (50000 * sum_time) / (3 * self.distance),
            "sportType" => 1,
            "startTime" => self.start_time,
            "status" => 0,
            "stopTime" => self.end_time,
            "totalDis" => self.distance,
            "totalSteps" => self.step_records.iter().fold(0, |sum, record| sum + record.step_count),
            "totalTime" => sum_time,
            "uid" => uid,
            "unCompleteReason" => 0,
            "uuid" => self.uuid.clone(),
        };

        let mut sign_param = BTreeMap::new();

        {
            for (k, v) in json.entries() {
                sign_param.insert(k.to_string(), v.to_string());
            }
        }

        let signature = compute_sign(&sign_param, MD5_SIGN_SALT_RUN);

        let json_extend = object! {
            "allLocJson" => object! {
                "allLocJson" => all_loc_json,
            }.to_string(),
            "fivePointJson" => object!{
                "fivePointJson" => five_point_json,
            }.to_string(),
            "speedPerTenSec" => speed_records,
            "stepsPerTenSec" => step_records,
            "isUpload" => false,
            "more" => true,
            "unid" => unid,
            "signature" => signature,
        };

        if let (JsonValue::Object(obj), JsonValue::Object(obj_extend)) = (&mut json, &json_extend) {
            for (k, v) in obj_extend.iter() {
                obj.insert(k, v.clone());
            }
        }

        json
    }
}
