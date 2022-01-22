use std::any::Any;
use crate::features::intersection::Intersection;
use crate::features::material::Material;
use crate::features::primitives::matrix::Matrix;
use crate::features::primitives::point::Point;
use crate::features::ray::Ray;
use crate::features::primitives::vector::Vector;
use serde::{Deserialize, Serialize};
use crate::features::shapes::Shape::{Plane, Sphere};
use crate::features::shapes::plane::Plane;
use crate::features::shapes::sphere::Sphere;

pub mod sphere;
pub mod plane;


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane)
}

impl Shape {
    fn intersect(&self, _ray: Ray) -> Vec<Intersection> {
        match self {
            Sphere(s) => { s.intersect(_ray) }
            Plane(p) => { p.intersect(_ray) }
        }
    }

    fn normal(&self, _point: Point) -> Vector {
        match self {
            Sphere(s) => { s.normal(_point) }
            Plane(p) => { p.normal(_point) }
        }
    }
}

pub trait ShapeTrait: Any {
    fn box_clone(&self) -> Box<dyn ShapeTrait>;
    fn as_any(&self) -> &dyn Any;
}

pub trait ShapeAttributes<Rhs = Self> {
    type Output;

    fn transformation(&self) -> Matrix;
    fn material(&self) -> Material;
    fn shape(&self) -> String;
    fn set_transform(&mut self, _transformation: Matrix);
    fn set_material(&mut self, _material: Material);
    fn with_transform(self, rhs: Rhs) -> Self::Output;
    fn with_material(self, rhs: Rhs) -> Self::Output;

    fn equals(&self, other: &Box<dyn ShapeTrait>) -> bool {
        self.shape() == other.shape() &&
            self.material().equals(other.material()) &&
            self.transformation().equals(other.transformation())
    }
}

impl Clone for Box<dyn ShapeTrait> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

impl PartialEq for Box<dyn ShapeTrait> {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}