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
    center: Point
}

impl Disk() {
    pub fn create() -> Self {
        Self {
            radius: 0.0,
            inner_radius: 0.0,
            center: Point::zero()
        }
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn inner_radius(&self) -> f64 {
        self.inner_radius
    }

    pub fn center(&self) -> Point {
        self.center
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
}

impl Intersect for Disk {
    fn intersect(_object: &Object, _ray: &Ray) -> Vec<Intersection> {
        todo!()
    }
}

impl Normal for Disk {
    fn normal(_object: &Object, _point: &Point) -> Vector {
        todo!()
    }
}