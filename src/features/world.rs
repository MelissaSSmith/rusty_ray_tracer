use uuid::Uuid;
use crate::features::color::Color;
use crate::features::color::consts::{BLACK, WHITE};
use crate::features::computation::Computation;
use crate::features::intersection::Intersection;
use crate::features::light::PointLight;
use crate::features::material::Material;
use crate::features::primitives::matrix::Matrix;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::ray::Ray;
use crate::features::shapes::Intersect;
use crate::features::shapes::shape::{Object, Shape};

#[derive(Clone)]
pub struct World {
    objects: Vec<Object>,
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
    
    pub fn create_world(light: PointLight, objects: Vec<Object>) -> Self {
        Self {
            objects,
            light: Some(light),
            recursion_limit: 5
        }
    }

    pub fn create_default() -> Self {
        let m1 = Material::create()
            .with_color(Color::create(0.8, 1.0, 0.6))
            .with_diffuse(0.7)
            .with_specular(0.2);
        let s1 = Shape::Sphere.create()
            .with_material(m1);
        let s2 = Shape::Sphere.create()
            .with_transform(Matrix::scale(0.5, 0.5, 0.5));
        let light = PointLight::create(WHITE, Point::create(-10.0, 10.0, -10.0));
        Self {
            light: Some(light),
            objects: vec![s1, s2],
            recursion_limit: 5
        }
    }

    pub fn objects(self) -> Vec<Object> {
        self.objects
    }

    pub fn light(self) -> Option<PointLight> {
        self.light
    }

    pub fn set_light(&mut self, light: PointLight) {
        self.light = Some(light);
    }

