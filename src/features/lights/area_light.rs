use std::borrow::BorrowMut;
use crate::features::color::Color;
use crate::features::primitives::point::Point;
use crate::features::primitives::vector::Vector;
use crate::features::sequence::Sequence;
use crate::features::world::World;

#[derive(Clone)]
pub struct AreaLight {
    corner: Point,
    u_vec: Vector,
    u_steps: usize,
    v_vec: Vector,
    v_steps: usize,
    positions: Vec<Point>,
    color: Color,
    jitter: Sequence
}

impl AreaLight {
    pub fn create(corner: Point, v1: Vector, u_steps: usize, v2: Vector, v_steps: usize, color: Color, jitter: bool) -> AreaLight {
        let u_vec = v1 / u_steps as f64;
        let v_vec = v2 / v_steps as f64;
        let samples = u_steps * v_steps;
        let mut sequence = Sequence::one(0.5);
        if jitter {
            sequence = Sequence::new();
        }

        let positions = {
            let mut result = Vec::<Point>::with_capacity(samples);

            for v in 0..v_steps {
                for u in 0..u_steps {
                    result.push(AreaLight::point_on_light(corner, u_vec, v_vec, u, v, sequence.borrow_mut()));
                }
            }

            result
        };

        AreaLight {
            corner,
            u_vec,
            u_steps,
            v_vec,
            v_steps,
            positions,
            color,
            jitter: sequence
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

    pub(crate) fn samples(&self) -> usize {
        self.positions.len()
    }

    pub(crate) fn positions(&self) -> &[Point] {
        &self.positions
    }

    pub(crate) fn intensity(&self) -> Color {
        self.color
    }

    fn jitter(&self) -> Sequence {
        self.jitter
    }

    pub fn with_jitter(self, jitter: Sequence) -> AreaLight {
        AreaLight {
            jitter,
            ..self
        }
    }

    fn point_on_light(corner: Point, u_vec: Vector, v_vec: Vector, u: usize, v: usize, sequence: &mut Sequence) -> Point {
        corner + u_vec * (u as f64 + sequence.next()) + v_vec * (v as f64 + sequence.next())
    }

    pub(crate) fn intensity_at(&self, point: &Point, world: &World) -> f64 {
        let mut total = 0.0;
        let mut sequence = self.jitter();

        for v in 0..self.v_steps() {
            for u in 0..self.u_steps() {
                let light_position = AreaLight::point_on_light(self.corner(), self.u_vec(), self.v_vec(), u, v, &mut sequence);
                if !world.is_shadowed(light_position, *point) {
                    total = total + 1.0;
                }
            }
        }
        total / self.samples() as f64
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;
    use crate::features::color::consts::WHITE;
    use crate::features::lights::area_light::AreaLight;
    use crate::features::lights::Light;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;
    use crate::features::primitives::vector::Vector;
    use crate::features::sequence::Sequence;
    use crate::features::world::World;

    #[test]
    fn test_creating_an_area_light() {
        let corner = Point::zero();
        let v1 = Vector::create(2.0, 0.0, 0.0);
        let v2 = Vector::create(0.0, 0.0, 1.0);

        let light = AreaLight::create(corner, v1, 4, v2, 2, WHITE, false);

        assert_eq!(light.corner(), corner);
        assert_eq!(light.u_vec(), Vector::create(0.5, 0.0, 0.0));
        assert_eq!(light.u_steps(), 4);
        assert_eq!(light.v_vec(), Vector::create(0.0, 0.0, 0.5));
        assert_eq!(light.v_steps(), 2);
        assert_eq!(light.samples(), 8);
    }

    #[test]
    fn test_finding_a_single_point_on_an_area_light() {
        let corner = Point::zero();
        let v1 = Vector::create(2.0, 0.0, 0.0);
        let v2 = Vector::create(0.0, 0.0, 1.0);

        let light = AreaLight::create(corner, v1, 4, v2, 2, WHITE, false);

        let tests = vec![
            (0, 0, Point::create(0.25, 0.0, 0.25)),
            (1, 0, Point::create(0.75, 0.0, 0.25)),
            (0, 1, Point::create(0.25, 0.0, 0.75)),
            (2, 0, Point::create(1.25, 0.0, 0.25)),
            (3, 1, Point::create(1.75, 0.0, 0.75))
        ];

        for test in tests {
            let point = AreaLight::point_on_light(light.corner(), light.u_vec(),
                                                  light.v_vec(), test.0, test.1,
                                                  light.jitter().borrow_mut());

            assert_eq!(point, test.2);
        }
    }

    #[test]
    fn test_area_light_intensity_function() {
        let mut world = World::create_default();
        let corner = Point::create(-0.5, -0.5, -5.0);
        let v1 = Vector::create(1.0, 0.0, 0.0);
        let v2 = Vector::create(0.0, 1.0, 0.0);
        let light = AreaLight::create(corner, v1, 2, v2, 2, WHITE, false);
        world.set_light(0, Light::create_area_light(light.clone()));

        let tests = vec![
            (Point::create(0.0, 0.0, 2.0), 0.0),
            (Point::create(1.0, -1.0, 2.0), 0.25),
            (Point::create(1.5, 0.0, 2.0), 0.5),
            (Point::create(1.25, 1.25, 3.0), 0.75),
            (Point::create(0.0, 0.0, -2.0), 1.0)
        ];

        for test in tests {
            let intensity = light.intensity_at(&test.0, &world);

            assert_eq!(intensity, test.1);
        }
    }

    #[test]
    fn test_finding_a_single_point_on_a_jittered_area_light() {
        let corner = Point::zero();
        let v1 = Vector::create(2.0, 0.0, 0.0);
        let v2 = Vector::create(0.0, 0.0, 1.0);
        let light = AreaLight::create(corner, v1, 4, v2, 2, WHITE, false)
            .with_jitter(Sequence::two(0.3, 0.7));

        let tests = vec![
            (0, 0, Point::create(0.15, 0.0, 0.35)),
            (1, 0, Point::create(0.65, 0.0, 0.35)),
            (0, 1, Point::create(0.15, 0.0, 0.85)),
            (2, 0, Point::create(1.15, 0.0, 0.35)),
            (3, 1, Point::create(1.65, 0.0, 0.85))
        ];

        for test in tests {
            let point = AreaLight::point_on_light(light.corner(), light.u_vec(),
                                                  light.v_vec(), test.0, test.1,
                                                  light.jitter().borrow_mut());

            assert_eq!(point, test.2);
        }
    }

    #[test]
    fn test_area_light_with_jittered_samples() {
        let mut world = World::create_default();
        let corner = Point::create(-0.5, -0.5, -5.0);
        let v1 = Vector::create(1.0, 0.0, 0.0);
        let v2 = Vector::create(0.0, 1.0, 0.0);
        let light = AreaLight::create(corner, v1, 2, v2, 2, WHITE, false)
            .with_jitter(Sequence::five(0.7, 0.3, 0.9, 0.1, 0.5));
        world.set_light(0, Light::create_area_light(light.clone()));


        let tests = vec![
            (Point::create(0.0, 0.0, 2.0), 0.0),
            (Point::create(1.0, -1.0, 2.0), 0.5),
            (Point::create(1.5, 0.0, 2.0), 0.75),
            (Point::create(1.25, 1.25, 3.0), 0.75),
            (Point::create(0.0, 0.0, -2.0), 1.0)
        ];

        for test in tests {
            let point = test.0;
            let intensity = light.intensity_at(&point, &world);

            assert_eq!(intensity, test.1);
        }
    }
}