use crate::features::intersection::Intersection;
use crate::features::primitives::point::Point;
use crate::features::primitives::vector::Vector;
use crate::features::ray::Ray;
use crate::features::shapes::{Intersect, Normal};
use crate::features::shapes::shape::Object;

#[derive(Clone)]
pub struct Group {
    shapes: Vec<Object>
}

impl Group {
    pub fn create() -> Group {
        Group { shapes: vec![] }
    }

    pub fn shapes(&self) -> Vec<Object> {
        self.shapes.clone()
    }

    pub fn add_child(&self, object: Object) -> Group {
        Group {
            shapes: [self.shapes.clone(), vec![object]].concat()
        }
    }
}

impl Intersect for Group {
    fn intersect(_object: &Object, _ray: &Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = vec![];

        for shape in  _object.shapes() {
            intersections.append(&mut Object::intersect(&shape, _ray));
        }

        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        intersections
    }
}

impl Normal for Group {
    fn normal(_object: &Object, _point: &Point) -> Vector {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::features::primitives::matrix::Matrix;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;
    use crate::features::primitives::vector::Vector;
    use crate::features::ray::Ray;
    use crate::features::shapes::group::Group;
    use crate::features::shapes::Intersect;
    use crate::features::shapes::shape::{Object, Shape};
    use crate::features::shapes::shape::Shape::Sphere;

    #[test]
    fn test_create_a_new_group() {
        let group = Shape::Group(Group::create()).create();

        assert!(group.transformation().equals(Matrix::identity()));
        assert_eq!(group.shapes().len(), 0);
    }

    #[test]
    fn test_shape_has_optional_parent() {
        let shape = Shape::Object.create();

        assert!(shape.parent().is_none());
    }

    #[test]
    fn test_add_a_child_to_a_group() {
        let mut group = Shape::Group(Group::create()).create();
        let mut object = Shape::Object.create();

        group.add_child(object.clone());

        println!("{}", group.shapes().len());
        assert_eq!(group.shapes().len(), 1);
        assert!(group.shapes()[0].equals(&object));
        assert!(group.shapes()[0].parent().is_some() && group.shapes()[0].parent().unwrap().equals(&group));
    }

    #[test]
    fn test_intersecting_a_ray_with_an_empty_group() {
        let group = Shape::Group(Group::create()).create();
        let ray = Ray::create(Point::create(0.0, 0.0, 0.0), Vector::create(0.0, 0.0, 1.0));

        let intersections = Group::intersect(&group, &ray);

        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn test_intersecting_a_ray_with_a_non_empty_group() {
        let mut group = Shape::Group(Group::create()).create();
        group.add_child(Shape::Sphere.create());
        group.add_child(Shape::Sphere.create().with_transform(Matrix::translate(0.0, 0.0, -3.0)));
        group.add_child(Shape::Sphere.create().with_transform(Matrix::translate(5.0, 0.0, 0.0)));
        let ray = Ray::create(Point::create(0.0, 0.0, -5.0), Vector::create(0.0, 0.0, 1.0));

        let intersections = Group::intersect(&group, &ray);

        assert_eq!(intersections.len(), 4);
        assert!(intersections[0].object.equals(&group.shapes()[1]));
        assert!(intersections[1].object.equals(&group.shapes()[1]));
        assert!(intersections[2].object.equals(&group.shapes()[0]));
        assert!(intersections[3].object.equals(&group.shapes()[0]));
    }

    #[test]
    fn test_intersecting_a_transformed_group() {
        let mut group = Shape::Group(Group::create()).create()
            .with_transform(Matrix::scale(2.0, 2.0, 2.0));
        group.add_child(Shape::Sphere.create().with_transform(Matrix::translate(5.0, 0.0, 0.0)));

        let ray = Ray::create(Point::create(10.0, 0.0, -10.0), Vector::create(0.0, 0.0, 1.0));

        let intersections = Object::intersect(&group, &ray);

        println!("{}", intersections.len());
        assert_eq!(intersections.len(), 2);
    }
}