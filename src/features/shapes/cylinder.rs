use crate::features::intersection::Intersection;
use crate::features::primitives::point::Point;
use crate::features::primitives::vector::Vector;
use crate::features::ray::Ray;
use crate::features::shapes::{Intersect, Normal};
use crate::features::shapes::shape::Object;

#[derive(Clone)]
pub struct Cylinder {}

impl Intersect for Cylinder {
    fn intersect(_object: &Object, _ray: &Ray) -> Vec<Intersection> {
        todo!()
    }
}

impl Normal for Cylinder {
    fn normal(_object: &Object, _point: &Point) -> Vector {
        todo!()
    }
}

#[cfg(test)]
mod tests {

}