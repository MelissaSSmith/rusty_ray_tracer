use std::iter::Map;
use std::ops::Deref;
use uuid::Uuid;
use crate::features::bounding_box::BoundingBox;
use crate::features::intersection::Intersection;
use crate::features::material::Material;
use crate::features::primitives::matrix::Matrix;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::primitives::vector::Vector;
use crate::features::ray::Ray;
use crate::features::shapes::{Intersect, Normal, NormalAt};
use crate::features::shapes::cone::Cone;
use crate::features::shapes::cube::Cube;
use crate::features::shapes::cylinder::Cylinder;
use crate::features::shapes::group::Group;
use crate::features::shapes::sphere::Sphere;
use crate::features::shapes::plane::Plane;
use crate::features::shapes::test::TestShape;
use crate::features::transformations::Transform;
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
    Group(Group),
    TestShape(TestShape)
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
            Shape::TestShape(_) => "TestShape",
            _ => "Object"
        }
    }

    fn create_object(&self, has_shadow: bool, material: Material) -> Object {
        match self {
            Shape::Sphere => {
                let bounds = BoundingBox::create()
                    .with_minimum(Point::create(-1.0, -1.0, -1.0))
                    .with_maximum(Point::create(1.0, 1.0, 1.0));
                Object::create(Shape::Sphere, has_shadow, material, bounds)
            }
            Shape::Plane => {
                let bounds = BoundingBox::create()
                    .with_minimum(Point::create(-f64::INFINITY, 0.0, -f64::INFINITY))
                    .with_maximum(Point::create(f64::INFINITY, 0.0, f64::INFINITY));
                Object::create(Shape::Plane, has_shadow, material, bounds)
            }
            Shape::Cube => {
                let bounds = BoundingBox::create()
                    .with_minimum(Point::create(-1.0, -1.0, -1.0))
                    .with_maximum(Point::create(1.0, 1.0, 1.0));
                Object::create(Shape::Cube, has_shadow, material, bounds)
            }
            Shape::Cylinder(c) => {
                let bounds = BoundingBox::create()
                    .with_minimum(Point::create(-1.0, c.minimum_bound(), -1.0))
                    .with_maximum(Point::create(1.0, c.maximum_bound(), 1.0));
                Object::create(Shape::Cylinder(*c), has_shadow, material, bounds)
            }
            Shape::Cone(c) => {
                let limit = f64::max(c.minimum_bound().abs(), c.maximum_bound().abs());
                let bounds = BoundingBox::create()
                    .with_minimum(Point::create(-limit, c.minimum_bound(), -limit))
                    .with_maximum(Point::create(limit, c.maximum_bound(), limit));
                Object::create(Shape::Cone(*c), has_shadow, material, bounds)
            }
            Shape::Group(g) => { Object::create(Shape::Group(g.clone()), has_shadow, material,BoundingBox::create()) },
            Shape::TestShape(t) => {
                Object::create(Shape::TestShape(*t), has_shadow, material, BoundingBox::create())
            }
            _ => { Object::create(Shape::Object, has_shadow, material, BoundingBox::create()) }
        }
    }

    pub fn create(&self) -> Object {
        self.create_object(true, Material::create())
    }

    pub fn air(&self) -> Object {
        let material = Material::create()
            .with_transparency(1.0)
            .with_refractive_index(1.00029);
        self.create_object(false, material)
    }

    pub fn vacuum(&self) -> Object {
        let material = Material::create()
            .with_transparency(1.0)
            .with_refractive_index(1.0);
        self.create_object(false, material)
    }

    pub fn water(&self) -> Object {
        let material = Material::create()
            .with_transparency(1.0)
            .with_refractive_index(1.33);
        self.create_object(false, material)
    }

    pub fn diamond(&self) -> Object {
        let material = Material::create()
            .with_transparency(1.0)
            .with_refractive_index(2.417);
        self.create_object(true, material)
    }

    pub fn glass(&self) -> Object {
        let material = Material::create()
            .with_transparency(1.0)
            .with_refractive_index(1.5);
        self.create_object(true, material)
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
    parent: Option<Uuid>,
    bounds: BoundingBox
}

