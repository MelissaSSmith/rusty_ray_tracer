use crate::features::color::Color;
use crate::features::color::consts::{BLACK, WHITE};
use crate::features::computation::Computation;
use crate::features::intersection::Intersection;
use crate::features::light::PointLight;
use crate::features::material::Material;
use crate::features::primitives::matrix::Matrix;
use crate::features::primitives::operations::consts::EPSILON;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::ray::Ray;
use crate::features::shapes::Shape;
use crate::features::shapes::sphere::Sphere;

#[derive(Clone)]
pub struct World {
    objects: Vec<Box<dyn Shape>>,
    light: Option<PointLight>,
    recursion_limit: u8
}

impl World {
    pub fn create() -> Self {
        Self {
            light: None,
            objects: vec![],
            recursion_limit: 5
        }
    }
    
    pub fn create_world(light: PointLight, objects: Vec<Box<dyn Shape>>) -> Self {
        Self {
            objects,
            light: Some(light),
            recursion_limit: 5
        }
    }

    pub fn create_default() -> Self {
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
            objects: vec![s1.box_clone(), s2.box_clone()],
            recursion_limit: 5
        }
    }

    pub fn objects(self) -> Vec<Box<dyn Shape>> {
        self.objects
    }

    pub fn light(self) -> Option<PointLight> {
        self.light
    }

    pub fn set_light(&mut self, light: PointLight) {
        self.light = Some(light);
    }

    pub fn set_object(&mut self, index: usize, object: Box<dyn Shape>) {
        self.objects.insert(index, object);
    }

    pub fn add_object(&mut self, object: Box<dyn Shape>) {
        self.objects.append(&mut vec![object])
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        self.color_at_impl(ray, self.recursion_limit)
    }

    fn intersect(&self, _ray: Ray) -> Vec<Intersection>{
        let mut intersections = vec![];
        for object in self.objects.iter() {
            let object_intersections = object.intersect(_ray);
            for intersection in object_intersections {
                intersections.push(intersection);
            }
        }
        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap()); //todo - double verify this
        intersections
    }

    fn shade_hit(&self, computation: &Computation, remaining: u8) -> Color {
        let shadowed = self.is_shadowed(computation.over_point());
        let object = computation.clone().object();

        let lighting = computation.clone().object().material().lighting(
            self.light.unwrap(),
            object,
            computation.over_point(),
            computation.eye_vector(),
            computation.normal_vector(),
            shadowed
        );
        let reflected = self.reflected_color(&computation, remaining);

        lighting + reflected
    }

    fn reflected_color(&self, _computations: &Computation, remaining: u8) -> Color {
        let reflective = _computations.clone().object().material().reflective();
        if reflective == 0.0 || remaining <= 0 {
            return BLACK;
        }
        let reflect_ray = Ray::create(_computations.over_point(), _computations.reflect_vector());
        let color = self.color_at_impl(&reflect_ray, remaining - 1);

        color * reflective
    }

    fn color_at_impl(&self, _ray: &Ray, remaining: u8) -> Color {
        let intersection = Intersection::hit(self.intersect(*_ray));
        return match intersection {
            None => { BLACK }
            Some(i) => {
                let computations = i.prepare_computations(*_ray, &vec![]);
                self.shade_hit(&computations, remaining)
            }
        }
    }

    fn is_shadowed(&self, point: Point) -> bool {
        return match self.light {
            None => { true }
            Some(light) => {
                let v = light.position - point;
                let distance = v.magnitude();
                let direction = v.normalize();

                let ray = Ray::create(point, direction);
                let intersections = self.intersect(ray);
                let hit = Intersection::hit(intersections);
                match hit {
                    None => { false }
                    Some(h) => {
                        h.t < distance
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::features::color::Color;
    use crate::features::color::consts::{BLACK, WHITE};
    use crate::features::intersection::Intersection;
    use crate::features::light::PointLight;
    use crate::features::material::Material;
    use crate::features::primitives::matrix::Matrix;
    use crate::features::primitives::operations::Operations;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;
    use crate::features::ray::Ray;
    use crate::features::shapes::plane::Plane;
    use crate::features::shapes::Shape;
    use crate::features::shapes::sphere::Sphere;
    use crate::features::primitives::vector::Vector;
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

    #[test]
    fn test_shading_an_intersection() {
        let world = World::create_default();
        let ray = Ray::create(Point::create(0.0, 0.0, -5.0), Vector::create(0.0, 0.0, 1.0));
        let shape = world.clone().objects()[0].clone();
        let intersection = Intersection::create(4.0, shape);

        let computation = intersection.prepare_computations(ray, &vec![]);
        let color = world.shade_hit(&computation, 1);

        assert!(color.equals(Color::create(0.38066, 0.47583, 0.2855)));
    }

    #[test]
    fn test_shading_an_intersection_from_the_inside() {
        let mut world = World::create_default();
        world.light = Some(PointLight::create(WHITE, Point::create(0.0, 0.25, 0.0)));
        let ray = Ray::create(Point::zero(), Vector::create(0.0, 0.0, 1.0));
        let shape = world.clone().objects()[1].clone();
        let intersection = Intersection::create(0.5, shape);

        let computation = intersection.prepare_computations(ray, &vec![]);
        let color = world.shade_hit(&computation, 1);

        assert!(color.equals(Color::create(0.90498, 0.90498, 0.90498)));
    }

    #[test]
    fn test_color_when_a_ray_misses() {
        let world = World::create_default();
        let ray = Ray::create(Point::create(0.0, 0.0, -5.0), Vector::create(0.0, 1.0, 0.0));

        let color = world.color_at(&ray);

        assert!(color.equals(Color::create(0.0, 0.0, 0.0)));
    }

    #[test]
    fn test_color_when_a_ray_hits() {
        let world = World::create_default();
        let ray = Ray::create(Point::create(0.0, 0.0, -5.0), Vector::create(0.0, 0.0, 1.0));

        let color = world.color_at(&ray);

        assert!(color.equals(Color::create(0.38066, 0.47583, 0.2855)));
    }

    #[test]
    fn test_color_with_an_intersection_behind_the_ray() {
        let mut s1_material = Material::create();
        s1_material.set_color(Color::create(0.8, 1.0, 0.6));
        s1_material.set_diffuse(0.7);
        s1_material.set_specular(0.2);
        s1_material.set_ambient(1.0);
        let mut s1 = Sphere::create();
        s1.set_material(s1_material);
        let mut s2_material = Material::create();
        s2_material.set_ambient(1.0);
        let mut s2 = Sphere::create();
        s2.set_transform(Matrix::scale(0.5, 0.5, 0.5));
        s2.set_material(s2_material);
        let light = PointLight::create(WHITE, Point::create(-10.0, 10.0, -10.0));
        let mut world = World::create();
        world.light = Some(light);
        world.objects = vec![Box::new(s1), Box::new(s2)];
        let ray = Ray::create(Point::create(0.0, 0.0, 0.75), Vector::create(0.0, 0.0, -1.0));

        let color = world.color_at(&ray);

        assert!(color.equals(world.objects[1].material().color()));
    }

    #[test]
    fn test_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let world = World::create_default();
        let point = Point::create(0.0, 10.0, 0.0);

        let result = world.is_shadowed(point);

        assert_eq!(result, false);
    }

    #[test]
    fn test_shadow_when_an_object_is_between_the_point_and_the_light() {
        let world = World::create_default();
        let point = Point::create(10.0, -10.0, 10.0);

        let result = world.is_shadowed(point);

        assert_eq!(result, true);
    }

    #[test]
    fn test_no_shadow_when_an_object_is_behind_the_light() {
        let world = World::create_default();
        let point = Point::create(-20.0, 20.0, -20.0);

        let result = world.is_shadowed(point);

        assert_eq!(result, false);
    }

    #[test]
    fn test_no_shadow_when_an_object_is_behind_the_point() {
        let world = World::create_default();
        let point = Point::create(-2.0, 2.0, -2.0);

        let result = world.is_shadowed(point);

        assert_eq!(result, false);
    }

    #[test]
    fn test_shade_hit_is_given_an_intersection_in_shadow() {
        let light = PointLight::create(WHITE, Point::create(0.0, 0.0, -10.0));
        let s1 = Sphere::create();
        let mut s2 = Sphere::create();
        s2.set_transform(Matrix::translate(0.0, 0.0, 10.0));
        let objects: Vec<Box<dyn Shape>> = vec![Box::new(s1), Box::new(s2.clone())];
        let world = World::create_world(light, objects);
        let ray = Ray::create(Point::create(0.0, 0.0, 5.0), Vector::create(0.0, 0.0, 1.0));
        let intersection = Intersection::create(4.0, Box::new(s2.clone()));
        let computations = intersection.prepare_computations(ray, &vec![]);

        let color = world.shade_hit(&computations, 1);

        assert!(color.equals(Color::create(0.1, 0.1, 0.1)));
    }

    #[test]
    fn test_reflect_color_for_a_non_reflective_material() {
        let mut world = World::create_default();
        let ray = Ray::create(Point::zero(), Vector::create(0.0, 0.0, 1.0));
        let mut shape = world.clone().objects().get(1).unwrap().clone();
        shape.material().set_ambient(1.0);
        world.set_object(1, shape.clone());
        let intersection = Intersection::create(1.0, shape);

        let computation = intersection.prepare_computations(ray, &vec![]);

        let color = world.reflected_color(&computation, 1);

        assert!(color.equals(BLACK));
    }

    #[test]
    fn test_reflect_color_for_a_reflective_material() {
        let sqrt2 = f64::sqrt(2.0);

        let mut world = World::create_default();
        let ray = Ray::create(Point::create(0.0, 0.0, -3.0), Vector::create(0.0, -sqrt2/2.0, sqrt2/2.0));
        let mut material = Material::create();
        material.set_reflective(0.5);
        let mut shape = Plane::create();
        shape.set_transform(Matrix::translate(0.0, -1.0, 0.0));
        shape.set_material(material);
        world.add_object(shape.box_clone());

        let intersection = Intersection::create(sqrt2, shape.box_clone());

        let computation = intersection.prepare_computations(ray, &vec![]);

        let color = world.reflected_color(&computation, 1);

        println!("{} {} {}", color.red, color.green, color.blue);
        assert!(color.equals(Color::create(0.19032, 0.2379, 0.14274)));
    }

    #[test]
    fn test_shade_hit_with_a_reflective_material() {
        let sqrt2 = f64::sqrt(2.0);

        let mut world = World::create_default();
        let ray = Ray::create(Point::create(0.0, 0.0, -3.0), Vector::create(0.0, -sqrt2/2.0, sqrt2/2.0));
        let mut shape = Plane::create();
        let mut material = Material::create();
        material.set_reflective(0.5);
        shape.set_transform(Matrix::translate(0.0, -1.0, 0.0));
        shape.set_material(material);
        world.add_object(shape.box_clone());

        let intersection = Intersection::create(sqrt2.sqrt(), shape.box_clone());

        let computation = intersection.prepare_computations(ray, &vec![]);

        let color = world.shade_hit(&computation, 1);

        println!("{} {} {}", color.red, color.green, color.blue);
        assert!(color.equals(Color::create(0.87677, 0.92436, 0.82918)));
    }

    #[test]
    fn test_color_at_with_mutually_reflective_surfaces() {
        let mut world = World::create();
        world.set_light(PointLight::create(WHITE, Point::zero()));

        let mut material = Material::create();
        material.set_reflective(1.0);
        let mut lower = Plane::create();
        lower.set_material(material.clone());
        lower.set_transform(Matrix::translate(0.0, -1.0, 0.0));

        let mut upper = Plane::create();
        upper.set_material(material);
        upper.set_transform(Matrix::translate(0.0, 1.0, 0.0));

        world.add_object(Box::new(lower));
        world.add_object(Box::new(upper));

        let ray = Ray::create(Point::zero(), Vector::create(0.0, 1.0, 0.0));

        let _ = world.color_at(&ray);
    }

    #[test]
    fn test_reflected_color_at_maximum_recursive_depth() {
        let mut world = World::create_default();
        let ray = Ray::create(Point::create(0.0, 0.0, -3.0), Vector::create(0.0, -2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0));
        let mut shape = Plane::create();
        let mut material = Material::create();
        material.set_reflective(0.5);
        shape.set_transform(Matrix::translate(0.0, -1.0, 0.0));
        shape.set_material(material);
        world.add_object(Box::new(shape.clone()));

        let intersection = Intersection::create(2.0_f64.sqrt(), Box::new(shape));

        let computation = intersection.prepare_computations(ray, &vec![]);

        let color = world.reflected_color(&computation, 0);

        assert!(color.equals(BLACK));
    }
}