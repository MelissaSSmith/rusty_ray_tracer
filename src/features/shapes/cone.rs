use crate::features::intersection::Intersection;
use crate::features::primitives::point::Point;
use crate::features::primitives::vector::Vector;
use crate::features::ray::Ray;
use crate::features::shapes::{Intersect, Normal};
use crate::features::shapes::shape::Object;

#[derive(Clone, Copy)]
pub struct Cone {}

impl Intersect for Cone {
    fn intersect(_object: &Object, _ray: &Ray) -> Vec<Intersection> {
        todo!()
    }
}

impl Normal for Cone {
    fn normal(_object: &Object, _point: &Point) -> Vector {
        todo!()
    }
}

#[cfg(test)]
mod tests {

}