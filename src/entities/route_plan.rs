use crate::entities::*;

#[derive(Debug)]
pub struct RoutePlan {
    pub route_points: Vec<GeoPoint>,
    pub min_distance: u32,
    pub min_points: u32,
}
