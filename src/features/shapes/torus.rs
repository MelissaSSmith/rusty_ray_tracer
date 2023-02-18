use crate::features::intersection::Intersection;
use crate::features::primitives::calculus::quartic;
use crate::features::primitives::operations::consts::EPSILON;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::primitives::vector::Vector;
use crate::features::ray::Ray;
use crate::features::shapes::{Intersect, Normal};
use crate::features::shapes::shape::Object;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Torus {
    swept_radius: f64,
    tube_radius: f64
}

impl Torus {
    fn create() -> Self {
        Self {
            swept_radius: 1.0,
            tube_radius: 1.0
        }
    }

    fn with_swept_radius(self, swept_radius: f64) -> Self {
        Self {
            swept_radius,
            ..self
        }
    }

    fn with_tube_radius(self, tube_radius: f64) -> Self {
        Self {
            tube_radius,
            ..self
        }
    }

    pub(crate) fn swept_radius(&self) -> f64 {
        self.swept_radius
    }

    pub(crate) fn tube_radius(&self) -> f64 {
        self.tube_radius
    }
}

impl Intersect for Torus {
    fn intersect(_object: &Object, _ray: &Ray) -> Vec<Intersection> {
        let dx = _ray.direction().x();
        let dy = _ray.direction().y();
        let dz = _ray.direction().z();

        let ox = _ray.origin().x();
        let oy = _ray.origin().y();
        let oz = _ray.origin().z();

        let sum_direction_squared = dx * dx + dy * dy + dz * dz;
        let e = ox * ox + oy * oy + oz * oz -
            _object.swept_radius() * _object.swept_radius() -
            _object.tube_radius() * _object.tube_radius();
        let f = ox * dx + oy * dy + oz * dz;
        let four_a_squared = 4.0 * _object.swept_radius() * _object.swept_radius();

        let coefficients = [
            e * e - four_a_squared * (_object.tube_radius() * _object.tube_radius() - oy * oy),
            4.0 * f * e + 2.0 * four_a_squared * oy * dy,
            2.0 * sum_direction_squared * e + 4.0 * f * f + four_a_squared * dy * dy,
            4.0 * sum_direction_squared * f,
            sum_direction_squared * sum_direction_squared];

        let solution = quartic(coefficients);

        if solution.is_empty() {
            return vec![]
        }

        let mut mint = f64::INFINITY;
        solution.iter()
            .for_each(|t| if (t > &EPSILON) && (t < &mint) {
                mint = *t
            });

        if mint.is_finite() {
            return vec![Intersection::create(mint, _object, 0.0, 0.0)];
        }

        vec![]
    }
}

impl Normal for Torus {
    fn normal(_object: &Object, _point: &Point) -> Vector {
        let object_squared = _object.swept_radius() * _object.swept_radius() +
            _object.tube_radius() * _object.tube_radius();
        let point_squared = _point.x() * _point.x() +
            _point.y() * _point.y() + _point.z() * _point.z();
        let vector = Vector::create(
            4.0 * _point.x() * (point_squared - object_squared),
            4.0 * _point.y() * (point_squared - object_squared +
                2.0 * _object.swept_radius() * _object.tube_radius()),
            4.0 * _point.z() * (point_squared - object_squared));

        vector.normalize()
    }
}