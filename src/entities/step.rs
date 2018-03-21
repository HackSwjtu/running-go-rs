use json::JsonValue;

use crate::config::*;
use crate::utils::*;

#[derive(Debug)]
pub struct StepRecord {
    pub id: u32,
    pub begin: u64,
    pub end: u64,
    pub step_count: u32,
    pub avg_diff: f64,
    pub max_diff: f64,
    pub min_diff: f64,
}

impl StepRecord {
    pub fn rand(start_time: u64, end_time: u64) -> Vec<Self> {
        let mut records = Vec::new();
        let mut curr_id = 0;
        let mut curr_time = start_time;

        while curr_time < end_time {
            let prev_time = curr_time;
            curr_time += rand_near_f64(10.0 * 1000.0, SPAMLE_TIME_ERR * 1000.0) as u64;

            records.push(StepRecord {
                id: curr_id,
                begin: prev_time,
                end: curr_time,
                step_count: rand_near(STEP_CNT_PER_10S, STEP_CNT_PER_10S_ERR),
                avg_diff: rand_near_f64(AVG_DIFF, AVG_DIFF_ERR),
                min_diff: rand_near_f64(MIN_DIFF, MIN_DIFF_ERR),
                max_diff: rand_near_f64(MAX_DIFF, MAX_DIFF_ERR),
            });

            curr_id += 1;
        }

        records
    }

    pub fn to_json(&self, flag: u64) -> JsonValue {
        object! {
            "id" => self.id,
            "flag" => flag,
            "beginTime" => self.begin,
            "endTime" => self.end,
            "stepsNum" => self.step_count,
            "minDiff" => self.min_diff,
            "maxDiff" => self.max_diff,
            "avgDiff" => self.avg_diff
        }
    }
}
