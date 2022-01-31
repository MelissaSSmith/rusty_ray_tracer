use std::iter::Map;
use std::ops::Deref;
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
use crate::features::shapes::group::Group;
use crate::features::shapes::sphere::Sphere;
use crate::features::shapes::plane::Plane;

//todo: derive => Debug, PartialEq, Serialize, Deserialize (requires Matrix not using Vec in backend)
#[derive(Clone)]
pub enum Shape {
    Object,
    Sphere,
    Plane,
    Cube,
    Cylinder(Cylinder),
    Cone(Cone),
    Group(Group)
}

impl Shape {
    fn as_str(&self) -> &'static str {
        match self {
            Shape::Sphere => "Sphere",
            Shape::Plane => "Plane",
            Shape::Cube => "Cube",
            Shape::Cylinder(_) => "Cylinder",
            Shape::Cone(_) => "Cone",
            Shape::Group(_) => "Group",
            _ => "Object"
        }
    }

    pub fn create(&self) -> Object {
        match self {
            Shape::Sphere => { Object::create(Shape::Sphere) }
            Shape::Plane => { Object::create(Shape::Plane) }
            Shape::Cube => { Object::create(Shape::Cube) }
            Shape::Cylinder(c) => { Object::create(Shape::Cylinder(*c)) }
            Shape::Cone(c) => { Object::create(Shape::Cone(*c)) }
            Shape::Group(g) => { Object::create(Shape::Group(g.clone())) }
            _ => { Object::create(Shape::Object) }
        }
    }

    pub fn air(&self) -> Object {
        match self {
            Shape::Sphere => { Object::air(Shape::Sphere) }
            Shape::Plane => { Object::air(Shape::Plane) }
            Shape::Cube => { Object::air(Shape::Cube) }
            Shape::Cylinder(c) => { Object::air(Shape::Cylinder(*c)) }
            Shape::Cone(c) => { Object::air(Shape::Cone(*c)) }
            Shape::Group(g) => { Object::air(Shape::Group(g.clone())) }
            _ => { Object::air(Shape::Object) }
        }
    }

    pub fn vacuum(&self) -> Object {
        match self {
            Shape::Sphere => { Object::vacuum(Shape::Sphere) }
            Shape::Plane => { Object::vacuum(Shape::Plane) }
            Shape::Cube => { Object::vacuum(Shape::Cube) }
            Shape::Cylinder(c) => { Object::vacuum(Shape::Cylinder(*c)) }
            Shape::Cone(c) => { Object::vacuum(Shape::Cone(*c)) }
            Shape::Group(g) => { Object::vacuum(Shape::Group(g.clone())) }
            _ => { Object::vacuum(Shape::Object) }
        }
    }

    pub fn water(&self) -> Object {
        match self {
            Shape::Sphere => { Object::water(Shape::Sphere) }
            Shape::Plane => { Object::water(Shape::Plane) }
            Shape::Cube => { Object::water(Shape::Cube) }
            Shape::Cylinder(c) => { Object::water(Shape::Cylinder(*c)) }
            Shape::Cone(c) => { Object::water(Shape::Cone(*c)) }
            Shape::Group(g) => { Object::water(Shape::Group(g.clone())) }
            _ => { Object::water(Shape::Object) }
        }
    }

    pub fn diamond(&self) -> Object {
        match self {
            Shape::Sphere => { Object::diamond(Shape::Sphere) }
            Shape::Plane => { Object::diamond(Shape::Plane) }
            Shape::Cube => { Object::diamond(Shape::Cube) }
            Shape::Cylinder(c) => { Object::diamond(Shape::Cylinder(*c)) }
            Shape::Cone(c) => { Object::diamond(Shape::Cone(*c)) }
            Shape::Group(g) => { Object::diamond(Shape::Group(g.clone())) }
            _ => { Object::diamond(Shape::Object) }
        }
    }

    pub fn glass(&self) -> Object {
        match self {
            Shape::Sphere => { Object::glass(Shape::Sphere) }
            Shape::Plane => { Object::glass(Shape::Plane) }
            Shape::Cube => { Object::glass(Shape::Cube) }
            Shape::Cylinder(c) => { Object::glass(Shape::Cylinder(*c)) }
            Shape::Cone(c) => { Object::glass(Shape::Cone(*c)) }
            Shape::Group(g) => { Object::glass(Shape::Group(g.clone())) }
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
    has_shadow: bool,
    parent: Box<Option<Object>>
}

impl Object {
    fn create(shape_type: Shape) -> Object {
        let transform = Matrix::identity();
        Object {
            transformation: transform.clone(),
            inverse_transformation: transform.inverse(),
            material: Material::create(),
            shape: shape_type,
            has_shadow: true,
            parent: Box::new(None)
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
            has_shadow: true,
            parent: Box::new(None)
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
            has_shadow: false,
            parent: Box::new(None)
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
            has_shadow: false,
            parent: Box::new(None)
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
            has_shadow: false,
            parent: Box::new(None)
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
            has_shadow: true,
            parent: Box::new(None)
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

    pub fn parent(&self) -> Box<Option<Object>> {
        self.parent.clone()
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

    pub fn maximum_bound(&self) -> f64 {
        match self.shape() {
            Shape::Cylinder(c) => { c.maximum_bound() },
            Shape::Cone(c) => { c.maximum_bound() },
            _ => 0.0
        }
    }

    pub fn minimum_bound(&self) -> f64 {
        match self.shape() {
            Shape::Cylinder(c) => { c.minimum_bound() },
            Shape::Cone(c) => { c.minimum_bound() },
            _ => 0.0
        }
    }

    pub fn closed(&self) -> bool {
        match self.shape() {
            Shape::Cylinder(c) => { c.closed() },
            Shape::Cone(c) => { c.closed() },
            _ => false
        }
    }

    pub fn shapes(&self) -> Vec<Object> {
        match self.shape() {
            Shape::Group(g) => { g.shapes() }
            _ => vec![]
        }
    }

    pub fn add_child(&mut self, object: Object) {
        match self.shape() {
            Shape::Group(g) => {
                let o = Object {
                    parent: Box::new(Some(self.deref().clone())),
                    ..object.clone()
                }.with_transform(
                    self.transformation() * object.transformation()
                );
                let group = g.add_child(o.clone());
                self.shape = Shape::Group(group)
            },
            _ => {}
        }
    }

    pub fn world_to_object(self, point: Point) -> Point {
        let mut object_point = point;

        if self.parent().is_some() {
            println!("Here {}", self.shape_type());
            object_point = self.parent().unwrap().world_to_object(point);
        }

        self.inverse_transformation() * object_point
    }

    pub fn normal_to_world(self, normal: Vector) -> Vector {
        let mut normal = (self.inverse_transformation().transpose() * normal).normalize();

        if self.parent.is_some() {
            normal = self.parent().unwrap().normal_to_world(normal);
        }

        normal
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
            Shape::Cone(_) => { Cone::intersect(_object, &transformed_ray) }
            Shape::Group(_) => { Group::intersect(_object, _ray) }
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
            Shape::Cone(_) => { Cone::normal(_object, &object_point) }
            Shape::Group(_) => { Group::normal(_object, &object_point) }
            _ => { Vector::zero() }
        };
        let world_normal = _object.inverse_transformation().transpose() * object_normal;
        world_normal.normalize()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use crate::features::primitives::matrix::Matrix;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;
    use crate::features::primitives::vector::Vector;
    use crate::features::shapes::group::Group;
    use crate::features::shapes::shape::{Object, Shape};

    #[test]
    fn test_convert_a_point_from_world_object_space() {
        let mut group_1 = Shape::Group(Group::create()).create()
            .with_transform(Matrix::rotate_y(PI/2.0));
        let mut group_2 = Shape::Group(Group::create()).create()
            .with_transform(Matrix::scale(2.0, 2.0, 2.0));
        let sphere = Shape::Sphere.create()
            .with_transform(Matrix::translate(5.0,0.0, 0.0));
        group_2.add_child(sphere);
        group_1.add_child(group_2);

        let object = group_1.shapes()[0].clone().shapes()[0].clone();

        let point = object.world_to_object(Point::create(-2.0, 0.0, -10.0));

        println!("{} {} {}", point.x(), point.y(), point.z());
        assert!(point.equals(Point::create(0.0, 0.0, -1.0)));
    }

    #[test]
    fn test_convert_a_normal_from_object_to_world_space() {
        let mut group_1 = Shape::Group(Group::create()).create()
            .with_transform(Matrix::rotate_y(PI/2.0));
        let mut group_2 = Shape::Group(Group::create()).create()
            .with_transform(Matrix::scale(1.0, 2.0, 3.0));
        let sphere = Shape::Sphere.create()
            .with_transform(Matrix::translate(5.0,0.0, 0.0));
        group_2.add_child(sphere.clone());
        group_1.add_child(group_2);

        let sqrt_3 = f64::sqrt(3.0);

        let normal = sphere.normal_to_world(Vector::create(sqrt_3/3.0, sqrt_3/3.0, sqrt_3/3.0));

        assert!(normal.equals(Vector::create(0.2857, 0.4286, -0.8571)));
    }
}