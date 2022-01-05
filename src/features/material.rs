use crate::features::color::Color;
use crate::features::color::consts::{BLACK, WHITE};
use crate::features::light::PointLight;
use crate::features::point::Point;
use crate::features::vector::Vector;

#[derive(Clone, Copy, PartialEq)]
pub struct Material {
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64
}

impl Material {
    pub fn create() -> Material {
        Material {
            color: WHITE,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0
        }
    }

    pub fn equals(&self, other_material: Material) -> bool {
        self.ambient == other_material.ambient &&
            self.diffuse == other_material.diffuse &&
            self.specular == other_material.specular &&
            self.shininess == other_material.shininess &&
            self.color.equals(other_material.color)
    }

    pub fn set_ambient(&mut self, ambient: f64) {
        self.ambient = ambient;
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_diffuse(&mut self, diffuse: f64) {
        self.diffuse = diffuse;
    }

    pub fn set_specular(&mut self, specular: f64) {
        self.specular = specular;
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn lighting(&self, light: PointLight, position: Point, eye_vector: Vector, normal_vector: Vector, in_shadow: bool) -> Color {
        let effective_color = self.color.multiply_colors(light.intensity);
        let light_vector = light.position.subtract_point(position).normalize();
        let ambient = effective_color.multiply(self.ambient);

        let light_dot_normal = light_vector.dot(normal_vector);
        let diffuse = self.calculate_diffuse(light_dot_normal, &effective_color);
        let specular = self.calculate_specular(light_dot_normal, light_vector, normal_vector, eye_vector, light.intensity);

        if in_shadow {
            return ambient;
        }

        ambient.add(diffuse).add(specular)
    }

    fn calculate_diffuse(&self, light_dot_normal: f64, effective_color: &Color) -> Color {
        if light_dot_normal < 0.0 {
            return BLACK;
        }
        effective_color.multiply(self.diffuse * light_dot_normal)
    }

    fn calculate_specular(&self, light_dot_normal: f64, light_vector: Vector, normal_vector: Vector, eye_vector: Vector, intensity: Color) -> Color {
        if light_dot_normal < 0.0 {
            return BLACK;
        }
        let reflection_vector = light_vector.negate().reflect(normal_vector);
        let reflection_dot_eye = reflection_vector.dot(eye_vector);
        if reflection_dot_eye <= 0.0 {
            return BLACK;
        }
        let factor = reflection_dot_eye.powf(self.shininess);
        intensity.multiply(self.specular).multiply(factor)
    }
}

#[cfg(test)]
mod tests {
    use crate::features::color::Color;
    use crate::features::color::consts::WHITE;
    use crate::features::light::PointLight;
    use crate::features::material::Material;
    use crate::features::point::Point;
    use crate::features::vector::Vector;

    #[test]
    fn test_default_material() {
        let material = Material::create();

        assert_eq!(material.ambient, 0.1);
        assert_eq!(material.diffuse, 0.9);
        assert_eq!(material.specular, 0.9);
        assert_eq!(material.shininess, 200.0);
    }

    #[test]
    fn test_lighting_with_the_eye_between_the_light_and_the_surface() {
        let material = Material::create();
        let position = Point::create(0.0, 0.0, 0.0);
        let eye_vector = Vector::create(0.0, 0.0, -1.0);
        let normal_vector = Vector::create(0.0, 0.0, -1.0);
        let light = PointLight::create(WHITE, Point::create(0.0, 0.0, -10.0));

        let result = material.lighting(light, position, eye_vector, normal_vector, false);

        assert!(result.equals(Color::create(1.9, 1.9, 1.9)));
    }

    #[test]
    fn test_lighting_with_the_eye_between_the_light_and_the_surface_eye_offset_45_degrees() {
        let material = Material::create();
        let position = Point::create(0.0, 0.0, 0.0);
        let eye_vector = Vector::create(0.0, 2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0);
        let normal_vector = Vector::create(0.0, 0.0, -1.0);
        let light = PointLight::create(WHITE, Point::create(0.0, 0.0, -10.0));

        let result = material.lighting(light, position, eye_vector, normal_vector, false);

        assert!(result.equals(Color::create(1.0, 1.0, 1.0)));
    }

    #[test]
    fn test_lighting_with_the_eye_opposite_surface_light_offset_45_degrees() {
        let material = Material::create();
        let position = Point::create(0.0, 0.0, 0.0);
        let eye_vector = Vector::create(0.0, 0.0, -1.0);
        let normal_vector = Vector::create(0.0, 0.0, -1.0);
        let light = PointLight::create(WHITE, Point::create(0.0, 10.0, -10.0));

        let result = material.lighting(light, position, eye_vector, normal_vector, false);

        assert!(result.equals(Color::create(0.7364, 0.7364, 0.7364)));
    }

    #[test]
    fn test_lighting_with_the_eye_in_the_path_of_the_reflection_vector() {
        let material = Material::create();
        let position = Point::create(0.0, 0.0, 0.0);
        let eye_vector = Vector::create(0.0, -2.0_f64.sqrt()/2.0, -2.0_f64.sqrt()/2.0);
        let normal_vector = Vector::create(0.0, 0.0, -1.0);
        let light = PointLight::create(WHITE, Point::create(0.0, 10.0, -10.0));

        let result = material.lighting(light, position, eye_vector, normal_vector, false);

        assert!(result.equals(Color::create(1.6364, 1.6364, 1.6364)));
    }

    #[test]
    fn test_lighting_with_the_light_behind_the_surface() {
        let material = Material::create();
        let position = Point::create(0.0, 0.0, 0.0);
        let eye_vector = Vector::create(0.0, 0.0, -1.0);
        let normal_vector = Vector::create(0.0, 0.0, -1.0);
        let light = PointLight::create(WHITE, Point::create(0.0, 0.0, 10.0));

        let result = material.lighting(light, position, eye_vector, normal_vector, false);

        assert!(result.equals(Color::create(0.1, 0.1, 0.1)));
    }

    #[test]
    fn test_lighting_with_the_surface_in_shadow() {
        let material = Material::create();
        let position = Point::create(0.0, 0.0, 0.0);
        let eye_vector = Vector::create(0.0, 0.0, -1.0);
        let normal_vector = Vector::create(0.0, 0.0, -1.0);
        let light = PointLight::create(WHITE, Point::create(0.0, 0.0, -10.0));
        let in_shadow = true;

        let result = material.lighting(light, position, eye_vector, normal_vector, in_shadow);

        assert!(result.equals(Color::create(0.1, 0.1, 0.1)));
    }
}