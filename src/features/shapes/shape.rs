use std::ops::Add;
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
use crate::features::shapes::csg::CSG;
use crate::features::shapes::cube::Cube;
use crate::features::shapes::cylinder::Cylinder;
use crate::features::shapes::group::Group;
use crate::features::shapes::sphere::Sphere;
use crate::features::shapes::plane::Plane;
use crate::features::shapes::smooth_triangle::SmoothTriangle;
use crate::features::shapes::triangle::Triangle;
use crate::features::transformations::Transform;
use crate::features::world::World;

#[derive(Clone)]
pub enum Shape {
    Object,
    Sphere,
    Plane,
    Cube,
    Cylinder(Cylinder),
    Cone(Cone),
    Group(Group),
    Triangle(Triangle),
    SmoothTriangle(SmoothTriangle),
    CSG(CSG)
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
            Shape::Triangle(_) => "Triangle",
            Shape::SmoothTriangle(_) => "SmoothTriangle",
            Shape::CSG(_) => "CSG",
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
                    .with_minimum(Point::create(f64::NEG_INFINITY, 0.0, f64::NEG_INFINITY))
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
            Shape::Group(g) => {
                Object::create(Shape::Group(g.clone()), has_shadow, material,BoundingBox::create())
            },
            Shape::Triangle(t) => {
                let bounds = BoundingBox::create() + t.point1() + t.point2() + t.point3();
                Object::create(Shape::Triangle(*t), has_shadow, material, bounds)
            },
            Shape::SmoothTriangle(t) => {
                let bounds = BoundingBox::create() + t.point1() + t.point2() + t.point3();
                Object::create(Shape::SmoothTriangle(*t), has_shadow, material, bounds)
            },
            Shape::CSG(csg) => {
                let bounds = BoundingBox::create() + csg.left().bounds() + csg.right().bounds().transform(csg.left().transformation()).transform(csg.right().transformation());
                Object::create(Shape::CSG(csg.clone()), has_shadow, material, bounds)
            },
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
    cumulative_transform: Matrix,
    cumulative_inverse_transform: Matrix,
    material: Material,
    shape: Shape,
    has_shadow: bool,
    bounds: BoundingBox,
    parent_space_bounds: BoundingBox
}

impl Object {
    fn create(shape_type: Shape, has_shadow: bool, material: Material, bounds: BoundingBox) -> Object {
        let transform = Matrix::identity();
        Object {
            id: Uuid::new_v4(),
            transformation: transform,
            inverse_transformation: transform.inverse(),
            cumulative_transform: transform,
            cumulative_inverse_transform: transform.inverse(),
            material,
            shape: shape_type,
            has_shadow,
            bounds,
            parent_space_bounds: bounds.transform(transform)
        }
    }

    pub fn equals(&self, other: &Object) -> bool {
        self.shape_type() == other.shape_type() &&
            self.material().equals(other.material()) &&
            self.transformation().equals(other.transformation()) &&
            self.cumulative_transform().equals(other.cumulative_transform())
    }

    pub fn id(&self)  -> Uuid {
        self.id
    }

    pub fn transformation(&self) -> Matrix {
        self.transformation
    }

    pub fn inverse_transformation(&self) -> Matrix {
        self.inverse_transformation
    }

    fn cumulative_transform(&self) -> Matrix {
        self.cumulative_transform
    }