impl Object {
    fn create(shape_type: Shape, has_shadow: bool, material: Material, bounds: BoundingBox) -> Object {
        let transform = Matrix::identity();
        Object {
            id: Uuid::new_v4(),
            transformation: transform.clone(),
            inverse_transformation: transform.inverse(),
            material,
            shape: shape_type,
            has_shadow,
            parent: None,
            bounds
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

    pub fn bounds(&self) -> BoundingBox {
        match self.shape() {
            Shape::Group(g) => {
                let mut bounds = BoundingBox::create();

                for child in g.shapes() {
                    let child_box = child.parent_space_bounds();
                    bounds = bounds + child_box;
                }

                bounds
            }
            _ => { self.bounds }
        }
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

    fn parent_space_bounds(&self) -> BoundingBox {
        self.bounds().transform(self.transformation())
    }

    fn world_to_object(_object: &Object, point: &Point, world: &World) -> Point {
        let object_point = match _object.parent() {
            Some(id) => {
                let parent = world.get_object_by_id(id).expect("Object not found!");
                Object::world_to_object(&parent, &point, world)
            }
            None => *point,
        };

        _object.inverse_transformation() * object_point
    }

    fn normal_to_world(_object: &Object, normal: &Vector, world: &World) -> Vector {
        let world_normal = (_object.inverse_transformation().transpose() * *normal).normalize();

        match _object.parent() {
            None => { world_normal }
            Some(id) => {
                let parent = world.get_object_by_id(id).expect("Object not found!");
                Object::normal_to_world(&parent, &world_normal, world)
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
            Shape::TestShape(_) => { TestShape::intersect(_object, _ray) }
            _ => { vec![] }
        }
    }
}

impl NormalAt for Object {
    fn normal(_object: &Object, _point: &Point, _world: Option<&World>) -> Vector {
        let object_point = match _world {
            None => { _object.inverse_transformation() * *_point }
            Some(w) => { Object::world_to_object(_object, _point, w) }
        };
        let object_normal = match _object.shape {
            Shape::Sphere => { Sphere::normal(_object, &object_point) }
            Shape::Plane => { Plane::normal(_object, &object_point) }
            Shape::Cube => { Cube::normal(_object, &object_point) }
            Shape::Cylinder(_) => { Cylinder::normal(_object, &object_point) }
            Shape::Cone(_) => { Cone::normal(_object, &object_point) }
            Shape::TestShape(_) => { TestShape::normal(_object, &object_point) }
            _ => { Vector::zero() }
        };
        match _world {
            None => {
                (_object.inverse_transformation().transpose() * object_normal).normalize()
            }
            Some(w) => { Object::normal_to_world(_object, &object_normal, w) }
        }
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
    use crate::features::shapes::{Normal, NormalAt};
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

        let point = Object::world_to_object(&object, &Point::create(-2.0, 0.0, -10.0), &world);

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

        let normal = Object::normal_to_world(&object, &Vector::create(sqrt_3, sqrt_3, sqrt_3), &world);

        assert!(normal.equals(Vector::create(0.2857, 0.4286, -0.8571)));
    }

    #[test]
    fn test_find_normal_on_object_in_a_group() {
        let sphere = Shape::Sphere.create()
            .with_transform(Matrix::translate(5.0,0.0, 0.0));
        let mut group_2 = Shape::Group(Group::create()).create()
            .with_transform(Matrix::scale(1.0, 2.0, 3.0));
        let mut group_1 = Shape::Group(Group::create()).create()
            .with_transform(Matrix::rotate_y(PI/2.0));

        group_2.add_child(sphere.clone());
        group_1.add_child(group_2);
        let world = World::create().with_objects(vec![group_1]);

        let object = world.get_object_by_id(sphere.id()).unwrap();

        let point = Object::normal(&object, &Point::create(1.7321, 1.1547, -5.5774), Some(&world));

        assert!(point.equals(Vector::create(0.2857, 0.4286, -0.8571)));
    }

    #[test]
    fn test_querying_a_shapes_bounding_box_in_its_parent_space() {
        let shape = Shape::Sphere.create()
            .with_transform(Matrix::translate(1.0, -3.0, 5.0) * Matrix::scale(0.5, 2.0, 4.0) );

        let bounds = shape.parent_space_bounds();

        assert!(bounds.minimum().equals(Point::create(0.5, -5.0, 1.0)));
        assert!(bounds.maximum().equals(Point::create(1.5, -1.0, 9.0)));
    }
}