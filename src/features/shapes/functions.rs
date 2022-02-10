use crate::features::primitives::operations::consts::EPSILON;
use crate::features::shapes::shape::Object;

impl Object {
    pub(crate) fn check_axis(origin: &f64, direction: &f64, min: f64, max: f64) -> (f64, f64) {
        let mut tmin = 0.0;
        let mut tmax = 0.0;

        let tmin_numerator = min - origin;
        let tmax_numerator = max - origin;

        if direction.abs() >= EPSILON {
            tmin = tmin_numerator / direction;
            tmax = tmax_numerator / direction;
        } else {
            tmin = tmin_numerator * f64::INFINITY;
            tmax = tmax_numerator * f64::INFINITY;
        }

        if tmin > tmax {
            return (tmax, tmin);
        }
        (tmin, tmax)
    }
}
