use crate::features::intersection::Intersection;
use crate::features::material::Material;
use crate::features::primitives::matrix::Matrix;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::primitives::vector::Vector;
use crate::features::ray::Ray;
use crate::features::shapes::{Intersect, Normal};
use crate::features::shapes::cube::Cube;
use crate::features::shapes::cylinder::Cylinder;
use crate::features::shapes::sphere::Sphere;
use crate::features::shapes::plane::Plane;

//todo: derive => Debug, PartialEq, Serialize, Deserialize (requires Matrix not using Vec in backend)
#[derive(Clone)]
pub enum Shape {
    Object,
    Sphere,
    Plane,
    Cube,
    Cylinder
}

impl Shape {
    fn as_str(&self) -> &'static str {
        match self {
            Shape::Sphere => "Sphere",
            Shape::Plane => "Plane",
            Shape::Cube => "Cube",
            Shape::Cylinder => "Cylinder",
            _ => "Object"
        }
    }

    pub fn create(&self) -> Object {
        match self {
            Shape::Sphere => { Object::create(String::from(Shape::Sphere.as_str())) }
            Shape::Plane => { Object::create(String::from(Shape::Plane.as_str())) }
            Shape::Cube => { Object::create(String::from(Shape::Cube.as_str())) }
            Shape::Cylinder => { Object::create(String::from(Shape::Cylinder.as_str())) }
            _ => { Object::create(String::from(Shape::Object.as_str())) }
        }
    }

    pub fn air(&self) -> Object {
        match self {
            Shape::Sphere => { Object::air(String::from(Shape::Sphere.as_str())) }
            Shape::Plane => { Object::air(String::from(Shape::Plane.as_str())) }
            Shape::Cube => { Object::air(String::from(Shape::Cube.as_str())) }
            Shape::Cylinder => { Object::air(String::from(Shape::Cylinder.as_str())) }
            _ => { Object::air(String::from(Shape::Object.as_str())) }
        }
    }

    pub fn vacuum(&self) -> Object {
        match self {
            Shape::Sphere => { Object::vacuum(String::from(Shape::Sphere.as_str())) }
            Shape::Plane => { Object::vacuum(String::from(Shape::Plane.as_str())) }
            Shape::Cube => { Object::vacuum(String::from(Shape::Cube.as_str())) }
            Shape::Cylinder => { Object::vacuum(String::from(Shape::Cylinder.as_str())) }
            _ => { Object::vacuum(String::from(Shape::Object.as_str())) }
        }
    }

    pub fn water(&self) -> Object {
        match self {
            Shape::Sphere => { Object::water(String::from(Shape::Sphere.as_str())) }
            Shape::Plane => { Object::water(String::from(Shape::Plane.as_str())) }
            Shape::Cube => { Object::water(String::from(Shape::Cube.as_str())) }
            Shape::Cylinder => { Object::water(String::from(Shape::Cylinder.as_str())) }
            _ => { Object::water(String::from(Shape::Object.as_str())) }
        }
    }

    pub fn diamond(&self) -> Object {
        match self {
            Shape::Sphere => { Object::diamond(String::from(Shape::Sphere.as_str())) }
            Shape::Plane => { Object::diamond(String::from(Shape::Plane.as_str())) }
            Shape::Cube => { Object::diamond(String::from(Shape::Cube.as_str())) }
            Shape::Cylinder => { Object::diamond(String::from(Shape::Cylinder.as_str())) }
            _ => { Object::diamond(String::from(Shape::Object.as_str())) }
        }
    }

    pub fn glass(&self) -> Object {
        match self {
            Shape::Sphere => { Object::glass(String::from(Shape::Sphere.as_str())) }
            Shape::Plane => { Object::glass(String::from(Shape::Plane.as_str())) }
            Shape::Cube => { Object::glass(String::from(Shape::Cube.as_str())) }
            Shape::Cylinder => { Object::glass(String::from(Shape::Cylinder.as_str())) }
            _ => { Object::glass(String::from(Shape::Object.as_str())) }
        }
    }
}

#[derive(Clone)]
pub struct Object {
    transformation: Matrix,
    inverse_transformation: Matrix,
    material: Material,
    shape: String,
    has_shadow: bool
}

impl Object {
    fn create(shape_type: String) -> Object {
        let transform = Matrix::identity();
        Object {
            transformation: transform.clone(),
            inverse_transformation: transform.inverse(),
            material: Material::create(),
            shape: shape_type,
            has_shadow: true
        }
    }

    fn glass(shape_type: String) -> Object {
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

    fn air(shape_type: String) -> Object {
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

    fn vacuum(shape_type: String) -> Object {
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

    fn water(shape_type: String) -> Object {
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

    fn diamond(shape_type: String) -> Object {
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
        self.shape() == other.shape() &&
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

    pub fn shape(&self) -> String {
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
        match _object.shape.as_ref() {
            "Sphere" => { Sphere::intersect(_object, &transformed_ray) }
            "Plane" => { Plane::intersect(_object, &transformed_ray) }
            "Cube" => { Cube::intersect(_object, &transformed_ray) }
            "Cylinder" => { Cylinder::intersect(_object, &transformed_ray) }
            _ => { vec![] }
        }
    }
}

impl Normal for Object {
    fn normal(_object: &Object, _point: &Point) -> Vector {
        let object_point = _object.inverse_transformation() * *_point;
        let object_normal = match _object.shape.as_ref() {
            "Sphere" => { Sphere::normal(_object, &object_point) }
            "Plane" => { Plane::normal(_object, &object_point) }
            "Cube" => { Cube::normal(_object, &object_point) }
            "Cylinder" => { Cylinder::normal(_object, &object_point) }
            _ => { Vector::zero() }
        };
        let world_normal = _object.inverse_transformation().transpose() * object_normal;
        world_normal.normalize()
    }
}
