use crate::features::color::Color;
use crate::features::point::Point;

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
    use crate::features::light::PointLight;
    use crate::features::point::Point;

    #[test]
    fn test_point_light_has_position_and_intensity() {
        let intensity = Color::create(1.0, 1.0, 1.0);
        let position = Point::create(0.0, 0.0, 0.0);

        let light = PointLight::create(intensity, position);

        assert!(light.position.equals(position));
        assert!(light.intensity.equals(intensity));
    }
}