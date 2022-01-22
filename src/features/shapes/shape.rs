use crate::features::intersection::Intersection;
use crate::features::material::Material;
use crate::features::primitives::matrix::Matrix;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::primitives::vector::Vector;
use crate::features::ray::Ray;
use crate::features::shapes::{Intersect, Normal};
use crate::features::shapes::sphere::Sphere;
use crate::features::shapes::plane::Plane;

//todo: derive => Debug, PartialEq, Serialize, Deserialize (requires Matrix not using Vec in backend)
#[derive(Clone)]
pub enum Shape {
    Object,
    Sphere,
    Plane
}

impl Shape {
    pub fn create(&self) -> Object {
        match self {
            Shape::Sphere => { Object::create(String::from("Sphere")) }
            Shape::Plane => { Object::create(String::from("Plane")) }
            _ => { Object::create(String::from("Object")) }
        }
    }

    pub fn air(&self) -> Object {
        match self {
            Shape::Sphere => { Object::air(String::from("Sphere")) }
            Shape::Plane => { Object::air(String::from("Plane")) }
            _ => { Object::air(String::from("Object")) }
        }
    }

    pub fn vacuum(&self) -> Object {
        match self {
            Shape::Sphere => { Object::vacuum(String::from("Sphere")) }
            Shape::Plane => { Object::vacuum(String::from("Plane")) }
            _ => { Object::vacuum(String::from("Object")) }
        }
    }

    pub fn water(&self) -> Object {
        match self {
            Shape::Sphere => { Object::water(String::from("Sphere")) }
            Shape::Plane => { Object::water(String::from("Plane")) }
            _ => { Object::water(String::from("Object")) }
        }
    }

    pub fn diamond(&self) -> Object {
        match self {
            Shape::Sphere => { Object::diamond(String::from("Sphere")) }
            Shape::Plane => { Object::diamond(String::from("Plane")) }
            _ => { Object::diamond(String::from("Object")) }
        }
    }

    pub fn glass(&self) -> Object {
        match self {
            Shape::Sphere => { Object::glass(String::from("Sphere")) }
            Shape::Plane => { Object::glass(String::from("Plane")) }
            _ => { Object::glass(String::from("Object")) }
        }
    }
}

#[derive(Clone)]
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

    fn glass(shape_type: String) -> Object {
        let material = Material::create()
            .with_transparency(1.0)
            .with_refractive_index(1.5);
        Object {
            transformation: Matrix::identity(),
            material,
            shape: shape_type
        }
    }

    fn air(shape_type: String) -> Object {
        let material = Material::create()
            .with_transparency(1.0)
            .with_refractive_index(1.00029);
        Object {
            transformation: Matrix::identity(),
            material,
            shape: shape_type
        }
    }

    fn vacuum(shape_type: String) -> Object {
        let material = Material::create()
            .with_transparency(1.0)
            .with_refractive_index(1.0);
        Object {
            transformation: Matrix::identity(),
            material,
            shape: shape_type
        }
    }

    fn water(shape_type: String) -> Object {
        let material = Material::create()
            .with_transparency(1.0)
            .with_refractive_index(1.33);
        Object {
            transformation: Matrix::identity(),
            material,
            shape: shape_type
        }
    }

    fn diamond(shape_type: String) -> Object {
        let material = Material::create()
            .with_transparency(1.0)
            .with_refractive_index(2.417);
        Object {
            transformation: Matrix::identity(),
            material,
            shape: shape_type
        }
    }

    pub fn equals(&self, other: &Object) -> bool {
        self.shape() == other.shape() &&
            self.material().equals(other.material()) &&
            self.transformation().equals(other.transformation())
    }

    pub fn transformation(&self) -> Matrix {
        self.transformation.clone()
    }

    pub fn material(&self) -> Material {
        self.material.clone()
    }

    pub fn shape(&self) -> String {
        self.shape.clone()
    }

    pub fn set_transform(&mut self, _transformation: Matrix) {
        self.transformation = _transformation;
    }

    pub fn set_material(&mut self, _material: Material) {
        self.material = _material;
    }

    pub fn with_transform(self, _transform: Matrix) -> Object {
        Object {
            transformation: _transform,
            ..self
        }
    }

    pub fn with_material(self, _material: Material) -> Object {
        Object {
            material: _material,
            ..self
        }
    }
}

impl Intersect for Object {
    fn intersect(_object: &Object, _ray: &Ray) -> Vec<Intersection> {
        match _object.shape.as_ref() {
            "Sphere" => { Sphere::intersect(_object, _ray) }
            "Plane" => { Plane::intersect(_object, _ray) }
            _ => { vec![] }
        }
    }
}

impl Normal for Object {
    fn normal(_object: &Object, _point: &Point) -> Vector {
        match _object.shape.as_ref() {
            "Sphere" => { Sphere::normal(_object, _point) }
            "Plane" => { Plane::normal(_object, _point) }
            _ => { Vector::zero() }
        }
    }
}
