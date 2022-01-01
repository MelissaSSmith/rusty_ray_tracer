pub mod tuple;
pub mod point;
pub mod vector;
pub mod matrix;
pub mod operations;
pub mod transformations;
pub mod ray;
pub mod shapes;
pub mod intersection;
pub mod color;
pub mod light;
pub mod material;

use std::any::Any;
use crate::features::point::Point;
use crate::features::tuple::Tuple;
use crate::features::vector::Vector;

pub trait Feature {
    fn as_any(&self) -> &dyn Any;
    fn to_tuple(&self) -> (f64, f64, f64, f64);
}

impl Feature for Point {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_tuple(&self) -> (f64, f64, f64, f64) {
        (self.value().x, self.value().y, self.value().z, 1.0)
    }
}

impl Feature for Vector {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_tuple(&self) -> (f64, f64, f64, f64) {
        (self.value().x, self.value().y, self.value().z, 0.0)
    }
}

impl Feature for Tuple {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_tuple(&self) -> (f64, f64, f64, f64) {
        (self.x, self.y, self.z, self.w)
    }
}

pub fn create_feature(tuple:(f64, f64, f64, f64)) -> Box<dyn Feature> {
    match tuple.3 {
        x if x == 0.0 => Box::new(Vector::create(tuple.0, tuple.1, tuple.2)),
        x if x == 1.0 => Box::new(Point::create(tuple.0, tuple.1, tuple.2)),
        _ => Box::new(Tuple::create(tuple.0, tuple.1, tuple.2, tuple.3))
    }
}

#[cfg(test)]
mod tests {
    use crate::features::{create_feature, Feature, Point, Tuple, Vector};

    #[test]
    fn tuple_with_w_equals_1_0_is_a_point() {
        let a = (4.3, -4.2, 3.1, 1.0);

        let feature: Box<dyn Feature> = create_feature(a);

        let point: &Point = match feature.as_any().downcast_ref::<Point>() {
            Some(point) => point,
            None => panic!("&feature isn't a Point!"),
        };

        assert_eq!(a.0, point.value().x);
        assert_eq!(a.1, point.value().y);
        assert_eq!(a.2, point.value().z);
    }

    #[test]
    fn tuple_with_w_equals_0_is_a_vector() {
        let a = (4.3, -4.2, 3.1, 0.0);

        let feature: Box<dyn Feature> = create_feature(a);

        let vector: &Vector = match feature.as_any().downcast_ref::<Vector>() {
            Some(vector) => vector,
            None => panic!("&feature isn't a Vector!"),
        };

        assert_eq!(a.0, vector.value().x);
        assert_eq!(a.1, vector.value().y);
        assert_eq!(a.2, vector.value().z);
    }

    #[test]
    fn test_tuple_with_random_w_is_a_tuple() {
        let a = (4.3, -4.2, 3.1, 6.0);

        let feature: Box<dyn Feature> = create_feature(a);

        let tuple: &Tuple = match feature.as_any().downcast_ref::<Tuple>() {
            Some(tuple) => tuple,
            None => panic!("&feature isn't a Tuple!"),
        };

        assert_eq!(a.0, tuple.x);
        assert_eq!(a.1, tuple.y);
        assert_eq!(a.2, tuple.z);
        assert_eq!(a.3, tuple.w);
    }

    #[test]
    fn test_point_returns_tuple_w_equal_1_0() {
        let point = Point::create(4.0, -4.0, 3.0);

        let tuple = point.to_tuple();

        assert_eq!(point.value().x, tuple.0);
        assert_eq!(point.value().y, tuple.1);
        assert_eq!(point.value().z, tuple.2);
        assert_eq!(1.0, tuple.3);
    }

    #[test]
    fn test_vector_returns_tuple_w_equals_0_0() {
        let vector = Vector::create(4.0,-4.0,3.0);

        let tuple = vector.to_tuple();

        assert_eq!(vector.value().x, tuple.0);
        assert_eq!(vector.value().y, tuple.1);
        assert_eq!(vector.value().z, tuple.2);
        assert_eq!(0.0, tuple.3)
    }

    #[test]
    fn test_tuple_type_return_equivalent_tuple() {
        let tuple_feature = Tuple::create(4.0,-4.0,3.0,6.0);

        let tuple = tuple_feature.to_tuple();

        assert_eq!(tuple_feature.x, tuple.0);
        assert_eq!(tuple_feature.y, tuple.1);
        assert_eq!(tuple_feature.z, tuple.2);
        assert_eq!(tuple_feature.w, tuple.3)
    }
}
