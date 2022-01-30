use std::iter::Map;
use crate::features::intersection::Intersection;
use crate::features::material::Material;
use crate::features::primitives::matrix::Matrix;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::primitives::vector::Vector;
use crate::features::ray::Ray;
use crate::features::shapes::{Intersect, Normal};
use crate::features::shapes::cone::Cone;
use crate::features::shapes::cube::Cube;
use crate::features::shapes::cylinder::Cylinder;
use crate::features::shapes::sphere::Sphere;
use crate::features::shapes::plane::Plane;
use crate::features::shapes::shape::Shape::Cone;

//todo: derive => Debug, PartialEq, Serialize, Deserialize (requires Matrix not using Vec in backend)
#[derive(Clone)]
pub enum Shape {
    Object,
    Sphere,
    Plane,
    Cube,
    Cylinder(Cylinder),
    Cone
}

impl Shape {
    fn as_str(&self) -> &'static str {
        match self {
            Shape::Sphere => "Sphere",
            Shape::Plane => "Plane",
            Shape::Cube => "Cube",
            Shape::Cylinder(_) => "Cylinder",
            Shape::Cone => "Cone",
            _ => "Object"
        }
    }

    pub fn create(&self) -> Object {
        match self {
            Shape::Sphere => { Object::create(Shape::Sphere) }
            Shape::Plane => { Object::create(Shape::Plane) }
            Shape::Cube => { Object::create(Shape::Cube) }
            Shape::Cylinder(c) => { Object::create(Shape::Cylinder(*c)) }
            Shape::Cone => { Object::create(Shape::Cone) }
            _ => { Object::create(Shape::Object) }
        }
    }

    pub fn air(&self) -> Object {
        match self {
            Shape::Sphere => { Object::air(Shape::Sphere) }
            Shape::Plane => { Object::air(Shape::Plane) }
            Shape::Cube => { Object::air(Shape::Cube) }
            Shape::Cylinder(c) => { Object::air(Shape::Cylinder(*c)) }
            Shape::Cone => { Object::air(Shape::Cone) }
            _ => { Object::air(Shape::Object) }
        }
    }

    pub fn vacuum(&self) -> Object {
        match self {
            Shape::Sphere => { Object::vacuum(Shape::Sphere) }
            Shape::Plane => { Object::vacuum(Shape::Plane) }
            Shape::Cube => { Object::vacuum(Shape::Cube) }
            Shape::Cylinder(c) => { Object::vacuum(Shape::Cylinder(*c)) }
            Shape::Cone => { Object::vacuum(Shape::Cone) }
            _ => { Object::vacuum(Shape::Object) }
        }
    }

    pub fn water(&self) -> Object {
        match self {
            Shape::Sphere => { Object::water(Shape::Sphere) }
            Shape::Plane => { Object::water(Shape::Plane) }
            Shape::Cube => { Object::water(Shape::Cube) }
            Shape::Cylinder(c) => { Object::water(Shape::Cylinder(*c)) }
            Shape::Cone => { Object::water(Shape::Cone) }
            _ => { Object::water(Shape::Object) }
        }
    }

    pub fn diamond(&self) -> Object {
        match self {
            Shape::Sphere => { Object::diamond(Shape::Sphere) }
            Shape::Plane => { Object::diamond(Shape::Plane) }
            Shape::Cube => { Object::diamond(Shape::Cube) }
            Shape::Cylinder(c) => { Object::diamond(Shape::Cylinder(*c)) }
            Shape::Cone => { Object::diamond(Shape::Cone) }
            _ => { Object::diamond(Shape::Object) }
        }
    }

    pub fn glass(&self) -> Object {
        match self {
            Shape::Sphere => { Object::glass(Shape::Sphere) }
            Shape::Plane => { Object::glass(Shape::Plane) }
            Shape::Cube => { Object::glass(Shape::Cube) }
            Shape::Cylinder(c) => { Object::glass(Shape::Cylinder(*c)) }
            Shape::Cone => { Object::glass(Shape::Cone) }
            _ => { Object::glass(Shape::Object) }
        }
    }
}

#[derive(Clone)]
pub struct Object {
    transformation: Matrix,
    inverse_transformation: Matrix,
    material: Material,
    shape: Shape,
    has_shadow: bool
}

impl Object {
    fn create(shape_type: Shape) -> Object {
        let transform = Matrix::identity();
        Object {
            transformation: transform.clone(),
            inverse_transformation: transform.inverse(),
            material: Material::create(),
            shape: shape_type,
            has_shadow: true
        }
    }

