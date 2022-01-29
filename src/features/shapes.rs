use crate::features::intersection::Intersection;
use crate::features::primitives::point::Point;
use crate::features::primitives::vector::Vector;
use crate::features::ray::Ray;
use crate::features::shapes::shape::Object;

pub mod sphere;
pub mod plane;
pub mod shape;
pub mod cube;
pub mod cylinder;

pub trait Intersect {
    fn intersect(_object: &Object, _ray: &Ray) -> Vec<Intersection>;
}

pub trait Normal {
    fn normal(_object: &Object, _point: &Point) -> Vector;
}