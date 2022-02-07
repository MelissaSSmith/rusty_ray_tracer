use crate::features::computation::Computation;
use crate::features::primitives::operations::consts::{EPSILON, LOW_EPSILON};
use crate::features::ray::Ray;
use smallvec::SmallVec;
use crate::features::shapes::{Normal, NormalAt};
use crate::features::shapes::shape::Object;

#[derive(Clone)]
pub struct Intersection {
    pub t: f64,
    pub object: Object
}

impl Intersection {
    pub fn create(_t: f64, _shape: &Object) -> Intersection {
        Intersection{t: _t, object: _shape.clone() }
    }

    pub fn hit(intersections: Vec<Intersection>) -> Option<Intersection> {
        let mut hit: Option<Intersection> = None;
        for intersection in intersections {
            let h = &hit;
            if intersection.t > 0.0001 {
                if h.is_none() {
                    hit = Some(intersection);
                } else if h.is_some() && intersection.t < h.as_ref().unwrap().t {
                    hit = Some(intersection);
                }
            }
        }
        hit
    }

    pub(crate) fn prepare_computations(&self, _ray: Ray, _intersections: &Vec<Intersection>) -> Computation {
        let mut computation = Computation::create(self.t, &self.object);

        let point = _ray.position(self.t);
        let normal = Object::normal(&self.object, &point, None);
        let eye_vector = -_ray.direction();

        let (n1, n2) = Intersection::calculate_n1_and_n2(_intersections, self.clone());

        let (inside, normal) = if normal ^ eye_vector < 0.0 {
            (true, -normal)
        } else {
            (false, normal)
        };

        computation.set_n1(n1);
        computation.set_n2(n2);
        computation.set_cos_i(eye_vector ^ normal);
        computation.set_point(point);
        computation.set_eye_vector(eye_vector);
        computation.set_normal_vector(normal);
        computation.set_inside(inside);
        computation.set_reflect_vector(_ray.direction.reflect(normal));
        computation.set_over_point(point + normal * EPSILON);
        computation.set_under_point(point - normal * EPSILON);

        computation
    }

    fn calculate_n1_and_n2(_intersections: &Vec<Intersection>, hit: Intersection) -> (f64, f64) {
        let mut n1 = 0.0;
        let mut n2 = 0.0;

        let mut containers = SmallVec::<[&Object; 32]>::new();
        for intersection in _intersections {
            let is_intersection = intersection.equals(hit.clone());

            if is_intersection {
                n1 = Intersection::grab_container_value(&containers);
            }
            match containers
                .iter()
                .position(|&object| object.equals(&intersection.object))
            {
                Some(pos) => {
                    let _ = containers.remove(pos);
                }
                None => {
                    containers.push(&intersection.object)
                },
            }

            if is_intersection {
                n2 = Intersection::grab_container_value(&containers);
                break;
            }
        }
        (n1, n2)
    }

    fn grab_container_value(containers: &SmallVec<[&Object; 32]>) -> f64 {
        if containers.is_empty() {
            return 1.0;
        }
        containers.last().unwrap().material().refractive_index()
    }


    pub fn schlick(computations: &Computation) -> f64 {
        let n1 = computations.n1();
        let n2 = computations.n2();

        let mut cos = computations.cos_i();

        if n1 > n2 {
            let n = n1 / n2;
            let sin2_t = n.powi(2) * (1.0 - cos.powi(2));
            if sin2_t > 1.0 {
                return 1.0;
            }
            cos = (1.0 - sin2_t).sqrt();
        }

        let r0 = ((n1 - n2) / (n1 + n2)).powi(2);

        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }

    fn equals(&self, _intersection: Intersection) -> bool {
        self.t == _intersection.t &&
            self.object.equals(&_intersection.object)
    }
}

#[cfg(test)]
mod tests {
    use crate::features::computation::Computation;
    use crate::features::intersection::Intersection;
    use crate::features::material::Material;
    use crate::features::primitives::matrix::Matrix;
    use crate::features::primitives::operations::consts::EPSILON;
    use crate::features::primitives::operations::Operations;
    use crate::features::primitives::point::Point;
    use crate::features::ray::Ray;
    use crate::features::primitives::tuple_trait::Tuple;
    use crate::features::primitives::vector::Vector;
    use crate::features::shapes::shape::Shape;

    #[test]
    fn test_intersection_encapsulates_t_and_object() {
        let intersection = Intersection::create(3.5, &Shape::Sphere.create());

        assert_eq!(intersection.t, 3.5);
    }

