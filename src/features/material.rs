use crate::features::color::Color;
use crate::features::color::consts::{BLACK, WHITE};
use crate::features::light::PointLight;
use crate::features::patterns::{Pattern, PatternTrait};
use crate::features::primitives::point::Point;
use crate::features::primitives::vector::Vector;
use crate::features::shapes::shape::Object;

#[derive(Clone)]
pub struct Material {
    pattern: Pattern,
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
    reflective: f64,
    transparency: f64,
    refractive_index: f64
}

impl Material {
    pub fn create() -> Material {
        Material {
            pattern: todo!(),
            color: WHITE,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0
        }
    }

    //todo: phase out
    pub fn create_with_attributes(ambient: f64, diffuse: f64, specular: f64, shininess: Option<f64>, reflective: Option<f64>, color: Option<Color>, pattern: Option<Box<dyn PatternTrait>>) -> Material {
        let m_color = match color {
            None => { WHITE }
            Some(c) => { c }
        };
        let m_shininess = match shininess {
            None => { 200.0 }
            Some(s) => { s }
        };
        let m_reflective = match reflective {
            None => { 0.0 }
            Some(r) => { r }
        };
        Material{
            pattern: todo!(),
            color: m_color,
            ambient,
            diffuse,
            specular,
            shininess: m_shininess,
            reflective: m_reflective,
            transparency: 0.0,
            refractive_index: 1.0
        }
    }

    pub fn equals(&self, other_material: Material) -> bool {
        self.ambient == other_material.ambient &&
            self.diffuse == other_material.diffuse &&
            self.specular == other_material.specular &&
            self.shininess == other_material.shininess &&
            self.reflective == other_material.reflective &&
            self.transparency == other_material.transparency &&
            self.refractive_index == other_material.refractive_index &&
            self.color.equals(other_material.color) //todo: add pattern
    }

    //setters
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

    pub fn set_reflective(&mut self, reflective: f64) {
        self.reflective = reflective;
    }

    pub fn set_transparency(&mut self, transparency: f64) {
        self.transparency = transparency;
    }

    pub fn set_refractive_index(&mut self, refractive_index: f64) {
        self.refractive_index = refractive_index;
    }

    pub fn set_pattern(&mut self, pattern: Pattern) {
        self.pattern = pattern;
    }

    //builders
    pub fn with_ambient(self, ambient: f64) -> Material {
        Material {
            ambient,
            ..self
        }
    }

    pub fn with_color(self, color: Color) -> Material {
        Material {
            color,
            ..self
        }
    }

    pub fn with_diffuse(self, diffuse: f64) -> Material {
        Material {
            diffuse,
            ..self
        }
    }

    pub fn with_specular(self, specular: f64) -> Material {
        Material {
            specular,
            ..self
        }
    }

    pub fn with_reflective(self, reflective: f64) -> Material {
        Material {
            reflective,
            ..self
        }
    }

    pub fn with_refractive_index(self, refractive_index: f64) -> Material {
        Material {
            refractive_index,
            ..self
        }
    }

    pub fn with_transparency(self, transparency: f64) -> Material {
        Material {
            transparency,
            ..self
        }
    }

    pub fn with_pattern(self, pattern: Pattern) -> Material {
        Material {
            pattern,
            ..self
        }
    }

    //getters
    pub fn color(&self) -> Color {
        self.color
    }

    pub fn reflective(&self) -> f64 {
        self.reflective
    }

    pub fn transparency(&self) -> f64 {
        self.transparency
    }

    pub fn refractive_index(&self) -> f64 {
        self.refractive_index
    }

