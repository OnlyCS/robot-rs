pub trait AngularUnit {
    const RAD_PER_UNIT: f64;
}

pub struct Radian;
pub struct Degree;

impl AngularUnit for Radian {
    const RAD_PER_UNIT: f64 = 1.0;
}

impl AngularUnit for Degree {
    const RAD_PER_UNIT: f64 = std::f64::consts::PI / 180.0;
}
