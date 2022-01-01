use crate::features::intersection::Intersection;
use crate::features::point::Point;
use crate::features::ray::Ray;
use crate::features::vector::Vector;

pub mod sphere;

pub trait Shape {
    type Item;
    fn intersect(&self, _ray: Ray) -> Vec<Intersection<Self::Item>> where <Self as Shape>::Item: Shape;
    fn normal(&self, _point: Point) -> Vector;
    fn equals(&self, _: Self::Item) -> bool;
}