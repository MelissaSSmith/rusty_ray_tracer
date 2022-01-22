use std::any::Any;
use crate::features::intersection::Intersection;
use crate::features::material::Material;
use crate::features::primitives::matrix::Matrix;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::ray::Ray;
use crate::features::primitives::vector::Vector;
use crate::features::shapes::Shape::{Plane, Sphere};
use crate::features::shapes::plane::Plane;
use crate::features::shapes::sphere::Sphere;

pub mod sphere;
pub mod plane;

//todo: derive => Debug, PartialEq, Serialize, Deserialize
#[derive(Clone)]
pub enum Shape {
    Object,
    Sphere,
    Plane
}

impl Shape {
    pub fn create(&self) -> Object {
        match self {
            Sphere => { Object::create(String::from("Sphere")) }
            Plane => { Object::create(String::from("Plane")) }
            _ => { Object::create(String::from("Object")) }
        }
    }

    pub fn intersect(&self, _object: &Object, _ray: &Ray) -> Vec<Intersection> {
        match self {
            Sphere => { Sphere::intersect(_object, _ray) }
            Plane => { Plane::intersect(_object, _ray) }
            _ => { vec![] }
        }
    }

    pub fn normal(&self, _object: &Object, _point: &Point) -> Vector {
        match self {
            Sphere => { Sphere::normal(_object, _point) }
            Plane => { Sphere::normal(_object, _point) }
            _ => { Vector::zero() }
        }
    }
}

pub trait ShapeTrait: Any {
    fn box_clone(&self) -> Box<dyn ShapeTrait>;
    fn as_any(&self) -> &dyn Any;
}

pub trait Intersect {
    fn intersect(_object: &Object, _ray: &Ray) -> Vec<Intersection>;
}

pub trait Normal {
    fn normal(_object: &Object, _point: &Point) -> Vector;
}

pub trait ShapeAttributes<Rhs = Self> { //todo: kill and make a part of object
    type Output;

    fn transformation(&self) -> Matrix;
    fn material(&self) -> Material;
    fn shape(&self) -> String;
    fn set_transform(&mut self, _transformation: Matrix);
    fn set_material(&mut self, _material: Material);
    fn with_transform(self, _matrix: Matrix) -> Self::Output;
    fn with_material(self, _material: Material) -> Self::Output;

    fn equals(&self, other: &Box<dyn ShapeTrait>) -> bool {
        self.shape() == other.shape() &&
            self.material().equals(other.material()) &&
            self.transformation().equals(other.transformation())
    }
}

pub struct Object {
    transformation: Matrix,
    material: Material,
    shape: String
}

impl Object {
    fn create(shape_type: String) -> Object {
        Object {
            transformation: Matrix::identity(),
            material: Material::create(),
            shape: shape_type
        }
    }

    fn to_shape(self) -> Shape {
        match self.shape.as_ref() {
            "Sphere" => { Shape::Sphere(self) },
            "Plane" => { Shape::Plane(self) },
            _ => { Shape::Object }
        }
    }
}

impl ShapeAttributes for Object {
    type Output = Object;

    fn transformation(&self) -> Matrix {
        self.transformation.clone()
    }

    fn material(&self) -> Material {
        self.material.clone()
    }

    fn shape(&self) -> String {
        self.shape.clone()
    }

    fn set_transform(&mut self, _transformation: Matrix) {
        self.transformation = _transformation;
    }

    fn set_material(&mut self, _material: Material) {
        self.material = _material;
    }

    fn with_transform(self, _transform: Matrix) -> Self::Output {
        Object {
            transformation: _transform,
            ..self
        }
    }

    fn with_material(self, _material: Material) -> Self::Output {
        Object {
            material: _material,
            ..self
        }
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