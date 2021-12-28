use crate::features::vector::Vector;

pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Point {
    pub fn add(&self, _vector: Vector) -> Point {
        Point{
            x: self.x + _vector.x,
            y: self.y + _vector.y,
            z: self.z + _vector.z
        }
    }

    pub fn subtract_vector(&self, _vector: Vector) -> Vector {
        Vector{
            x: self.x - _vector.x,
            y: self.y - _vector.y,
            z: self.z - _vector.z
        }
    }

    pub fn subtract_point(&self, _point: Point) -> Vector {
        Vector{
            x: self.x - _point.x,
            y: self.y - _point.y,
            z: self.z - _point.z
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::features::point::Point;
    use crate::features::vector::Vector;

    #[test]
    fn test_add_vector_and_point_creates_point() {
        let point = Point{x: 3.0, y:-2.0, z:5.0};
        let vector = Vector{x: -2.0, y:3.0, z:1.0};

        let new_point = point.add(vector);

        assert_eq!(1.0, new_point.x);
        assert_eq!(1.0, new_point.y);
        assert_eq!(6.0, new_point.z);
    }

    #[test]
    fn test_subtract_points_creates_vector() {
        let point_a = Point{x:3.0, y:2.0, z:1.0};
        let point_b = Point{x:5.0, y:6.0, z:7.0};

        let vector = point_a.subtract_point(point_b);

        assert_eq!(-2.0, vector.x);
        assert_eq!(-4.0, vector.y);
        assert_eq!(-6.0, vector.z);
    }

    #[test]
    fn test_subtract_vector_from_point_creates_vector() {
        let point_a = Point{x:3.0, y:2.0, z:1.0};
        let vector_a = Vector{x:5.0, y:6.0, z:7.0};

        let vector = point_a.subtract_vector(vector_a);

        assert_eq!(-2.0, vector.x);
        assert_eq!(-4.0, vector.y);
        assert_eq!(-6.0, vector.z);
    }
}