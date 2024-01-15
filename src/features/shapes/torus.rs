use crate::features::intersection::Intersection;
use crate::features::math::quartic_algebra::quartic;
use crate::features::primitives::operations::consts::{LOW_EPSILON};
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::primitives::vector::Vector;
use crate::features::ray::Ray;
use crate::features::shapes::{Intersect, Normal};
use crate::features::shapes::shape::Object;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Torus {
    radius: f64,
    tube_radius: f64,
    center: Point
}

impl Torus {
    pub fn create() -> Self {
        Self {
            radius: 1.0,
            tube_radius: 0.5,
            center: Point::zero()
        }
    }

    pub fn with_radius(self, radius: f64) -> Self {
        Self {
            radius,
            ..self
        }
    }

    pub fn with_tube_radius(self, tube_radius: f64) -> Self {
        Self {
            tube_radius,
            ..self
        }
    }

    pub fn with_center(self, center: Point) -> Self {
        Self {
            center,
            ..self
        }
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn tube_radius(&self) -> f64 {
        self.tube_radius
    }

    pub fn center(&self) -> Point {
        self.center
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

        let a = _object.radius();
        let b = _object.tube_radius();

        let sum_direction_squared = dx * dx + dy * dy + dz * dz;
        let e = ox * ox + oy * oy  + oz * oz - a * a - b * b;
        let f = ox * dx + oy * dy + oz * dz;
        let four_a_squared = 4.0 * a * a;

        let coefficients = [
            e * e - four_a_squared * (b * b - oy * oy),
            4.0 * f * e + 2.0 * four_a_squared * oy * dy,
            2.0 * sum_direction_squared * e + 4.0 * f * f + four_a_squared * dy * dy,
            4.0 * sum_direction_squared * f,
            sum_direction_squared * sum_direction_squared
        ];

        let solution = quartic(coefficients);

        if solution.is_empty() {
            return vec![]
        }

        let mut mint = f64::INFINITY;
        solution.iter()
            .for_each(|t| if (t > &LOW_EPSILON) && (t < &mint) {
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
        let object_squared = _object.radius().powi(2) + _object.tube_radius().powi(2);
        let point_squared = _point.x().powi(2) + _point.y().powi(2) + _point.z().powi(2);
        Vector::create(
            4.0 * _point.x() * (point_squared - object_squared),
            4.0 * _point.y() * (point_squared - object_squared +
                2.0 * _object.radius() * _object.radius()),
            4.0 * _point.z() * (point_squared - object_squared)
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;
    use crate::features::primitives::vector::Vector;
    use crate::features::shapes::Normal;
    use crate::features::shapes::shape::Shape;
    use crate::features::shapes::torus::Torus;

    #[test]
    fn test_intersecting_a_torus_with_a_ray() {

    }

    #[test]
    fn test_computing_the_normal_vector_on_a_torus() {
        let torus = Shape::Torus(Torus::create()).create();

        let tests = vec![
            (Point::create(0.0, 0.0, 0.0), Vector::create(0.0, 0.0, 0.0)),
            (Point::create(1.0, 1.0, 1.0), Vector::create(7.0, 15.0, 7.0)),
            (Point::create(-1.0, -1.0, 0.0), Vector::create(-3.0, -11.0, 0.0))
        ];

        for test in tests {
            let normal = Torus::normal(&torus, &test.0);

            assert_eq!(test.1, normal);
        }
    }
}