    fn inverse_cumulative_transform(&self) -> Matrix {
        self.cumulative_inverse_transform
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

    pub fn bounds(&self) -> BoundingBox {
        self.bounds
    }

    fn parent_space_bounds(&self) -> BoundingBox {
        self.parent_space_bounds
    }

    pub fn has_shadow(&self) -> bool {
        self.has_shadow
    }

    pub fn set_transform(&mut self, _transformation: Matrix) {
        self.cumulative_transform = _transformation;
        self.cumulative_inverse_transform = self.cumulative_transform().inverse();
        self.parent_space_bounds = self.bounds.transform(_transformation);
        match self.shape() {
            Shape::Group(_) => {
                let group = Group::create_with_children(self.children(), self.cumulative_transform());
                self.shape = Shape::Group(group);

            }
            Shape::CSG(csg) => {
                let left = csg.left().with_transform(_transformation);
                let right = csg.right().with_transform(_transformation);
                self.shape = Shape::CSG(CSG::create(csg.operation(), left, right));
            }
            _ => {
                self.transformation = _transformation;
                self.inverse_transformation = _transformation.inverse();
            }
        }
    }

    pub fn set_material(&mut self, _material: Material) {
        self.material = _material;
    }

    pub fn with_transform(self, _transform: Matrix) -> Object {
        match self.shape {
            Shape::CSG(ref c) => {
                let new_csg = CSG::create(c.operation(),
                                          c.left().with_transform(_transform),
                                          c.right().with_transform(_transform));

                Object {
                    shape: Shape::CSG(new_csg),
                    transformation: _transform,
                    inverse_transformation: _transform.inverse(),
                    cumulative_transform: _transform * self.cumulative_transform(),
                    cumulative_inverse_transform: (_transform * self.cumulative_transform()).inverse(),
                    parent_space_bounds: self.bounds.transform(_transform),
                    ..self
                }
            }
            Shape::Group(ref g) => {
                let group = Group::create_with_children(g.shapes(), _transform);

                Object {
                    shape: Shape::Group(group),
                    cumulative_transform: _transform * self.cumulative_transform(),
                    cumulative_inverse_transform: (_transform * self.cumulative_transform()).inverse(),
                    parent_space_bounds: self.bounds.transform(_transform),
                    ..self
                }
            }
            _ => {
                Object {
                    transformation: _transform,
                    inverse_transformation: _transform.inverse(),
                    cumulative_transform: _transform * self.cumulative_transform(),
                    cumulative_inverse_transform: (_transform * self.cumulative_transform()).inverse(),
                    parent_space_bounds: self.bounds.transform(_transform),
                    ..self
                }
            }
        }

    }

    pub fn with_material(self, _material: Material) -> Object {
        Object {
            material: _material,
            ..self
        }
    }

    pub fn with_has_shadow(self, _has_shadow: bool) -> Object {
        match self.shape {
            Shape::CSG(c) => {
                let new_csg = CSG::create(c.operation(), c.left().with_has_shadow(_has_shadow), c.right().with_has_shadow(_has_shadow));

                Object {
                    shape: Shape::CSG(new_csg),
                    has_shadow: _has_shadow,
                    ..self
                }
            }
            _ => {
                Object {
                    has_shadow: _has_shadow,
                    ..self
                }
            }
        }
    }

    pub fn with_children(self, children: Vec<Object>) -> Object {
        match self.shape() {
            Shape::Group(_) => {
                let group = Group::create_with_children(children, self.cumulative_transform());
                let mut group_bounds = BoundingBox::create();
                for child in group.shapes() {
                    let child_box = child.parent_space_bounds();
                    group_bounds = group_bounds + child_box;
                }
                Object {
                    shape: Shape::Group(group),
                    transformation: Matrix::identity(),
                    inverse_transformation: Matrix::identity(),
                    bounds: group_bounds,
                    parent_space_bounds: group_bounds.transform(self.transformation()),
                    ..self
                }
            }
            _ => {
                self.clone()
            }
        }
    }

    fn with_id(self, id: Uuid) -> Object {
        Object {
            id,
            ..self
        }
    }

    //todo: with_left
    //todo: with_right handle transforms on create as well

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

    fn partition_children(&self) -> Vec<Object> {
        match self.shape() {
            Shape::Group(_) => {
                let mut left_children = Vec::with_capacity(self.children().len());
                let mut right_children = Vec::with_capacity(self.children().len());
                let mut children = Vec::with_capacity(self.children().len());

                let (left, right) = self.bounds().split();
                for child in self.children() {
                    if left.contains_box(child.parent_space_bounds()) {
                        left_children.push(child.clone());
                    } else if right.contains_box(child.parent_space_bounds()) {
                        right_children.push(child.clone());
                    } else {
                        children.push(child);
                    }
                }

                if left_children.len() > 0 {
                    children.push(self.make_subgroup(left_children));
                }
                if right_children.len() > 0 {
                    children.push(self.make_subgroup(right_children));
                }

                children
            }
            _ => { self.children() }
        }
    }

    fn make_subgroup(&self, children: Vec<Object>) -> Object {
        Shape::Group(Group::create()).create()
            .with_children(children)
    }

    pub fn divide(&self, threshold: usize) -> Object {
        let mut children = self.children();
        if threshold <= children.len() {
            children = self.partition_children();
        }

        let mut new_children = Vec::with_capacity(children.len());
        for child in children {
            match child.shape() {
                Shape::Group(_) => {
                    new_children.push(child.divide(threshold));
                }
                _ => {
                    new_children.push(child);
                }
            }
        }

        self.clone().with_children(new_children)
    }

    pub(crate) fn includes(&self, other: &Object) -> bool {
        match self.shape() {
            Shape::Group(_) => {
                self.children().iter().any(|child| child.includes(other))
            }
            Shape::CSG(c) => {
                c.left().includes(other) || c.right().includes(other)
            },
            _ => { self.equals(other) }
        }
    }

    fn world_to_object(_object: &Object, point: &Point) -> Point {
        _object.inverse_cumulative_transform() * *point
    }

    fn normal_to_world(_object: &Object, normal: &Vector) -> Vector {
        let world_normal = (_object.inverse_cumulative_transform().transpose() * *normal).normalize();

        Vector::create(world_normal.x(), world_normal.y(), world_normal.z()).normalize()
    }
}

impl Intersect for Object {
    fn intersect(_object: &Object, _ray: &Ray) -> Vec<Intersection> {
        let transformed_ray = _ray.transform(_object.inverse_transformation());
        match _object.shape() {
            Shape::Sphere => { Sphere::intersect(_object, &transformed_ray) }
            Shape::Plane => { Plane::intersect(_object, &transformed_ray) }
            Shape::Cube => { Cube::intersect(_object, &transformed_ray) }
            Shape::Cylinder(_) => { Cylinder::intersect(_object, &transformed_ray) }
            Shape::Cone(_) => { Cone::intersect(_object, &transformed_ray) }
            Shape::Group(_) => { Group::intersect(_object, _ray) }
            Shape::Triangle(_) => { Triangle::intersect(_object, _ray) }
            Shape::SmoothTriangle(_) => { SmoothTriangle::intersect(_object, _ray) }
            Shape::CSG(csg) => { csg.intersect(_ray, _object) }
            _ => { vec![] }
        }
    }
}

impl NormalAt for Object {
    fn normal(_object: &Object, _point: &Point, _hit: &Intersection) -> Vector {
        let object_point = Object::world_to_object(_object, _point);

        let object_normal = match _object.shape {
            Shape::Sphere => { Sphere::normal(_object, &object_point) }
            Shape::Plane => { Plane::normal(_object, &object_point) }
            Shape::Cube => { Cube::normal(_object, &object_point) }
            Shape::Cylinder(_) => { Cylinder::normal(_object, &object_point) }
            Shape::Cone(_) => { Cone::normal(_object, &object_point) }
            Shape::Triangle(t) => { t.normal_vector() }
            Shape::SmoothTriangle(_) => { SmoothTriangle::normal(_object, &object_point, _hit) }
            _ => { panic!("{} has no normal", _object.shape_type()); }
        };

        Object::normal_to_world(_object, &object_normal)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use crate::features::intersection::Intersection;
    use crate::features::primitives::matrix::Matrix;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;
    use crate::features::primitives::vector::Vector;
    use crate::features::shapes::group::Group;
    use crate::features::shapes::{NormalAt};
    use crate::features::shapes::shape::{Object, Shape};
    use crate::features::world::World;

    #[test]
    fn test_convert_a_point_from_world_object_space() {
        let sphere = Shape::Sphere.create()
            .with_transform(Matrix::translate(5.0,0.0, 0.0));
        let group_2 = Shape::Group(Group::create()).create()
            .with_transform(Matrix::scale(2.0, 2.0, 2.0))
            .with_children(vec![sphere.clone()]);
        let mut group_1 = Shape::Group(Group::create()).create()
            .with_transform(Matrix::rotate_y(PI/2.0))
            .with_children(vec![group_2]);
        group_1.set_transform(Matrix::identity());

        let sphere = group_1.children()[0].clone().children()[0].clone();
        println!("Shape: {}", sphere.shape_type());

        let point = Object::world_to_object(&sphere, &Point::create(-2.0, 0.0, -10.0));

        println!("Point: {}", point);
        assert!(point.equals(Point::create(0.0, 0.0, -1.0)));
    }

    #[test]
    fn test_convert_a_normal_from_object_to_world_space() {
        let sphere = Shape::Sphere.create()
            .with_transform(Matrix::translate(5.0,0.0, 0.0));
        let group_2 = Shape::Group(Group::create()).create()
            .with_transform(Matrix::scale(1.0, 2.0, 3.0))
            .with_children(vec![sphere.clone()]);
        let group_1 = Shape::Group(Group::create()).create()
            .with_transform(Matrix::rotate_y(PI/2.0))
            .with_children(vec![group_2]);

        let object = group_1.children()[0].clone().children()[0].clone();

        let sqrt_3 = f64::sqrt(3.0)/3.0;

        let normal = Object::normal_to_world(&object, &Vector::create(sqrt_3, sqrt_3, sqrt_3));

        assert!(normal.equals(Vector::create(0.2857, 0.4286, -0.8571)));
    }

    #[test]
    fn test_find_normal_on_object_in_a_group() {
        let sphere = Shape::Sphere.create()
            .with_transform(Matrix::translate(5.0,0.0, 0.0));
        let group_2 = Shape::Group(Group::create()).create()
            .with_transform(Matrix::scale(1.0, 2.0, 3.0))
            .with_children(vec![sphere.clone()]);
        let group_1 = Shape::Group(Group::create()).create()
            .with_transform(Matrix::rotate_y(PI/2.0))
            .with_children(vec![group_2]);

        let object = group_1.children()[0].clone().children()[0].clone();
        let intersection = Intersection::create(0.0, &object, 0.0, 0.0);

        let point = Object::normal(&object, &Point::create(1.7321, 1.1547, -5.5774), &intersection);

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

    #[test]
    fn test_subdividing_a_primitive_does_nothing() {
        let mut shape = Shape::Sphere.create();

        let divided_object = shape.divide(1);

        assert!(divided_object.equals(&shape));
    }

    #[test]
    fn test_create_subgroup_from_list_of_children() {
        let s1 = Shape::Sphere.create();
        let s2 = Shape::Sphere.create();
        let group = Shape::Group(Group::create()).create();

        let new_group = group.make_subgroup(vec![s1, s2]);

        assert_eq!(new_group.children().len(), 2);
    }

    #[test]
    fn test_subdividing_a_group_partitions_its_children() {
        let s1 = Shape::Sphere.create()
            .with_transform(Matrix::translate(-2.0, -2.0, 0.0));
        let s2 = Shape::Sphere.create()
            .with_transform(Matrix::translate(-2.0, 2.0, 0.0));
        let s3 = Shape::Sphere.create()
            .with_transform(Matrix::scale(4.0, 4.0, 4.0));

        let group = Shape::Group(Group::create()).create()
            .with_children(vec![s1.clone(), s2.clone(), s3.clone()]);

        let divided_group = group.divide(1);

        assert_eq!(2, divided_group.children().len());
        assert!(divided_group.children()[0].equals(&s3));

        assert_eq!(divided_group.children()[1].shape_type(), "Group");
        assert_eq!(divided_group.children()[1].children()[0].shape_type(), "Group");
        assert_eq!(divided_group.children()[1].children()[1].shape_type(), "Group");
        assert!(divided_group.children()[1].children()[0].children()[0].equals(&s1));
        assert!(divided_group.children()[1].children()[1].children()[0].equals(&s2));
    }

    #[test]
    fn test_subdividing_a_group_with_too_few_children() {
        let s1 = Shape::Sphere.create()
            .with_transform(Matrix::translate(-2.0, 0.0, 0.0));
        let s2 = Shape::Sphere.create()
            .with_transform(Matrix::translate(2.0, 1.0, 0.0));
        let s3 = Shape::Sphere.create()
            .with_transform(Matrix::translate(2.0, -1.0, 0.0));
        let s4 = Shape::Sphere.create();
        let subgroup = Shape::Group(Group::create()).create()
            .with_children(vec![s1.clone(), s2.clone(), s3.clone()]);
        let group = Shape::Group(Group::create()).create()
            .with_children(vec![subgroup.clone(), s4.clone()]);

        let divided_group = group.divide(3);

        assert_eq!(2, divided_group.children().len());
        assert!(divided_group.children()[0].equals(&subgroup));
        assert!(divided_group.children()[1].equals(&s4));

        assert_eq!(2, divided_group.children()[0].children().len());
        assert!(divided_group.children()[0].children()[0].children()[0].equals(&s1));
        assert!(divided_group.children()[0].children()[1].children()[0].equals(&s2));
        assert!(divided_group.children()[0].children()[1].children()[1].equals(&s3));
    }
}