    fn glass(shape_type: Shape) -> Object {
        let transform = Matrix::identity();
        let material = Material::create()
            .with_transparency(1.0)
            .with_refractive_index(1.5);
        Object {
            transformation: transform.clone(),
            inverse_transformation: transform.inverse(),
            material,
            shape: shape_type,
            has_shadow: true
        }
    }

    fn air(shape_type: Shape) -> Object {
        let transform = Matrix::identity();
        let material = Material::create()
            .with_transparency(1.0)
            .with_refractive_index(1.00029);
        Object {
            transformation: transform.clone(),
            inverse_transformation: transform.inverse(),
            material,
            shape: shape_type,
            has_shadow: false
        }
    }

    fn vacuum(shape_type: Shape) -> Object {
        let transform = Matrix::identity();
        let material = Material::create()
            .with_transparency(1.0)
            .with_refractive_index(1.0);
        Object {
            transformation: transform.clone(),
            inverse_transformation: transform.inverse(),
            material,
            shape: shape_type,
            has_shadow: false
        }
    }

    fn water(shape_type: Shape) -> Object {
        let transform = Matrix::identity();
        let material = Material::create()
            .with_transparency(1.0)
            .with_refractive_index(1.33);
        Object {
            transformation: transform.clone(),
            inverse_transformation: transform.inverse(),
            material,
            shape: shape_type,
            has_shadow: false
        }
    }

    fn diamond(shape_type: Shape) -> Object {
        let transform = Matrix::identity();
        let material = Material::create()
            .with_transparency(1.0)
            .with_refractive_index(2.417);
        Object {
            transformation: transform.clone(),
            inverse_transformation: transform.inverse(),
            material,
            shape: shape_type,
            has_shadow: true
        }
    }

    pub fn equals(&self, other: &Object) -> bool {
        self.shape_type() == other.shape_type() &&
            self.material().equals(other.material()) &&
            self.transformation().equals(other.transformation())
    }

    pub fn transformation(&self) -> Matrix {
        self.transformation.clone()
    }

    pub fn inverse_transformation(&self) -> Matrix {
        self.inverse_transformation.clone()
    }

    pub fn material(&self) -> Material {
        self.material.clone()
    }

    pub fn shape_type(&self) -> String {
        String::from(self.shape.as_str())
    }

    pub fn shape(&self) -> Shape {
        self.shape.clone()
    }

    pub fn has_shadow(&self) -> bool {
        self.has_shadow
    }

    pub fn set_transform(&mut self, _transformation: Matrix) {
        self.transformation = _transformation.clone();
        self.inverse_transformation = _transformation.inverse();
    }

    pub fn set_material(&mut self, _material: Material) {
        self.material = _material;
    }

    pub fn with_transform(self, _transform: Matrix) -> Object {
        Object {
            transformation: _transform.clone(),
            inverse_transformation: _transform.inverse(),
            ..self
        }
    }

    pub fn with_material(self, _material: Material) -> Object {
        Object {
            material: _material,
            ..self
        }
    }

    pub fn with_has_shadow(self, _has_shadow: bool) -> Object {
        Object {
            has_shadow: _has_shadow,
            ..self
        }
    }
}

impl Intersect for Object {
    fn intersect(_object: &Object, _ray: &Ray) -> Vec<Intersection> {
        let transformed_ray = _ray.transform(_object.clone().inverse_transformation());
        match _object.shape {
            Shape::Sphere => { Sphere::intersect(_object, &transformed_ray) }
            Shape::Plane => { Plane::intersect(_object, &transformed_ray) }
            Shape::Cube => { Cube::intersect(_object, &transformed_ray) }
            Shape::Cylinder(_) => { Cylinder::intersect(_object, &transformed_ray) }
            Shape::Cone => { Cone::intersect(_object, &transformed_ray) }
            _ => { vec![] }
        }
    }
}

impl Normal for Object {
    fn normal(_object: &Object, _point: &Point) -> Vector {
        let object_point = _object.inverse_transformation() * *_point;
        let object_normal = match _object.shape {
            Shape::Sphere => { Sphere::normal(_object, &object_point) }
            Shape::Plane => { Plane::normal(_object, &object_point) }
            Shape::Cube => { Cube::normal(_object, &object_point) }
            Shape::Cylinder(_) => { Cylinder::normal(_object, &object_point) }
            Shape::Cone => { Cone::normal(_object, &object_point) }
            _ => { Vector::zero() }
        };
        let world_normal = _object.inverse_transformation().transpose() * object_normal;
        world_normal.normalize()
    }
}
