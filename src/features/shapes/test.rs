use crate::features::intersection::Intersection;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::primitives::vector::Vector;
use crate::features::ray::Ray;
use crate::features::shapes::{Intersect, Normal};
use crate::features::shapes::shape::{Object, Shape};

#[derive(Clone, Copy)]
pub struct TestShape {
    saved_ray: Option<Ray>
}

impl TestShape {
    pub fn create() -> TestShape {
        TestShape {
            saved_ray: None
        }
    }
}

impl Intersect for TestShape {
    fn intersect(_object: &Object, _ray: &Ray) -> Vec<Intersection> {
        let test_shape = TestShape {
            saved_ray: Some(*_ray)
        };
        vec![Intersection::create(0.0, &Shape::TestShape(test_shape).create())]
    }
}

impl Normal for TestShape {
    fn normal(_object: &Object, _point: &Point) -> Vector {
        Vector::create(_point.x(), _point.y(), _point.z())
    }
}