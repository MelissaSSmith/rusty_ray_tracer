use canvas_draw::color::Color;
use linear_algebra::point::Point;
use crate::features::world::World;

#[derive(Clone, Copy)]
pub struct PointLight {
    intensity: Color,
    position: [Point; 1]
}

impl PointLight {
    pub fn create(intensity: Color, position: Point) -> Self {
        Self{intensity, position: [position]}
    }

    pub fn positions(&self) -> &[Point] {
        &self.position
    }

    pub(crate) fn intensity(&self) -> Color {
        self.intensity
    }

    pub(crate) fn intensity_at(&self, point: &Point, world: &World) -> f64 {
        if world.is_shadowed(self.position[0], *point) {
            return 0.0;
        }
        1.0
    }
}

#[cfg(test)]
mod tests {
    use canvas_draw::color::consts::WHITE;
    use crate::features::lights::point_light::PointLight;
    use linear_algebra::point::Point;
    use linear_algebra::tuple_trait::Tuple;
    use crate::features::world::World;

    #[test]
    fn test_point_light_has_position_and_intensity() {
        let intensity = WHITE;
        let position = Point::zero();

        let light = PointLight::create(intensity, position);

        assert_eq!(light.positions()[0], position);
        assert!(light.intensity.equals(intensity));
    }

    #[test]
    fn test_point_lights_evaluate_the_light_intensity_at_a_given_point() {
        let world = World::create_default();
        let light = world.lights()[0].clone();

        let tests = vec![
            (Point::create(0.0, 1.0001, 0.0), 1.0),
            (Point::create(-1.0001, 0.0, 0.0), 1.0),
            (Point::create(0.0, 0.0, -1.0001), 1.0),
            (Point::create(0.0, 0.0, 1.0001), 0.0),
            (Point::create(1.0001, 0.0, 0.0), 0.0),
            (Point::create(0.0, -1.0001, 0.0), 0.0),
            (Point::create(0.0, 0.0, 0.0), 0.0)
        ];

        for test in tests {
            let point = test.0;
            let intensity = light.intensity_at(&point, &world);

            assert_eq!(intensity, test.1);
        }
    }
}