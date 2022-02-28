use std::ops::Deref;
use crate::features::shapes::shape::Object;

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

    fn left(&self) -> Object {
        self.left.deref().clone()
    }

    fn right(&self) -> Object {
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
}

#[cfg(test)]
mod tests {
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
}

