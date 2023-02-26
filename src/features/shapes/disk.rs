use crate::features::intersection::Intersection;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::primitives::vector::Vector;
use crate::features::ray::Ray;
use crate::features::shapes::{Intersect, Normal};
use crate::features::shapes::shape::Object;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Disk {
    radius: f64,
    inner_radius: f64,
    height: f64,
    phi_max: f64,
    center: Point,
    normal: Vector
}

impl Disk {
    pub fn create() -> Self {
        Self {
            radius: 0.0,
            inner_radius: 0.0,
            height: 0.0,
            phi_max: 360.0,
            center: Point::zero(),
            normal: Vector::create(0.0, 0.0, -1.0)
        }
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn inner_radius(&self) -> f64 {
        self.inner_radius
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn center(&self) -> Point {
        self.center
    }

    pub fn phi_max(&self) -> f64 {
        self.phi_max
    }

    pub fn with_radius(self, radius: f64) -> Self {
        Self {
            radius,
            ..self
        }
    }

    pub fn with_inner_radius(self, inner_radius: f64) -> Self {
        Self {
            inner_radius,
            ..self
        }
    }

    pub fn with_center(self, center: Point) -> Self {
        Self {
            center,
            ..self
        }
    }

    pub fn with_height(self, height: f64) -> Self {
        Self {
            height,
            ..self
        }
    }

    fn hash_three(n: f64) -> Vector {
        let vector = Vector::create(n.sin(), (n + 1.0).sin(), (n + 2.0).sin());
        (vector * Vector::create(43758.5453123,12578.1459123,19642.3490423))
    }

    fn area(&self) -> f64 {
        self.phi_max() * 0.5 * (self.radius().powi(2) - self.inner_radius().powi(2))
    }
}

impl Intersect for Disk {
    fn intersect(_object: &Object, _ray: &Ray) -> Vec<Intersection> {
        let h = _ray.origin().z() + _ray.direction().z();
        let t = (h - _ray.origin().z()) / _ray.direction().z();
        let hit = (_object.height() - _ray.origin().z()) / _ray.direction().z();
        if hit <= 0.0 {
            return vec![];
        }
        if _ray.direction().z() == 0.0 {
            return vec![];
        }
        todo!()
    }
}

impl Normal for Disk {
    fn normal(_object: &Object, _point: &Point) -> Vector {
        let point_squared = _point.x().powi(2) + _point.y().powi(2) + _point.z().powi(2);
        *_point - Point::create(0.0, 0.0, -1.0)
    }
}