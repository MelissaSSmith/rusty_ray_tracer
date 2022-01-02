use crate::features::color::Color;
use crate::features::color::consts::WHITE;
use crate::features::intersection::Intersection;
use crate::features::light::PointLight;
use crate::features::material::Material;
use crate::features::matrix::Matrix;
use crate::features::point::Point;
use crate::features::ray::Ray;
use crate::features::shapes::Shape;
use crate::features::shapes::sphere::Sphere;

// #[derive(Clone, PartialEq)]
// enum Object {
//     Sphere(Sphere)
// }

#[derive(Clone)]
struct World {
    objects: Vec<Box<dyn Shape>>,
    light: Option<PointLight>
}

impl World {
    pub fn create() -> Self {
        Self {
            light: None,
            objects: vec![]
        }
    }

    fn create_default() -> Self {
        let mut s1_material = Material::create();
        s1_material.set_color(Color::create(0.8, 1.0, 0.6));
        s1_material.set_diffuse(0.7);
        s1_material.set_specular(0.2);
        let mut s1 = Sphere::create();
        s1.set_material(s1_material);
        let mut s2 = Sphere::create();
        s2.set_transform(Matrix::scale(0.5, 0.5, 0.5));
        let light = PointLight::create(WHITE, Point::create(-10.0, 10.0, -10.0));
        Self {
            light: Some(light),
            objects: vec![Box::new(s1), Box::new(s2)]
        }
    }

    pub fn objects(self) -> Vec<Box<dyn Shape>> {
        self.objects
    }

    pub fn light(self) -> Option<PointLight> {
        self.light
    }

    fn intersect(&self, _ray: Ray) -> Vec<Intersection>{
        let mut intersections = vec![];
        for object in self.objects.iter() {
            let object_intersections = object.intersect(_ray);
            for intersection in object_intersections {
                intersections.push(intersection);
            }
        }
        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        intersections
    }
}

#[cfg(test)]
mod tests {
    use crate::features::color::Color;
    use crate::features::color::consts::WHITE;
    use crate::features::intersection::Intersection;
    use crate::features::material::Material;
    use crate::features::matrix::Matrix;
    use crate::features::point::Point;
    use crate::features::ray::Ray;
    use crate::features::shapes::Shape;
    use crate::features::shapes::sphere::Sphere;
    use crate::features::vector::Vector;
    use crate::features::world::World;

    #[test]
    fn test_create_a_world() {
        let world = World::create();

        assert_eq!(0, world.objects.len());
        assert!(world.light.is_none())
    }

    #[test]
    fn test_create_default_world() {
        let mut s1 = Sphere::create();
        let mut m1 = Material::create();
        m1.set_color(Color::create(0.8, 1.0, 0.6));
        m1.set_diffuse(0.7);
        m1.set_specular(0.2);
        s1.set_material(m1);
        let mut s2 = Sphere::create();
        s2.set_transform(Matrix::scale(0.5, 0.5, 0.5));

        let world = World::create_default();

        assert!(world.clone().light().unwrap().position.equals(Point::create(-10.0, 10.0, -10.0)));
        assert!(world.clone().light().unwrap().intensity.equals(WHITE));
        assert_eq!(2, world.clone().objects().len());
        assert!(world.clone().objects()[0].equals(s1.as_any()));
        assert!(world.clone().objects()[1].equals(s2.as_any()));
    }

    #[test]
    fn test_intersect_world_with_a_ray() {
        let world = World::create_default();
        let ray = Ray::create(Point::create(0.0, 0.0, -5.0), Vector::create(0.0, 0.0, 1.0));

        let intersections: Vec<Intersection> = world.intersect(ray);

        assert_eq!(intersections.len(), 4);
        assert_eq!(intersections[0].t, 4.0);
        assert_eq!(intersections[1].t, 4.5);
        assert_eq!(intersections[2].t, 5.5);
        assert_eq!(intersections[3].t, 6.0);
    }
}