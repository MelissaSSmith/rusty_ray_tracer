use std::borrow::Borrow;
use crate::features::shapes::Shape;

#[derive(Clone)]
pub struct Intersection<T: Shape> {
    pub t: f64,
    pub object: T
}

impl<T: Shape + Shape<Item = T>> Intersection<T> {
    pub fn create(_t: f64, _shape: T) -> Intersection<T> {
        Intersection{t: _t, object: _shape}
    }

    pub fn hit(intersections: Vec<Intersection<T>>) -> Option<Intersection<T>> {
        let mut hit: Option<Intersection<T>> = None;
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

    fn equals(&self, _intersection: Intersection<T>) -> bool {
        self.t == _intersection.t && self.object.equals(_intersection.object)
    }
}

#[cfg(test)]
mod tests {
    use crate::features::intersection::Intersection;
    use crate::features::shapes::sphere::Sphere;

    #[test]
    fn test_intersection_encapsulates_t_and_object() {
        let sphere = Sphere::create();
        let intersection = Intersection::create(3.5, sphere);

        assert_eq!(intersection.t, 3.5);
    }

    #[test]
    fn test_aggregating_intersections() {
        let s = Sphere::create();
        let i1 = Intersection::create(1.0, s.clone());
        let i2 = Intersection::create(2.0, s.clone());

        let intersections = vec![i1.clone(), i2.clone()];

        assert_eq!(2, intersections.len());
        assert_eq!(1.0, intersections[0].t);
        assert_eq!(2.0, intersections[1].t);
    }

    #[test]
    fn test_hit_when_all_intersections_have_position_t() {
        let s = Sphere::create();
        let i1 = Intersection::create(1.0, s.clone());
        let i2 = Intersection::create(2.0, s.clone());

        let intersections = vec![i1.clone(), i2.clone()];
        let hit = Intersection::hit(intersections);

        assert_eq!(hit.is_some(), true);
        assert!(i1.equals(hit.unwrap()));
    }

    #[test]
    fn test_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::create();
        let i1 = Intersection::create(-1.0, s.clone());
        let i2 = Intersection::create(1.0, s.clone());

        let intersections = vec![i1.clone(), i2.clone()];
        let hit = Intersection::hit(intersections);

        assert_eq!(hit.is_some(), true);
        assert!(i2.equals(hit.unwrap()));
    }

    #[test]
    fn test_no_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::create();
        let i1 = Intersection::create(-2.0, s.clone());
        let i2 = Intersection::create(-1.0, s.clone());

        let intersections = vec![i1, i2];
        let hit = Intersection::hit(intersections);

        assert_eq!(hit.is_none(), true);
    }

    #[test]
    fn test_hit_is_always_lowest_non_negative_intersection() {
        let s = Sphere::create();
        let i1 = Intersection::create(5.0, s.clone());
        let i2 = Intersection::create(7.0, s.clone());
        let i3 = Intersection::create(-3.0, s.clone());
        let i4 = Intersection::create(2.0, s.clone());

        let intersections = vec![i1, i2, i3, i4.clone()];
        let hit = Intersection::hit(intersections);

        assert_eq!(hit.is_some(), true);
        assert!(i4.equals(hit.unwrap()));
    }
}