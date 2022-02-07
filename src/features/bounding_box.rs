use std::ops::Add;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;

#[derive(Clone, Copy)]
pub struct BoundingBox {
    minimum: Point,
    maximum: Point
}

impl BoundingBox {
    pub fn create() -> BoundingBox {
        BoundingBox {
            minimum: Point::create(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            maximum: Point::create(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY)
        }
    }

    pub fn minimum(&self) -> Point {
        self.minimum
    }

    pub fn maximum(&self) -> Point {
        self.maximum
    }

    pub fn with_minimum(self, minimum: Point) -> BoundingBox {
        BoundingBox {
            minimum,
            ..self
        }
    }

    pub fn with_maximum(self, maximum: Point) -> BoundingBox {
        BoundingBox {
            maximum,
            ..self
        }
    }

    pub fn contains_point(&self, point: Point) -> bool {
        self.minimum().x() <= point.x() && point.x() <= self.maximum().x() &&
            self.minimum().y() <= point.y() && point.y() <= self.maximum().y() &&
            self.minimum().z() <= point.z() && point.z() <= self.maximum().z()
    }
}

impl Add<BoundingBox> for BoundingBox {
    type Output = BoundingBox;

    fn add(self, rhs: BoundingBox) -> Self::Output {
        let min_x = if rhs.minimum().x() < self.minimum().x() { rhs.minimum().x() } else { self.minimum().x() };
        let min_y = if rhs.minimum().y() < self.minimum().y() { rhs.minimum().y() } else { self.minimum().y() };
        let min_z = if rhs.minimum().z() < self.minimum().z() { rhs.minimum().z() } else { self.minimum().z() };

        let max_x = if rhs.maximum().x() > self.maximum().x() { rhs.maximum().x() } else { self.maximum().x() };
        let max_y = if rhs.maximum().y() > self.maximum().y() { rhs.maximum().y() } else { self.maximum().y() };
        let max_z = if rhs.maximum().z() > self.maximum().z() { rhs.maximum().z() } else { self.maximum().z() };

        BoundingBox {
            minimum: Point::create(min_x, min_y, min_z),
            maximum: Point::create(max_x, max_y, max_z)
        }
    }
}

impl Add<Point> for BoundingBox {
    type Output = BoundingBox;

    fn add(self, rhs: Point) -> Self::Output {
        let min_x = if rhs.x() < self.minimum().x() { rhs.x() } else { self.minimum().x() };
        let min_y = if rhs.y() < self.minimum().y() { rhs.y() } else { self.minimum().y() };
        let min_z = if rhs.z() < self.minimum().z() { rhs.z() } else { self.minimum().z() };

        let max_x = if rhs.x() > self.maximum().x() { rhs.x() } else { self.maximum().x() };
        let max_y = if rhs.y() > self.maximum().y() { rhs.y() } else { self.maximum().y() };
        let max_z = if rhs.z() > self.maximum().z() { rhs.z() } else { self.maximum().z() };

        BoundingBox {
            minimum: Point::create(min_x, min_y, min_z),
            maximum: Point::create(max_x, max_y, max_z)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::features::bounding_box::BoundingBox;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;

    #[test]
    fn test_create_an_empty_bounding_box() {
        let bound_box = BoundingBox::create();

        assert!(bound_box.minimum().equals(Point::create(f64::INFINITY, f64::INFINITY, f64::INFINITY)));
        assert!(bound_box.maximum().equals(Point::create(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY)));
    }

    #[test]
    fn test_create_a_bounding_box_with_volume() {
        let bound_box = BoundingBox::create()
            .with_minimum(Point::create(-1.0, -2.0, -3.0))
            .with_maximum(Point::create(3.0, 2.0, 1.0));

        assert!(bound_box.minimum().equals(Point::create(-1.0, -2.0, -3.0)));
        assert!(bound_box.maximum().equals(Point::create(3.0, 2.0, 1.0)));
    }

    #[test]
    fn test_add_points_to_an_empty_bounding_box() {
        let point_1 = Point::create(-5.0, 2.0, 0.0);
        let point_2 = Point::create(7.0, 0.0, -3.0);

        let bound_box = BoundingBox::create() + point_1 + point_2;

        assert!(bound_box.minimum().equals(Point::create(-5.0, 0.0, -3.0)));
        assert!(bound_box.maximum().equals(Point::create(7.0, 2.0, 0.0)));
    }

    #[test]
    fn test_add_one_bounding_to_another() {
        let bound_box_1 = BoundingBox::create()
            .with_minimum(Point::create(-5.0, -2.0, 0.0))
            .with_maximum(Point::create(7.0, 4.0, 4.0));
        let bound_box_2 = BoundingBox::create()
            .with_minimum(Point::create(8.0, -7.0, -2.0))
            .with_maximum(Point::create(14.0, 2.0, 8.0));

        let bound_box = bound_box_1 + bound_box_2;

        assert!(bound_box.minimum().equals(Point::create(-5.0, -7.0, -2.0)));
        assert!(bound_box.maximum().equals(Point::create(14.0, 4.0, 8.0)));
    }

    #[test]
    fn test_check_to_see_if_a_box_contains_a_given_point() {
        let bound_box = BoundingBox::create()
            .with_minimum(Point::create(5.0, -2.0, 0.0))
            .with_maximum(Point::create(11.0, 4.0, 7.0));

        let tests = vec![
            (Point::create(5.0, -2.0, 0.0), true),
            (Point::create(11.0, 4.0, 7.0), true),
            (Point::create(8.0, 1.0, 3.0), true),
            (Point::create(3.0, 0.0, 3.0), false),
            (Point::create(8.0, -4.0, 3.0), false),
            (Point::create(8.0, 1.0, -1.0), false),
            (Point::create(13.0, 1.0, 3.0), false),
            (Point::create(8.0, 5.0, 3.0), false),
            (Point::create(8.0, 1.0, 8.0), false)
        ];

        for test in tests {
            let result = bound_box.contains_point(test.0);

            assert_eq!(result, test.1);
        }
    }
}