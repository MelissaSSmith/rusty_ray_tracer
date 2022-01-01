use crate::features::intersection::Intersection;
use crate::features::ray::Ray;

pub mod sphere;

pub trait Shape {
    type Item;
    fn intersect(&self, _ray: Ray) -> Vec<Intersection<Self::Item>> where <Self as Shape>::Item: Shape;
    fn equals(&self, _: Self::Item) -> bool;
}