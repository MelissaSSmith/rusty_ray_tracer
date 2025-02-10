use linear_algebra::point::Point;
use linear_algebra::vector::Vector;
use crate::features::intersection::Intersection;
use crate::features::ray::Ray;
use crate::features::shapes::shape::Object;

pub mod sphere;
pub mod plane;
pub mod shape;
pub mod cube;
pub mod cylinder;
pub mod cone;
pub mod group;
pub mod functions;
pub mod triangle;
pub mod smooth_triangle;
pub mod csg;
pub mod torus;
pub mod disk;

pub trait Intersect {
    fn intersect(_object: &Object, _ray: &Ray) -> Vec<Intersection>;
}

pub trait Normal {
    fn normal(_object: &Object, _point: &Point) -> Vector;
}

pub trait NormalAt {
    fn normal(_object: &Object, _point: &Point, _hit: &Intersection) -> Vector;
}