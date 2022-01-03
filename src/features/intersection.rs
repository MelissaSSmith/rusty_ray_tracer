use crate::features::computation::Computation;
use crate::features::ray::Ray;
use crate::features::shapes::Shape;

#[derive(Clone)]
pub struct Intersection {
    pub t: f64,
    pub object: Box<dyn Shape>
}

impl Intersection {
    pub fn create(_t: f64, _shape: Box<dyn Shape>) -> Intersection {
        Intersection{t: _t, object: _shape}
    }

    pub fn hit(intersections: Vec<Intersection>) -> Option<Intersection> {
        let mut hit: Option<Intersection> = None;
        for intersection in intersections {
            let h = &hit;
            if intersection.t > 0.0 {
                if h.is_none() {
                    hit = Some(intersection);
                } else if h.is_some() && intersection.t < h.as_ref().unwrap().t {
                    hit = Some(intersection);
                }
            }
        }
        hit
    }

    pub(crate) fn prepare_computations(&self, _ray: Ray) -> Computation {
        let mut computation = Computation::create(self.t, self.object.clone());

        computation.set_point(_ray.position(self.t));
        computation.set_eye_vector(_ray.direction.negate());
        computation.set_normal_vector(self.object.normal(computation.point()));
        if computation.normal_vector().dot(computation.eye_vector()) < 0.0 {
            computation.set_inside(true);
            computation.set_normal_vector(computation.normal_vector().negate());
        }
        computation
    }

    fn equals(&self, _intersection: Intersection) -> bool {
        self.t == _intersection.t && self.object == _intersection.object
    }
}

#[cfg(test)]
mod tests {
    use crate::features::intersection::Intersection;
    use crate::features::point::Point;
    use crate::features::ray::Ray;
    use crate::features::shapes::Shape;
    use crate::features::shapes::sphere::Sphere;
    use crate::features::vector::Vector;

    #[test]
    fn test_intersection_encapsulates_t_and_object() {
        let sphere = Sphere::create();
        let intersection = Intersection::create(3.5, Box::new(sphere));

        assert_eq!(intersection.t, 3.5);
    }

    #[test]
    fn test_aggregating_intersections() {
        let s = Sphere::create();
        let i1 = Intersection::create(1.0, Box::new(s.clone()));
        let i2 = Intersection::create(2.0, Box::new(s.clone()));

        let intersections = vec![i1.clone(), i2.clone()];

        assert_eq!(2, intersections.len());
        assert_eq!(1.0, intersections[0].t);
        assert_eq!(2.0, intersections[1].t);
    }

    #[test]
    fn test_hit_when_all_intersections_have_position_t() {
        let s = Sphere::create();
        let i1 = Intersection::create(1.0, Box::new(s.clone()));
        let i2 = Intersection::create(2.0, Box::new(s.clone()));

        let intersections = vec![i1.clone(), i2.clone()];
        let hit = Intersection::hit(intersections);

        assert_eq!(hit.is_some(), true);
        assert!(i1.equals(hit.unwrap()));
    }

    #[test]
    fn test_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::create();
        let i1 = Intersection::create(-1.0, Box::new(s.clone()));
        let i2 = Intersection::create(1.0, Box::new(s.clone()));

        let intersections = vec![i1.clone(), i2.clone()];
        let hit = Intersection::hit(intersections);

        assert_eq!(hit.is_some(), true);
        assert!(i2.equals(hit.unwrap()));
    }

    #[test]
    fn test_no_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::create();
        let i1 = Intersection::create(-2.0, Box::new(s.clone()));
        let i2 = Intersection::create(-1.0, Box::new(s.clone()));

        let intersections = vec![i1, i2];
        let hit = Intersection::hit(intersections);

        assert_eq!(hit.is_none(), true);
    }

    #[test]
    fn test_hit_is_always_lowest_non_negative_intersection() {
        let s = Sphere::create();
        let i1 = Intersection::create(5.0, Box::new(s.clone()));
        let i2 = Intersection::create(7.0, Box::new(s.clone()));
        let i3 = Intersection::create(-3.0, Box::new(s.clone()));
        let i4 = Intersection::create(2.0, Box::new(s.clone()));

        let intersections = vec![i1, i2, i3, i4.clone()];
        let hit = Intersection::hit(intersections);

        assert_eq!(hit.is_some(), true);
        assert!(i4.equals(hit.unwrap()));
    }

    #[test]
    fn test_precompute_state_of_an_intersection() {
        let ray = Ray::create(Point::create(0.0, 0.0, -5.0), Vector::create(0.0, 0.0, 1.0));
        let shape = Sphere::create();
        let intersection = Intersection::create(4.0, Box::new(shape.clone()));

        let computation = intersection.prepare_computations(ray);

        assert_eq!(4.0, computation.t());
        assert!(computation.point().equals(Point::create(0.0, 0.0, -1.0)));
        assert!(computation.eye_vector().equals(Vector::create(0.0, 0.0, -1.0)));
        assert!(computation.normal_vector().equals(Vector::create(0.0, 0.0, -1.0)));
        assert!(computation.object().equals(shape.as_any()));
    }

    #[test]
    fn test_hit_when_an_intersection_occurs_on_the_outside() {
        let ray = Ray::create(Point::create(0.0, 0.0, -5.0), Vector::create(0.0, 0.0, 1.0));
        let shape = Sphere::create();
        let intersection = Intersection::create(4.0, Box::new(shape.clone()));

        let computation = intersection.prepare_computations(ray);

        assert_eq!(false, computation.inside());
    }

    #[test]
    fn test_hit_when_an_intersection_occurs_on_the_inside() {
        let ray = Ray::create(Point::create(0.0, 0.0, 0.0), Vector::create(0.0, 0.0, 1.0));
        let shape = Sphere::create();
        let intersection = Intersection::create(1.0, Box::new(shape.clone()));

        let computation = intersection.prepare_computations(ray);

        assert_eq!(true, computation.inside());
        assert!(computation.point().equals(Point::create(0.0, 0.0, 1.0)));
        assert!(computation.eye_vector().equals(Vector::create(0.0, 0.0, -1.0)));
        assert!(computation.normal_vector().equals(Vector::create(0.0, 0.0, -1.0)));
    }
}