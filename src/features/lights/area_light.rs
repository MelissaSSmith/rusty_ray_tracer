use crate::features::color::Color;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::primitives::vector::Vector;

#[derive(Clone, Copy)]
pub struct AreaLight {
    corner: Point,
    u_vec: Vector,
    u_steps: usize,
    v_vec: Vector,
    v_steps: usize,
    samples: usize,
    position: Point,
    color: Color
}

impl AreaLight {
    pub fn create(corner: Point, v1: Vector, u_steps: usize, v2: Vector, v_steps: usize, color: Color) -> AreaLight {
        let position = Point::create(1.0, 00.0, 0.5);
        AreaLight {
            corner,
            u_vec: v1 / u_steps as f64,
            u_steps,
            v_vec: v2 / v_steps as f64,
            v_steps,
            samples: u_steps * v_steps,
            position,
            color
        }
    }

    fn corner(&self) -> Point {
        self.corner
    }

    fn u_vec(&self) -> Vector {
        self.u_vec
    }

    fn v_vec(&self) -> Vector {
        self.v_vec
    }

    fn u_steps(&self) -> usize {
        self.u_steps
    }

    fn v_steps(&self) -> usize {
        self.v_steps
    }

    fn samples(&self) -> usize {
        self.samples
    }

    fn position(&self) -> Point {
        self.position
    }
}

#[cfg(test)]
mod tests {
    use crate::features::color::consts::WHITE;
    use crate::features::lights::area_light::AreaLight;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;
    use crate::features::primitives::vector::Vector;

    #[test]
    fn test_creating_an_area_light() {
        let corner = Point::zero();
        let v1 = Vector::create(2.0, 0.0, 0.0);
        let v2 = Vector::create(0.0, 0.0, 1.0);

        let light = AreaLight::create(corner, v1, 4, v2, 2, WHITE);

        assert!(light.corner().equals(corner));
        assert!(light.u_vec().equals(Vector::create(0.5, 0.0, 0.0)));
        assert_eq!(light.u_steps(), 4);
        assert!(light.v_vec().equals(Vector::create(0.0, 0.0, 0.5)));
        assert_eq!(light.v_steps(), 2);
        assert_eq!(light.samples(), 8);
        assert!(light.position().equals(Point::create(1.0, 0.0, 0.5)));
    }

    #[test]
    fn test_finding_a_single_point_on_an_area_light() {
        let corner = Point::zero();
        let v1 = Vector::create(2.0, 0.0, 0.0);
        let v2 = Vector::create(0.0, 0.0, 1.0);

        let light = AreaLight::create(corner, v1, 4, v2, 2, WHITE);

        let tests = vec![
            (0, 0, Point::create(0.25, 0.0, 0.25))
        ];
    }
}