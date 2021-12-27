use std::any::Any;

pub trait Feature {
    fn as_any(&self) -> &dyn Any;
    fn to_tuple(&self) -> (f64, f64, f64, f64);
}

pub struct Point {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64
}

impl Feature for Point {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_tuple(&self) -> (f64, f64, f64, f64) {
        (self.x, self.y, self.z, 1.0)
    }
}

pub struct Vector {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64
}

impl Feature for Vector {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_tuple(&self) -> (f64, f64, f64, f64) {
        (self.x, self.y, self.z, 0.0)
    }
}

impl Feature for () {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_tuple(&self) -> (f64, f64, f64, f64) {
        (0.0, 0.0, 0.0, 0.0)
    }
}

pub fn create_feature(tuple:(f64, f64, f64, f64)) -> Box<dyn Feature> {
    match tuple.3 {
        x if x == 0.0 => Box::new(Vector{x: tuple.0, y:tuple.1, z:tuple.2}),
        x if x == 1.0 => Box::new(Point{x: tuple.0, y:tuple.1, z:tuple.2}),
        _ => Box::new({})
    }
}

#[cfg(test)]
mod tests {
    use crate::features::{create_feature, Feature, Point, Vector};

    #[test]
    fn tuple_with_w_equals_1_0_is_a_point() {
        let a = (4.3, -4.2, 3.1, 1.0);

        let feature: Box<dyn Feature> = create_feature(a);

        let point: &Point = match feature.as_any().downcast_ref::<Point>() {
            Some(point) => point,
            None => panic!("&feature isn't a Point!"),
        };

        assert_eq!(a.0, point.x);
        assert_eq!(a.1, point.y);
        assert_eq!(a.2, point.z);
    }

    #[test]
    fn tuple_with_w_equals_0_is_a_vector() {
        let a = (4.3, -4.2, 3.1, 0.0);

        let feature: Box<dyn Feature> = create_feature(a);

        let vector: &Vector = match feature.as_any().downcast_ref::<Vector>() {
            Some(vector) => vector,
            None => panic!("&feature isn't a Vector!"),
        };

        assert_eq!(a.0, vector.x);
        assert_eq!(a.1, vector.y);
        assert_eq!(a.2, vector.z);
    }

    #[test]
    fn test_point_returns_tuple_w_equal_1_0() {
        let point = Point { x: 4.0, y: -4.0, z: 3.0 };

        let tuple = point.to_tuple();

        assert_eq!(point.x, tuple.0);
        assert_eq!(point.y, tuple.1);
        assert_eq!(point.z, tuple.2);
        assert_eq!(1.0, tuple.3);
    }

    #[test]
    fn test_vector_returns_tuple_w_equals_0_0() {
        let vector = Vector { x: 4.0, y: -4.0, z: 3.0 };

        let tuple = vector.to_tuple();

        assert_eq!(vector.x, tuple.0);
        assert_eq!(vector.y, tuple.1);
        assert_eq!(vector.z, tuple.2);
        assert_eq!(0.0, tuple.3)
    }
}