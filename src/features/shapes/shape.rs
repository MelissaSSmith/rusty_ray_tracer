use std::iter::Map;
use std::ops::Deref;
use uuid::Uuid;
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
use crate::features::world::World;

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
    id: Uuid,
    transformation: Matrix,
    inverse_transformation: Matrix,
    material: Material,
    shape: Shape,
    has_shadow: bool,
    parent: Option<Uuid>
}

impl Object {
    fn create(shape_type: Shape) -> Object {
        let transform = Matrix::identity();
        Object {
            id: Uuid::new_v4(),
            transformation: transform.clone(),
            inverse_transformation: transform.inverse(),
            material: Material::create(),
            shape: shape_type,
            has_shadow: true,
            parent: None
        }
    }

    fn glass(shape_type: Shape) -> Object {
        let transform = Matrix::identity();
        let material = Material::create()
            .with_transparency(1.0)
            .with_refractive_index(1.5);
        Object {
            id: Uuid::new_v4(),
            transformation: transform.clone(),
            inverse_transformation: transform.inverse(),
            material,
            shape: shape_type,
            has_shadow: true,
            parent: None
        }
    }

    fn air(shape_type: Shape) -> Object {
        let transform = Matrix::identity();
        let material = Material::create()
            .with_transparency(1.0)
            .with_refractive_index(1.00029);
        Object {
            id: Uuid::new_v4(),
            transformation: transform.clone(),
            inverse_transformation: transform.inverse(),
            material,
            shape: shape_type,
            has_shadow: false,
            parent: None
        }
    }

    fn vacuum(shape_type: Shape) -> Object {
        let transform = Matrix::identity();
        let material = Material::create()
            .with_transparency(1.0)
            .with_refractive_index(1.0);
        Object {
            id: Uuid::new_v4(),
            transformation: transform.clone(),
            inverse_transformation: transform.inverse(),
            material,
            shape: shape_type,
            has_shadow: false,
            parent: None
        }
    }

    fn water(shape_type: Shape) -> Object {
        let transform = Matrix::identity();
        let material = Material::create()
            .with_transparency(1.0)
            .with_refractive_index(1.33);
        Object {
            id: Uuid::new_v4(),
            transformation: transform.clone(),
            inverse_transformation: transform.inverse(),
            material,
            shape: shape_type,
            has_shadow: false,
            parent: None
        }
    }

    fn diamond(shape_type: Shape) -> Object {
        let transform = Matrix::identity();
        let material = Material::create()
            .with_transparency(1.0)
            .with_refractive_index(2.417);
        Object {
            id: Uuid::new_v4(),
            transformation: transform.clone(),
            inverse_transformation: transform.inverse(),
            material,
            shape: shape_type,
            has_shadow: true,
            parent: None
        }
    }

    pub fn equals(&self, other: &Object) -> bool {
        self.shape_type() == other.shape_type() &&
            self.material().equals(other.material()) &&
            self.transformation().equals(other.transformation())
    }

    pub fn id(&self)  -> Uuid {
        self.id
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

    pub fn parent(&self) -> Option<Uuid> {
        self.parent
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

    fn set_parent(&mut self, object: Object) {
        self.parent = Some(object.id())
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

    pub fn children(&self) -> Vec<Object> {
        match self.shape() {
            Shape::Group(g) => { g.shapes() }
            _ => vec![]
        }
    }

    pub fn add_child(&mut self, mut object: Object) {
        match self.shape() {
            Shape::Group(mut g) => {
                object.set_parent(self.clone());
                g.add_child(object);

                self.shape = Shape::Group(g)
            },
            _ => {}
        }
    }

    pub fn get_object_by_id(&self, id:Uuid) -> Option<Object> {
        match self.shape() {
            Shape::Group(g) => { g.get_object_by_id(id) }
            _ => { None }
        }
    }

    pub fn world_to_object(self, point: Point, world: &World) -> Point {
        let object_point = match self.parent() {
            Some(id) => {
                let parent = world.get_object_by_id(id).expect("Object not found!");
                parent.world_to_object(point, world)
            }
            None => point,
        };

        self.inverse_transformation() * object_point
    }

    pub fn normal_to_world(self, normal: Vector, world: &World) -> Vector {
        let world_normal = (self.inverse_transformation().transpose() * normal).normalize();

        match self.parent() {
            None => { world_normal }
            Some(id) => {
                let parent = world.get_object_by_id(id).expect("Object not found!");
                parent.normal_to_world(world_normal, world)
            }
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
    use crate::features::world::World;

    #[test]
    fn test_convert_a_point_from_world_object_space() {
        let sphere = Shape::Sphere.create()
            .with_transform(Matrix::translate(5.0,0.0, 0.0));
        let mut group_2 = Shape::Group(Group::create()).create()
            .with_transform(Matrix::scale(2.0, 2.0, 2.0));
        let mut group_1 = Shape::Group(Group::create()).create()
            .with_transform(Matrix::rotate_y(PI/2.0));

        group_2.add_child(sphere.clone());
        group_1.add_child(group_2);
        let world = World::create().with_objects(vec![group_1]);

        let object = world.get_object_by_id(sphere.id()).unwrap();

        let point = object.world_to_object(Point::create(-2.0, 0.0, -10.0), &world);

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
        let world = World::create().with_objects(vec![group_1]);

        let object = world.get_object_by_id(sphere.id()).unwrap();

        let sqrt_3 = f64::sqrt(3.0)/3.0;

        let normal = object.normal_to_world(Vector::create(sqrt_3, sqrt_3, sqrt_3), &world);

        assert!(normal.equals(Vector::create(0.2857, 0.4286, -0.8571)));
    }
}