    pub fn lighting(&self, light: &PointLight, object: &Object, position: &Point, eye_vector: &Vector, normal_vector: &Vector, in_shadow: bool) -> Color { //todo: use pattern
        let color = self.pattern.pattern_at_object(object, position);

        let effective_color = color * light.intensity;
        let ambient = effective_color * self.ambient;
        if in_shadow {
            return ambient;
        }

        let mut diffuse = BLACK;
        let mut specular = BLACK;

        let light_vector = (light.position - *position).normalize();
        let light_dot_normal = light_vector ^ *normal_vector;
        if light_dot_normal > 0.0 {
            diffuse = effective_color * self.diffuse * light_dot_normal;

            let reflection_vector = (-light_vector).reflect(*normal_vector);
            let reflection_dot_eye = reflection_vector ^ *eye_vector;
            if reflection_dot_eye > 0.0 {
                let factor = reflection_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor
            }
        }

        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod tests {
    use crate::features::color::Color;
    use crate::features::color::consts::{BLACK, WHITE};
    use crate::features::light::PointLight;
    use crate::features::material::Material;
    use crate::features::patterns::stripe::StripePattern;
    use crate::features::primitives::point::Point;
    use crate::features::shapes::sphere::Sphere;
    use crate::features::primitives::tuple_trait::Tuple;
    use crate::features::primitives::vector::Vector;
    use crate::features::shapes::shape::Shape;

    #[test]
    fn test_default_material() {
        let material = Material::create();

        assert_eq!(material.ambient, 0.1);
        assert_eq!(material.diffuse, 0.9);
        assert_eq!(material.specular, 0.9);
        assert_eq!(material.shininess, 200.0);
        assert_eq!(material.reflective, 0.0);
        assert_eq!(material.transparency, 0.0);
        assert_eq!(material.refractive_index, 1.0);
    }

    #[test]
    fn test_lighting_with_the_eye_between_the_light_and_the_surface() {
        let material = Material::create();
        let position = Point::zero();
        let eye_vector = Vector::create(0.0, 0.0, -1.0);
        let normal_vector = Vector::create(0.0, 0.0, -1.0);
        let light = PointLight::create(WHITE, Point::create(0.0, 0.0, -10.0));

        let result = material.lighting(&light, &Shape::Sphere.create(), &position, &eye_vector, &normal_vector, false);

        assert!(result.equals(Color::create(1.9, 1.9, 1.9)));
    }

    #[test]
    fn test_lighting_with_the_eye_between_the_light_and_the_surface_eye_offset_45_degrees() {
        let material = Material::create();
        let position = Point::zero();
        let eye_vector = Vector::create(0.0, 2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0);
        let normal_vector = Vector::create(0.0, 0.0, -1.0);
        let light = PointLight::create(WHITE, Point::create(0.0, 0.0, -10.0));

        let result = material.lighting(&light, &Shape::Sphere.create(), &position, &eye_vector, &normal_vector, false);

        assert!(result.equals(Color::create(1.0, 1.0, 1.0)));
    }

    #[test]
    fn test_lighting_with_the_eye_opposite_surface_light_offset_45_degrees() {
        let material = Material::create();
        let position = Point::zero();
        let eye_vector = Vector::create(0.0, 0.0, -1.0);
        let normal_vector = Vector::create(0.0, 0.0, -1.0);
        let light = PointLight::create(WHITE, Point::create(0.0, 10.0, -10.0));

        let result = material.lighting(&light, &Shape::Sphere.create(), &position, &eye_vector, &normal_vector, false);

        assert!(result.equals(Color::create(0.7364, 0.7364, 0.7364)));
    }

    #[test]
    fn test_lighting_with_the_eye_in_the_path_of_the_reflection_vector() {
        let material = Material::create();
        let position = Point::zero();
        let eye_vector = Vector::create(0.0, -2.0_f64.sqrt()/2.0, -2.0_f64.sqrt()/2.0);
        let normal_vector = Vector::create(0.0, 0.0, -1.0);
        let light = PointLight::create(WHITE, Point::create(0.0, 10.0, -10.0));

        let result = material.lighting(&light, &Shape::Sphere.create(), &position, &eye_vector, &normal_vector, false);

        assert!(result.equals(Color::create(1.6364, 1.6364, 1.6364)));
    }

    #[test]
    fn test_lighting_with_the_light_behind_the_surface() {
        let material = Material::create();
        let position = Point::zero();
        let eye_vector = Vector::create(0.0, 0.0, -1.0);
        let normal_vector = Vector::create(0.0, 0.0, -1.0);
        let light = PointLight::create(WHITE, Point::create(0.0, 0.0, 10.0));

        let result = material.lighting(&light, &Shape::Sphere.create(), &position, &eye_vector, &normal_vector, false);

        assert!(result.equals(Color::create(0.1, 0.1, 0.1)));
    }

    #[test]
    fn test_lighting_with_the_surface_in_shadow() {
        let material = Material::create();
        let position = Point::zero();
        let eye_vector = Vector::create(0.0, 0.0, -1.0);
        let normal_vector = Vector::create(0.0, 0.0, -1.0);
        let light = PointLight::create(WHITE, Point::create(0.0, 0.0, -10.0));
        let in_shadow = true;

        let result = material.lighting(&light, &Shape::Sphere.create(), &position, &eye_vector, &normal_vector, in_shadow);

        assert!(result.equals(Color::create(0.1, 0.1, 0.1)));
    }

    #[test]
    fn test_lighting_with_a_pattern_applied() {
        let pattern = StripePattern::create(WHITE, BLACK);
        let material = Material::create_with_attributes(1.0, 0.0, 0.0, None,None, None, Some(Box::new(pattern)));
        let eye_vector = Vector::create(0.0, 0.0, -1.0);
        let normal_vector = Vector::create(0.0, 0.0, -1.0);
        let light = PointLight::create(WHITE, Point::create(0.0, 0.0, -10.0));

        let c1 = material.lighting(&light, &Shape::Sphere.create(), &Point::create(0.9, 0.0, 0.0), &eye_vector, &normal_vector, false);
        let c2 = material.lighting(&light, &Shape::Sphere.create(), &Point::create(1.1, 0.0, 0.0), &eye_vector, &normal_vector, false);

        assert!(c1.equals(WHITE));
        assert!(c2.equals(BLACK));
    }
}