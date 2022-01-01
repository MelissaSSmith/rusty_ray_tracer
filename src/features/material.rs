#[derive(Clone, Copy)]
pub struct Material {
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64
}

impl Material {
    pub fn create() -> Material {
        Material {
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
            self.shininess == other_material.shininess
    }

    pub fn set_ambient(&mut self, ambient: f64) {
        self.ambient = ambient;
    }
}

#[cfg(test)]
mod tests {
    use crate::features::material::Material;

    #[test]
    fn test_default_material() {
        let material = Material::create();

        assert_eq!(material.ambient, 0.1);
        assert_eq!(material.diffuse, 0.9);
        assert_eq!(material.specular, 0.9);
        assert_eq!(material.shininess, 200.0);
    }
}