    #[test]
    fn test_aggregating_intersections() {
        let s = Shape::Sphere.create();
        let i1 = Intersection::create(1.0, &s);
        let i2 = Intersection::create(2.0, &s);

        let intersections = vec![i1.clone(), i2.clone()];

        assert_eq!(2, intersections.len());
        assert_eq!(1.0, intersections[0].t);
        assert_eq!(2.0, intersections[1].t);
    }

    #[test]
    fn test_hit_when_all_intersections_have_position_t() {
        let s = Shape::Sphere.create();
        let i1 = Intersection::create(1.0, &s);
        let i2 = Intersection::create(2.0, &s);

        let intersections = vec![i1.clone(), i2.clone()];
        let hit = Intersection::hit(intersections);

        assert_eq!(hit.is_some(), true);
        assert!(i1.equals(hit.unwrap()));
    }

    #[test]
    fn test_hit_when_some_intersections_have_negative_t() {
        let s = Shape::Sphere.create();
        let i1 = Intersection::create(-1.0, &s);
        let i2 = Intersection::create(1.0, &s);

        let intersections = vec![i1.clone(), i2.clone()];
        let hit = Intersection::hit(intersections);

        assert_eq!(hit.is_some(), true);
        assert!(i2.equals(hit.unwrap()));
    }

    #[test]
    fn test_no_hit_when_all_intersections_have_negative_t() {
        let s = Shape::Sphere.create();
        let i1 = Intersection::create(-2.0, &s);
        let i2 = Intersection::create(-1.0, &s);

        let intersections = vec![i1, i2];
        let hit = Intersection::hit(intersections);

        assert_eq!(hit.is_none(), true);
    }

    #[test]
    fn test_hit_is_always_lowest_non_negative_intersection() {
        let s = Shape::Sphere.create();
        let i1 = Intersection::create(5.0, &s);
        let i2 = Intersection::create(7.0, &s);
        let i3 = Intersection::create(-3.0, &s);
        let i4 = Intersection::create(2.0, &s);

        let intersections = vec![i1, i2, i3, i4.clone()];
        let hit = Intersection::hit(intersections);

        assert_eq!(hit.is_some(), true);
        assert!(i4.equals(hit.unwrap()));
    }

    #[test]
    fn test_precompute_state_of_an_intersection() {
        let ray = Ray::create(Point::create(0.0, 0.0, -5.0), Vector::create(0.0, 0.0, 1.0));
        let shape = Shape::Sphere.create();
        let intersection = Intersection::create(4.0, &shape);

        let computation = intersection.prepare_computations(ray, &vec![]);

        assert_eq!(4.0, computation.t());
        assert!(computation.point().equals(Point::create(0.0, 0.0, -1.0)));
        assert!(computation.eye_vector().equals(Vector::create(0.0, 0.0, -1.0)));
        assert!(computation.normal_vector().equals(Vector::create(0.0, 0.0, -1.0)));
    }

    #[test]
    fn test_hit_when_an_intersection_occurs_on_the_outside() {
        let ray = Ray::create(Point::create(0.0, 0.0, -5.0), Vector::create(0.0, 0.0, 1.0));
        let shape = Shape::Sphere.create();
        let intersection = Intersection::create(4.0, &shape);

        let computation = intersection.prepare_computations(ray, &vec![]);

        assert_eq!(false, computation.inside());
    }

    #[test]
    fn test_hit_when_an_intersection_occurs_on_the_inside() {
        let ray = Ray::create(Point::zero(), Vector::create(0.0, 0.0, 1.0));
        let shape = Shape::Sphere.create();
        let intersection = Intersection::create(1.0, &shape);

        let computation = intersection.prepare_computations(ray, &vec![]);

        assert_eq!(true, computation.inside());
        assert!(computation.point().equals(Point::create(0.0, 0.0, 1.0)));
        assert!(computation.eye_vector().equals(Vector::create(0.0, 0.0, -1.0)));
        assert!(computation.normal_vector().equals(Vector::create(0.0, 0.0, -1.0)));
    }

    #[test]
    fn test_hit_should_offset_the_point() {
        let ray = Ray::create(Point::create(0.0, 0.0, -5.0), Vector::create(0.0, 0.0, 1.0));
        let mut sphere = Shape::Sphere.create();
        sphere.set_transform(Matrix::translate(0.0, 0.0, 1.0));

        let intersection = Intersection::create(5.0, &sphere);

        let computation = intersection.prepare_computations(ray, &vec![]);

        assert!(computation.over_point().z() < -EPSILON/2.0);
        assert!(computation.point().z() > computation.over_point().z());
    }