    pub fn set_object(&mut self, index: usize, object: Object) {
        let _ = self.objects.remove(index);
        self.objects.insert(index, object);
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object)
    }

    pub fn with_objects(self, objects: Vec<Object>) -> World {
        World {
            objects,
            ..self
        }
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        self.color_at_impl(ray, self.recursion_limit)
    }

    pub fn get_object_by_id(&self, id: Uuid) -> Option<Object> {
        for object in &self.objects {
            if object.id() == id {
                return Some(object.clone());
            }

            if let Some(container) = object.get_object_by_id(id) {
                return Some(container);
            }
        }

        None
    }

    fn intersect(&self, _ray: Ray) -> Vec<Intersection>{
        let mut intersections = vec![];
        for object in self.objects.iter() {
            let object_intersections = Object::intersect(object, &_ray);
            for intersection in object_intersections {
                intersections.push(intersection);
            }
        }
        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        intersections
    }

    fn shade_hit(&self, computation: &Computation, remaining: u8) -> Color {
        let shadowed = self.is_shadowed(computation.over_point());

        let surface_color = computation.clone().object().material().lighting(
            &self.light.unwrap(),
            &computation.clone().object(),
            &computation.over_point(),
            &computation.eye_vector(),
            &computation.normal_vector(),
            shadowed
        );
        let reflected = self.reflected_color(&computation, remaining);
        let refracted = self.refracted_color(&computation, remaining);

        let material = &computation.clone().object().material();
        if material.reflective() > 0.0 && material.transparency() > 0.0 {
            let reflectance = Intersection::schlick(computation);

            return  surface_color + reflected * reflectance + refracted * (1.0 - reflectance);
        }

        surface_color + reflected + refracted
    }

    fn reflected_color(&self, _computations: &Computation, remaining: u8) -> Color {
        let reflective = _computations.clone().object().material().reflective();
        if remaining == 0 {
            return BLACK;
        }
        let reflect_ray = Ray::create(_computations.over_point(), _computations.reflect_vector());
        let color = self.color_at_impl(&reflect_ray, remaining - 1);

        color * reflective
    }

    fn refracted_color(&self, _computations: &Computation, remaining: u8) -> Color {
        let transparency = _computations.clone().object().material().transparency();
        if transparency == 0.0 || remaining == 0 {
            return BLACK;
        }

        let n_ratio = _computations.n1() / _computations.n2();
        let cos_i = _computations.eye_vector() ^ _computations.normal_vector();
        let sin2_t = n_ratio * n_ratio * (1.0 - cos_i * cos_i);
        if sin2_t > 1.0 {
            return BLACK;
        }
        let cos_t = f64::sqrt(1.0 - sin2_t);

        let direction = (_computations.normal_vector() * (n_ratio * cos_i - cos_t)) -
            (_computations.eye_vector() * n_ratio);
        let refract_ray = Ray::create(_computations.under_point(), direction);
        let color = self.color_at_impl(&refract_ray, remaining-1);

        color * transparency
    }

    fn color_at_impl(&self, _ray: &Ray, remaining: u8) -> Color {
        let intersections = self.intersect(*_ray);
        let intersection = Intersection::hit(intersections.clone());
        return match intersection {
            None => { BLACK }
            Some(i) => {
                let computations = i.prepare_computations(*_ray, &intersections);
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
                        h.t < distance && h.object.has_shadow()
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
    use crate::features::patterns::EmptyCreate;
    use crate::features::patterns::test::TestPattern;
    use crate::features::primitives::matrix::Matrix;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;
    use crate::features::ray::Ray;
    use crate::features::primitives::vector::Vector;
    use crate::features::shapes::shape::Shape;
    use crate::features::world::World;

    #[test]
    fn test_create_a_world() {
        let world = World::create();

        assert_eq!(0, world.objects.len());
        assert!(world.light.is_none())
    }

    #[test]
    fn test_create_default_world() {
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
        let intersection = Intersection::create(4.0, &world.clone().objects()[0], 0.0, 0.0);

        let computation = intersection.prepare_computations(ray, &vec![]);
        let color = world.shade_hit(&computation, 1);

        assert!(color.equals(Color::create(0.38066, 0.47583, 0.2855)));
    }

    #[test]
    fn test_shading_an_intersection_from_the_inside() {
        let mut world = World::create_default();
        world.light = Some(PointLight::create(WHITE, Point::create(0.0, 0.25, 0.0)));
        let ray = Ray::create(Point::zero(), Vector::create(0.0, 0.0, 1.0));
        let intersection = Intersection::create(0.5, &world.clone().objects()[1], 0.0, 0.0);

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
        let s1_material = Material::create()
            .with_color(Color::create(0.8, 1.0, 0.6))
            .with_diffuse(0.7)
            .with_specular(0.2)
            .with_ambient(1.0);
        let s1 = Shape::Sphere.create()
            .with_material(s1_material);
        let s2_material = Material::create()
            .with_ambient(1.0);
        let s2 = Shape::Sphere.create()
            .with_material(s2_material)
            .with_transform(Matrix::scale(0.5, 0.5, 0.5));
        let light = PointLight::create(WHITE, Point::create(-10.0, 10.0, -10.0));
        let mut world = World::create();
        world.light = Some(light);
        world.objects = vec![s1, s2];
        let ray = Ray::create(Point::create(0.0, 0.0, 0.75), Vector::create(0.0, 0.0, -1.0));

        let color = world.color_at(&ray);

        assert!(color.equals(WHITE));
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
        let s1 = Shape::Sphere.create();
        let s2 = Shape::Sphere.create()
            .with_transform(Matrix::translate(0.0, 0.0, 10.0));
        let objects = vec![s1, s2.clone()];
        let world = World::create_world(light, objects);
        let ray = Ray::create(Point::create(0.0, 0.0, 5.0), Vector::create(0.0, 0.0, 1.0));
        let intersection = Intersection::create(4.0, &s2, 0.0, 0.0);
        let computations = intersection.prepare_computations(ray, &vec![]);

        let color = world.shade_hit(&computations, 1);

        assert!(color.equals(Color::create(0.1, 0.1, 0.1)));
    }

    #[test]
    fn test_reflect_color_for_a_non_reflective_material() {
        let mut world = World::create_default();
        let ray = Ray::create(Point::zero(), Vector::create(0.0, 0.0, 1.0));
        let shape = world.clone().objects()[1].clone();
        shape.material().set_ambient(1.0);
        world.set_object(1, shape.clone());
        let intersection = Intersection::create(1.0, &shape, 0.0, 0.0);

        let computation = intersection.prepare_computations(ray, &vec![]);

        let color = world.reflected_color(&computation, 1);

        assert!(color.equals(BLACK));
    }

    #[test]
    fn test_reflect_color_for_a_reflective_material() {
        let sqrt2 = f64::sqrt(2.0);

        let material = Material::create()
            .with_reflective(0.5);
        let shape = Shape::Plane.create()
            .with_material(material)
            .with_transform(Matrix::translate(0.0, -1.0, 0.0));
        let mut world = World::create_default();
        world.add_object(shape.clone());

        let ray = Ray::create(Point::create(0.0, 0.0, -3.0), Vector::create(0.0, -sqrt2/2.0, sqrt2/2.0));
        let list = vec![Intersection::create(sqrt2, &shape, 0.0, 0.0)];
        let computation = list[0].prepare_computations(ray, &list);

        let color = world.reflected_color(&computation, 3);

        assert!(color.equals(Color::create(0.19032, 0.2379, 0.14274)));
    }

    #[test]
    fn test_shade_hit_with_a_reflective_material() {
        let sqrt2 = f64::sqrt(2.0);

        let material = Material::create()
            .with_reflective(0.5);
        let shape = Shape::Plane.create()
            .with_material(material)
            .with_transform(Matrix::translate(0.0, -1.0, 0.0));
        let mut world = World::create_default();
        world.add_object(shape.clone());

        let ray = Ray::create(Point::create(0.0, 0.0, -3.0), Vector::create(0.0, -sqrt2/2.0, sqrt2/2.0));
        let list = vec![Intersection::create(sqrt2, &shape, 0.0, 0.0)];
        let computation = list[0].prepare_computations(ray, &list);

        let color = world.shade_hit(&computation, 1);

        assert!(color.equals(Color::create(0.87677, 0.92436, 0.82918)));
    }

    #[test]
    fn test_color_at_with_mutually_reflective_surfaces() {
        let mut world = World::create();
        world.set_light(PointLight::create(WHITE, Point::zero()));

        let material = Material::create().with_reflective(1.0);

        let lower = Shape::Plane.create()
            .with_material(material.clone())
            .with_transform(Matrix::translate(0.0, -1.0, 0.0));

        let upper = Shape::Plane.create()
            .with_material(material.clone())
            .with_transform(Matrix::translate(0.0, 1.0, 0.0));

        world.add_object(lower);
        world.add_object(upper);

        let ray = Ray::create(Point::zero(), Vector::create(0.0, 1.0, 0.0));

        let _ = world.color_at(&ray);
    }

    #[test]
    fn test_reflected_color_at_maximum_recursive_depth() {
        let ray = Ray::create(Point::create(0.0, 0.0, -3.0), Vector::create(0.0, -2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0));
        let material = Material::create().with_reflective(0.5);
        let shape = Shape::Plane.create()
            .with_material(material)
            .with_transform(Matrix::translate(0.0, -1.0, 0.0));

        let mut world = World::create_default();
        world.add_object(shape.clone());

        let intersection = Intersection::create(2.0_f64.sqrt(), &shape, 0.0, 0.0);

        let computation = intersection.prepare_computations(ray, &vec![]);

        let color = world.reflected_color(&computation, 0);

        assert!(color.equals(BLACK));
    }

    #[test]
    fn test_find_the_refracted_color_of_an_opaque_object() {
        let world = World::create_default();
        let shape = &world.clone().objects()[0];
        let ray = Ray::create(Point::create(0.0, 0.0, -5.0), Vector::create(0.0, 0.0, 1.0));
        let intersections = vec![Intersection::create(4.0, shape, 0.0, 0.0), Intersection::create(6.0, shape, 0.0, 0.0)];

        let computations = intersections[0].prepare_computations(ray, &intersections);
        let color = world.refracted_color(&computations, 5);

        assert!(color.equals(BLACK));
    }

    #[test]
    fn test_find_the_refracted_color_at_the_maximum_recursion_depth() {
        let world = World::create_default();
        let mut shape = world.clone().objects()[0].clone();
        shape.set_material(Material::create()
                .with_refractive_index(1.5)
                .with_transparency(1.0)
        );
        let ray = Ray::create(Point::create(0.0, 0.0, -5.0), Vector::create(0.0, 0.0, 1.0));
        let intersections = vec![Intersection::create(4.0, &shape, 0.0, 0.0), Intersection::create(6.0, &shape, 0.0, 0.0)];

        let computations = intersections[0].prepare_computations(ray, &intersections);
        let color = world.refracted_color(&computations, 0);

        assert!(color.equals(BLACK));
    }

    #[test]
    fn test_refracted_color_when_under_total_internal_reflection() {
        let sqrt2 = f64::sqrt(2.0);
        let mut world = World::create_default();
        let mut shape = world.clone().objects()[0].clone();
        shape.set_material(Material::create()
            .with_refractive_index(1.5)
            .with_transparency(1.0)
        );
        world.set_object(0, shape.clone());
        let ray = Ray::create(Point::create(0.0, 0.0, sqrt2/2.0), Vector::create(0.0, 1.0, 0.0));
        let intersections = vec![Intersection::create(-sqrt2/2.0, &shape, 0.0, 0.0), Intersection::create(sqrt2/2.0, &shape, 0.0, 0.0)];

        let computations = intersections[1].prepare_computations(ray, &intersections);
        let color = world.refracted_color(&computations, 5);

        assert!(color.equals(BLACK));
    }

    #[test]
    fn test_refracted_color_with_a_refracted_ray() {
        let world = World::create_default();
        let mut a = world.clone().objects()[0].clone();
        a.set_material(Material::create()
            .with_ambient(1.0)
            .with_pattern(TestPattern::create())
        );
        let mut b = world.clone().objects()[1].clone();
        b.set_material(Material::create()
            .with_transparency(1.0)
            .with_refractive_index(1.5)
        );
        let world = world.with_objects(vec![a.clone(), b.clone()]);

        let ray = Ray::create(Point::create(0.0, 0.0, 0.1), Vector::create(0.0, 1.0, 0.0));
        let intersections = vec![
            Intersection::create(-0.9899, &a, 0.0, 0.0),
            Intersection::create(-0.4899, &b, 0.0, 0.0),
            Intersection::create(0.4899, &b, 0.0, 0.0),
            Intersection::create(0.9899, &a, 0.0, 0.0)
        ];

        let computations = intersections[2].prepare_computations(ray, &intersections);

        let color = world.refracted_color(&computations, 5);

        assert!(color.equals(Color::create(0.0, 0.99888, 0.04725)));
    }

    #[test]
    fn test_shade_hit_with_a_transparent_material() {
        let sqrt2 = f64::sqrt(2.0);
        let mut world = World::create_default();
        let floor = Shape::Plane.create()
            .with_transform(Matrix::translate(0.0, -1.0, 0.0))
            .with_material(Material::create()
                .with_transparency(0.5)
                .with_refractive_index(1.5)
            );
        let ball = Shape::Plane.create()
            .with_transform(Matrix::translate(0.0, -3.5, -0.5))
            .with_material(Material::create()
                .with_color(Color::create(1.0, 0.0, 0.0))
                .with_ambient(0.5)
            );
        world.add_object(floor.clone());
        world.add_object(ball.clone());

        let ray = Ray::create(Point::create(0.0, 0.0, -3.0), Vector::create(0.0, -sqrt2/2.0, sqrt2/2.0));
        let intersections = vec![Intersection::create(sqrt2, &floor, 0.0, 0.0)];
        let computations = intersections[0].prepare_computations(ray, &intersections);

        let color = world.shade_hit(&computations, 5);

        assert!(color.equals(Color::create(0.93642, 00.68642, 0.68642)));
    }

    #[test]
    fn test_shade_hit_with_a_transparent_and_reflective_material() {
        let sqrt2 = f64::sqrt(2.0);
        let mut world = World::create_default();
        let floor = Shape::Plane.create()
            .with_transform(Matrix::translate(0.0, -1.0, 0.0))
            .with_material(Material::create()
                .with_transparency(0.5)
                .with_refractive_index(1.5)
                .with_reflective(0.5)
            );
        let ball = Shape::Plane.create()
            .with_transform(Matrix::translate(0.0, -3.5, -0.5))
            .with_material(Material::create()
                .with_color(Color::create(1.0, 0.0, 0.0))
                .with_ambient(0.5)
            );
        world.add_object(floor.clone());
        world.add_object(ball.clone());

        let ray = Ray::create(Point::create(0.0, 0.0, -3.0), Vector::create(0.0, -sqrt2/2.0, sqrt2/2.0));
        let intersections = vec![Intersection::create(sqrt2, &floor, 0.0, 0.0)];
        let computations = intersections[0].prepare_computations(ray, &intersections);

        let color = world.shade_hit(&computations, 5);

        assert!(color.equals(Color::create(0.93391, 0.69643, 0.69243)));
    }
}