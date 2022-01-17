use crate::features::color::Color;
use crate::features::primitives::point::Point;

#[derive(Clone, Copy)]
pub struct PointLight {
    pub(crate) intensity: Color,
    pub(crate) position: Point
}

impl PointLight {
    pub fn create(intensity: Color, position: Point) -> PointLight {
        PointLight{intensity, position}
    }
}

#[cfg(test)]
mod tests {
    use crate::features::color::Color;
    use crate::features::color::consts::WHITE;
    use crate::features::light::PointLight;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple::Tuple;

    #[test]
    fn test_point_light_has_position_and_intensity() {
        let intensity = WHITE;
        let position = Point::zero();

        let light = PointLight::create(intensity, position);

        assert!(light.position.equals(position));
        assert!(light.intensity.equals(intensity));
    }
}