use crate::prelude::*;

/// Finds the unsigned minimum difference between two angles
/// in radians. Will calculate across zero
///
/// # Arguments
/// * `a`: An angle in radians
/// * `b`: An angle in radians
///
/// # Returns
/// The unsigned minimum difference between the two angles,
/// in radians
pub fn angle_difference(a: f64, b: f64) -> f64 {
    let diff = (a - b).abs();

    if diff > f64::consts::PI {
        f64::consts::TAU - diff
    } else {
        diff
    }
}

/// Wraps an angle (rad) to the range of 0 >= ang > 2pi.
pub fn wrap_angle(angle: &mut f64) {
    while *angle < 0.0 {
        *angle += f64::consts::TAU;
    }

    while *angle >= f64::consts::TAU {
        *angle -= f64::consts::TAU;
    }
}

/// Steps towards a target value by a given size.
///
/// # Arguments
/// * `num`: A reference to the number to step.
/// * `target`: The target value.
/// * `step`: The size of the step.
pub fn towards(num: &mut f64, target: f64, step: f64) {
    if (*num - target).abs() <= step {
        *num = target;
    } else if *num < target {
        *num += step;
    } else {
        *num -= step;
    }
}

/// Steps an angular value towards a target angle, taking the
/// shortest path with a specified step size. Modifies the
/// argument `current`. Target will always lie in the range
/// of 0 >= ang > 2pi.
///
/// # Arguments
/// * `current`: A reference to the current angle.
/// * `target`: The target angle.
/// * `step`: The size of the step.
pub fn towards_angular(current: &mut f64, mut target: f64, step: f64) {
    wrap_angle(current);
    wrap_angle(&mut target);

    let diff = (*current - target + f64::consts::PI).rem_euclid(f64::consts::TAU) - f64::consts::PI;

    if diff.abs() <= step || diff.abs() > f64::consts::PI {
        *current = target;
    } else {
        *current += step * diff.signum();
    }

    wrap_angle(current);
}
