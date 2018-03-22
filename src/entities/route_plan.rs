use crate::entities::*;

#[derive(Debug)]
pub struct RoutePlan {
    pub route_points: Vec<GeoPoint>,
    pub min_distance: u64,
    pub min_points: u64,
}
