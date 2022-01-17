use std::any::Any;
use crate::features::intersection::Intersection;
use crate::features::material::Material;
use crate::features::primitives::matrix::Matrix;
use crate::features::primitives::point::Point;
use crate::features::ray::Ray;
use crate::features::primitives::vector::Vector;

pub mod sphere;
pub mod plane;

pub trait Shape: Any {
    fn box_clone(&self) -> Box<dyn Shape>;
    fn as_any(&self) -> &dyn Any;
    fn transformation(&self) -> Matrix;
    fn material(&self) -> Material;
    fn set_transform(&mut self, _transformation: Matrix);
    fn set_material(&mut self, _material: Material);
    fn normal(&self, _point: Point) -> Vector;
    fn intersect(&self, _ray: Ray) -> Vec<Intersection>;
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}