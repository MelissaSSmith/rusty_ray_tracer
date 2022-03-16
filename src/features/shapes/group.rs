use uuid::Uuid;
use crate::features::bounding_box::BoundingBox;
use crate::features::intersection::Intersection;
use crate::features::primitives::matrix::Matrix;
use crate::features::ray::Ray;
use crate::features::shapes::Intersect;
use crate::features::shapes::shape::Object;

#[derive(Clone)]
pub struct Group {
    children: Vec<Object>
}

impl Group {
    pub fn create() -> Group {
        Group { children: vec![] }
    }

    pub fn create_with_children(children: Vec<Object>, transform: Matrix) -> Group {
        Group {
            children: Group::transform_children(children, transform)
        }
    }

    pub fn shapes(&self) -> Vec<Object> {
        self.children.clone()
    }

    fn transform_children(children: Vec<Object>, transform: Matrix) -> Vec<Object> {
        let mut transformed_children = vec![];
        for mut child in children {
            child.set_transform(transform * child.transformation());
            transformed_children.push(child);
        }

        transformed_children
    }
}

impl Intersect for Group {
    fn intersect(_object: &Object, _ray: &Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = vec![];

        if BoundingBox::intersects(&_object.bounds(), _ray) {
            for shape in  _object.children() {
                intersections.append(&mut Object::intersect(&shape, _ray));
            }
        }

        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        intersections
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use crate::features::primitives::matrix::Matrix;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;
    use crate::features::primitives::vector::Vector;
    use crate::features::ray::Ray;
    use crate::features::shapes::cylinder::Cylinder;
    use crate::features::shapes::group::Group;
    use crate::features::shapes::Intersect;
    use crate::features::shapes::shape::{Object, Shape};

    #[test]
    fn test_create_a_new_group() {
        let group = Shape::Group(Group::create()).create();

        assert!(group.transformation().equals(Matrix::identity()));
        assert_eq!(group.children().len(), 0);
    }

    #[test]
    fn test_add_a_child_to_a_group() {
        let object = Shape::Object.create();

        let group = Shape::Group(Group::create()).create()
            .with_children(vec![object.clone()]);

        assert_eq!(group.children().len(), 1);
        assert!(group.children()[0].equals(&object));
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
        let group = Shape::Group(Group::create()).create()
            .with_children(
                vec![
                    Shape::Sphere.create(),
                    Shape::Sphere.create().with_transform(Matrix::translate(0.0, 0.0, -3.0)),
                    Shape::Sphere.create().with_transform(Matrix::translate(5.0, 0.0, 0.0))
                ]
            );
        let ray = Ray::create(Point::create(0.0, 0.0, -5.0), Vector::create(0.0, 0.0, 1.0));

        let intersections = Group::intersect(&group, &ray);

        assert_eq!(intersections.len(), 4);
        assert!(intersections[0].object.equals(&group.children()[1]));
        assert!(intersections[1].object.equals(&group.children()[1]));
        assert!(intersections[2].object.equals(&group.children()[0]));
        assert!(intersections[3].object.equals(&group.children()[0]));
    }

    #[test]
    fn test_intersecting_a_transformed_group() {
        let group = Shape::Group(Group::create()).create()
            .with_transform(Matrix::scale(2.0, 2.0, 2.0))
            .with_children(vec![Shape::Sphere.create().with_transform(Matrix::translate(5.0, 0.0, 0.0))]);

        let ray = Ray::create(Point::create(10.0, 0.0, -10.0), Vector::create(0.0, 0.0, 1.0));

        let intersections = Object::intersect(&group, &ray);

        assert_eq!(intersections.len(), 2);
    }

    #[test]
    fn test_group_has_a_bounding_box_that_contains_its_children() {
        let sphere = Shape::Sphere.create()
            .with_transform(Matrix::translate(2.0, 5.0, -3.0) * Matrix::scale(2.0, 2.0, 2.0));
        let cylinder = Shape::Cylinder(
            Cylinder::create()
                .with_minimum_bound(-2.0)
                .with_maximum_bound(2.0)
        ).create()
            .with_transform(Matrix::translate(-4.0, -1.0, 4.0) * Matrix::scale(0.5, 1.0, 0.5));
        let group = Shape::Group(Group::create()).create()
            .with_children(vec![sphere, cylinder]);

        let bounds = group.bounds();

        assert!(bounds.minimum().equals(Point::create(-4.5, -3.0, -5.0)));
        assert!(bounds.maximum().equals(Point::create(4.0, 7.0, 4.5)));
    }

    #[test]
    fn test_intersecting_group_does_not_test_children_if_box_is_missed() {
        let child = Shape::Sphere.create();
        let group = Shape::Group(Group::create()).create()
            .with_children(vec![child]);
        let ray = Ray::create(Point::create(0.0, 0.0, -5.0), Vector::create(0.0, 1.0, 0.0));

        let intersections = Object::intersect(&group, &ray);

        assert_eq!(0, intersections.len());
    }

    #[test]
    fn test_intersecting_group_tests_children_if_box_is_hit() {
        let child = Shape::Sphere.create();
        let group = Shape::Group(Group::create()).create()
            .with_children(vec![child]);
        let ray = Ray::create(Point::create(0.0, 0.0, -5.0), Vector::create(0.0, 0.0, 1.0));

        let intersections = Object::intersect(&group, &ray);

        assert_eq!(true, intersections.len() > 0);
    }

    // #[test]
    // fn test_partitioning_a_groups_children() {
    //     let s1 = Shape::Sphere.create()
    //         .with_transform(Matrix::translate(-2.0, 0.0, 0.0));
    //     let s2 = Shape::Sphere.create()
    //         .with_transform(Matrix::translate(2.0, 0.0, 0.0));
    //     let s3 = Shape::Sphere.create();
    //
    //     let group = Group::create_with_children(vec![s1.clone(), s2.clone(), s3.clone()], Matrix::identity(), Uuid::new_v4());
    //
    //     let (left, right) = group.partition_children();
    //
    //     assert!(group.shapes()[0].equals(&s3));
    //     assert!(left[0].equals(&s1));
    //     assert!(right[0].equals(&s2));
    // }
}