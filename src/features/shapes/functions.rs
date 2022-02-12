use crate::features::primitives::operations::consts::EPSILON;
use crate::features::shapes::shape::Object;

impl Object {
    pub(crate) fn check_axis(origin: &f64, direction: &f64, min: f64, max: f64) -> (f64, f64) {
        let tmin_numerator = min - origin;
        let tmax_numerator = max - origin;

        let tmin = tmin_numerator / direction;
        let tmax = tmax_numerator / direction;

        if tmin > tmax {
            return (tmax, tmin);
        }
        (tmin, tmax)
    }
}
