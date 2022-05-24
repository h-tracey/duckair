fn main() {
    const EARTH_RADIUS_IN_KM : f64 = 6371.0;

    let lat_degrees_one: f64 = 41.4075;
    let long_degrees_one: f64 = -81.851111;

    let lat_degrees_two: f64 = 40.7861;
    let long_degrees_two: f64 = -111.9822;

    let lat_radians_one: f64 = lat_degrees_one.to_radians();
    let lat_radians_two: f64 = lat_degrees_two.to_radians();

    let delta_lat: f64 = (lat_degrees_one - lat_degrees_two).to_radians();
    let delta_long: f64 = (long_degrees_one - long_degrees_two).to_radians();

    let inner_circ_angle = f64::powi((delta_lat / 2.0).sin(),2)
        + lat_radians_one.cos()
        * lat_radians_two.cos()
        * f64::powi((delta_long / 2.0).sin(), 2);
    
    let central_angle: f64 = 2.0 * inner_circ_angle.sqrt().asin();

    let distance: f64 = EARTH_RADIUS_IN_KM * central_angle;

    println!("Distance between points: {:.1} km", distance);
}
