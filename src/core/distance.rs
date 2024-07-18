const EARTH_RADIUS_M: f32 = 6371e3;


pub fn distance_haversine_m(lat1: f32, lon1: f32, lat2: f32, lon2: f32) -> f32 {
    let lat1r = lat1.to_radians();
    let lat2r = lat2.to_radians();
    let lon1r = lon1.to_radians();
    let lon2r = lon2.to_radians();

    let dlon = lon2r - lon1r;
    let dlat = lat2r - lat1r;
    let a = f32::powi(f32::sin(dlat / 2.0), 2) + f32::cos(lat1r) * f32::cos(lat2r) * f32::powi(f32::sin(dlon / 2.0), 2);

    return 2.0 * f32::asin(f32::sqrt(a)) * EARTH_RADIUS_M;
}