use std::ops::Deref;
use crate::features::intersection::Intersection;
use crate::features::ray::Ray;
use crate::features::shapes::Intersect;
use crate::features::shapes::shape::{Object, Shape};

#[derive(Clone, Copy, PartialOrd, PartialEq, Debug)]
enum CSGOperation {
    Union,
    Difference,
    Intersection
}

#[derive(Clone)]
pub struct CSG {
    operation: CSGOperation,
    left: Box<Object>,
    right: Box<Object>
}

impl CSG {
    fn create(operation: CSGOperation, left: Object, right: Object) -> CSG {
        CSG {
            operation,
            left: Box::new(left),
            right: Box::new(right)
        }
    }

    fn operation(&self) -> CSGOperation {
        self.operation.clone()
    }

    pub(crate) fn left(&self) -> Object {
        self.left.deref().clone()
    }

    pub(crate) fn right(&self) -> Object {
        self.right.deref().clone()
    }

    fn intersection_allowed(&self, operation: CSGOperation, left_hit: bool, inside_left: bool, inside_right: bool) -> bool {
        match operation {
            CSGOperation::Union => {
                (left_hit && !inside_right) || (!left_hit && !inside_left)
            }
            CSGOperation::Difference => {
                (left_hit && !inside_right) || (!left_hit && inside_left)
            }
            CSGOperation::Intersection => {
                (left_hit && inside_right) || (!left_hit && inside_left)
            }
        }
    }

    fn filter_intersections(&self, intersections: Vec<Intersection>) -> Vec<Intersection> {
        let mut inl = false;
        let mut inr = false;

        let mut result = Vec::<Intersection>::new();

        for intersection in intersections {
            let lhit = self.left.includes(&intersection.object());

            if self.intersection_allowed(self.operation(), lhit, inl, inr) {
                result.push(intersection);
            }

            if lhit {
                inl = !inl;
            } else {
                inr = !inr;
            }
        }

        result
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut left_intersections = Object::intersect(&self.left(), ray);
        let mut right_intersections = Object::intersect(&self.right(), ray);

        left_intersections.append(&mut right_intersections);

        left_intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        self.filter_intersections(left_intersections)
    }
}

#[cfg(test)]
mod tests {
    use crate::features::intersection::Intersection;
    use crate::features::primitives::matrix::Matrix;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;
    use crate::features::primitives::vector::Vector;
    use crate::features::ray::Ray;
    use crate::features::shapes::csg::{CSG, CSGOperation};
    use crate::features::shapes::shape::Shape;

    #[test]
    fn test_csg_created_with_an_operation_and_two_shapes() {
        let s1 = Shape::Sphere.create();
        let s2 = Shape::Cube.create();

        let csg = CSG::create(CSGOperation::Union, s1.clone(), s2.clone());

        assert_eq!(CSGOperation::Union, csg.operation);
        assert_eq!(csg.left().shape_type(), s1.shape_type());
        assert_eq!(csg.right().shape_type(), s2.shape_type());
    }

    #[test]
    fn test_evaluate_rule_for_a_csg_operation() {
        let s1 = Shape::Sphere.create();
        let s2 = Shape::Cube.create();
        let csg = CSG::create(CSGOperation::Union, s1.clone(), s2.clone());

        let tests = vec![
            (CSGOperation::Union, true, true, true, false),
            (CSGOperation::Union, true, true, false, true),
            (CSGOperation::Union, true, false, true, false),
            (CSGOperation::Union, true, false, false, true),
            (CSGOperation::Union, false, true, true, false),
            (CSGOperation::Union, false, true, false, false),
            (CSGOperation::Union, false, false, true, true),
            (CSGOperation::Union, false, false, false, true),
            (CSGOperation::Intersection, true, true, true, true),
            (CSGOperation::Intersection, true, true, false, false),
            (CSGOperation::Intersection, true, false, true, true),
            (CSGOperation::Intersection, true, false, false, false),
            (CSGOperation::Intersection, false, true, true, true),
            (CSGOperation::Intersection, false, true, false, true),
            (CSGOperation::Intersection, false, false, true, false),
            (CSGOperation::Intersection, false, false, false, false),
            (CSGOperation::Difference, true, true, true, false),
            (CSGOperation::Difference, true, true, false, true),
            (CSGOperation::Difference, true, false, true, false),
            (CSGOperation::Difference, true, false, false, true),
            (CSGOperation::Difference, false, true, true, true),
            (CSGOperation::Difference, false, true, false, true),
            (CSGOperation::Difference, false, false, true, false),
            (CSGOperation::Difference, false, false, false, false)
        ];

        for test in tests {
            let result = csg.intersection_allowed(test.0, test.1, test.2, test.3);

            assert_eq!(result, test.4);
        }
    }

    #[test]
    fn test_filtering_a_list_of_intersections() {
        let sphere = Shape::Sphere.create();
        let cube = Shape::Cube.create();

        let tests: Vec<(CSGOperation, usize, usize)> = vec![
            (CSGOperation::Union, 0, 3),
            (CSGOperation::Intersection, 1, 2),
            (CSGOperation::Difference, 0, 1)
        ];

        for test in tests {
            let csg = CSG::create(test.0, sphere.clone(), cube.clone());
            let intersections = vec![
                Intersection::create(1.0, &sphere, 0.0, 0.0),
                Intersection::create(2.0, &cube, 0.0, 0.0),
                Intersection::create(3.0, &sphere, 0.0, 0.0),
                Intersection::create(4.0, &cube, 0.0, 0.0)
            ];

            let result = csg.filter_intersections(intersections.clone());

            assert_eq!(2, result.len());
            assert!(result[0].equals(&intersections[test.1]));
            assert!(result[1].equals(&intersections[test.2]));
        }
    }

    #[test]
    fn test_ray_misses_a_csg_object() {
        let csg = CSG::create(CSGOperation::Union, Shape::Sphere.create(), Shape::Cube.create());
        let ray = Ray::create(Point::create(0.0, 2.0, -5.0), Vector::create(0.0, 0.0, 1.0));

        let intersections = csg.intersect(&ray);

        assert_eq!(0, intersections.len());
    }

    #[test]
    fn test_ray_hits_a_csg_object() {
        let s1 = Shape::Sphere.create();
        let s2 = Shape::Sphere.create()
            .with_transform(Matrix::translate(0.0, 0.0, 0.5));
        let csg = CSG::create(CSGOperation::Union, s1.clone(), s2.clone());
        let ray = Ray::create(Point::create(0.0, 0.0, -5.0), Vector::create(0.0, 0.0, 1.0));

        let intersections = csg.intersect(&ray);

        assert_eq!(2, intersections.len());
        assert_eq!(intersections[0].t, 4.0);
        assert!(intersections[0].object().equals(&s1));
        assert_eq!(intersections[1].t, 6.5);
        assert!(intersections[1].object().equals(&s2));
    }
}

