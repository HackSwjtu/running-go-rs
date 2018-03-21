use json::JsonValue;

use crate::entities::*;

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