    #[test]
    fn test_precompute_the_reflection_vector() {
        let sqrt_2 = f64::sqrt(2.0);
        let shape = Shape::Plane.create();
        let ray = Ray::create(Point::create(0.0, 1.0, -1.0), Vector::create(0.0, -sqrt_2/2.0, sqrt_2/2.0));
        let intersection = Intersection::create(sqrt_2, &shape);

        let computation = intersection.prepare_computations(ray, &vec![]);

        let reflect = computation.reflect_vector();
        assert!(reflect.equals(Vector::create(0.0, sqrt_2/2.0, sqrt_2/2.0)));
    }

    #[test]
    fn test_n1_and_n2_at_various_intersections() {
        let mut a = Shape::Sphere.glass();
        a.set_transform(Matrix::scale(2.0, 2.0, 2.0));
        a.set_material(Material::create().with_refractive_index(1.5));
        let mut b = Shape::Sphere.glass();
        b.set_transform(Matrix::translate(0.0, 0.0, -0.25));
        b.set_material(Material::create().with_refractive_index(2.0));
        let mut c = Shape::Sphere.glass();
        c.set_transform(Matrix::translate(0.0, 0.0, 0.25));
        c.set_material(Material::create().with_refractive_index(2.5));

        let ray = Ray::create(Point::create(0.0, 0.0, -4.0), Vector::create(0.0, 0.0, 1.0));
        let intersections = vec![
            Intersection::create(2.0, &a),
            Intersection::create(2.75, &b),
            Intersection::create(3.25, &c),
            Intersection::create(4.75, &b),
            Intersection::create(5.25, &c),
            Intersection::create(6.0, &a)
        ];

        let mut computations = Vec::<Computation>::new();
        for (index, intersection) in intersections.iter().enumerate() {
            let comp = intersection.prepare_computations(ray, &intersections);
            computations.push(comp);
        }

        assert_eq!(1.0, computations[0].n1());
        assert_eq!(1.5, computations[0].n2());

        assert_eq!(1.5, computations[1].n1());
        assert_eq!(2.0, computations[1].n2());

        assert_eq!(2.0, computations[2].n1());
        assert_eq!(2.5, computations[2].n2());

        assert_eq!(2.5, computations[3].n1());
        assert_eq!(2.5, computations[3].n2());

        assert_eq!(2.5, computations[4].n1());
        assert_eq!(1.5, computations[4].n2());

        assert_eq!(1.5, computations[5].n1());
        assert_eq!(1.0, computations[5].n2());
    }

    #[test]
    fn test_under_point_is_offset_below_the_surface() {
        let ray = Ray::create(Point::create(0.0, 0.0, 0.5), Vector::create(0.0, 0.0, 1.0));
        let shape = Shape::Sphere.glass().with_transform(Matrix::translate(0.0, 0.0, 1.0));

        let intersection = Intersection::create(5.0, &shape);
        let intersections = vec![intersection.clone()];

        let computation = intersection.prepare_computations(ray, &intersections);

        assert!(computation.under_point().z() > EPSILON/2.0);
        assert!(computation.point().z() < computation.under_point().z());
    }


    #[test]
    fn test_determine_reflectance_under_total_internal_reflection() {
        let sqrt2 = f64::sqrt(2.0);
        let shape = Shape::Sphere.glass();
        let ray = Ray::create(Point::create(0.0, 0.0, sqrt2/2.0), Vector::create(0.0, 1.0, 0.0));
        let intersections = vec![Intersection::create(-sqrt2/2.0, &shape), Intersection::create(sqrt2/2.0, &shape)];
        let computations = intersections[1].prepare_computations(ray, &intersections);

        let reflectance = Intersection::schlick(&computations);

        assert_eq!(reflectance, 1.0);
    }

    #[test]
    fn test_determine_reflectance_of_a_perpendicular_ray() {
        let sqrt2 = f64::sqrt(2.0);
        let shape = Shape::Sphere.glass();
        let ray = Ray::create(Point::zero(), Vector::create(0.0, 1.0, 0.0));
        let intersections = vec![Intersection::create(-1.0, &shape), Intersection::create(1.0, &shape)];
        let computations = intersections[1].prepare_computations(ray, &intersections);

        let reflectance = Intersection::schlick(&computations);

        assert!(reflectance.equals(0.04));
    }

    #[test]
    fn test_determine_reflectance_when_n2_is_greater_than_n1_and_small_angle() {
        let shape = Shape::Sphere.glass();
        let ray = Ray::create(Point::create(0.0, 0.99, -2.0), Vector::create(0.0, 0.0, 1.0));
        let intersections = vec![Intersection::create(1.8589, &shape)];
        let computations = intersections[0].prepare_computations(ray, &intersections);

        let reflectance = Intersection::schlick(&computations);

        assert!(reflectance.equals(0.48873));
    }
}