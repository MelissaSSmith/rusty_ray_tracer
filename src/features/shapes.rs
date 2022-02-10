use crate::features::intersection::Intersection;
use crate::features::primitives::point::Point;
use crate::features::primitives::vector::Vector;
use crate::features::ray::Ray;
use crate::features::shapes::shape::Object;
use crate::features::world::World;

pub mod sphere;
pub mod plane;
pub mod shape;
pub mod cube;
pub mod cylinder;
pub mod cone;
pub mod group;
pub mod functions;

pub trait Intersect {
    fn intersect(_object: &Object, _ray: &Ray) -> Vec<Intersection>;
}

pub trait Normal {
    fn normal(_object: &Object, _point: &Point) -> Vector;
}

pub trait NormalAt {
    fn normal(_object: &Object, _point: &Point, _world: Option<&World>) -> Vector;
}