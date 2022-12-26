use crate::features::color::Color;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::primitives::vector::Vector;
use crate::features::world::World;

#[derive(Clone, Copy, Debug)]
pub struct SpotLight {
    center: [Point; 1],
    color: Color,
    radius_start: f64,
    radius_end: f64,
    fade: f64,
    direction: Vector
}

impl SpotLight {
    pub fn create(center: Point, intensity: Color, r1: f64, r2: f64) -> SpotLight {
        SpotLight {
            center: [center],
            color: intensity,
            radius_start: r1,
            radius_end: r2,
            fade: 45.0,
            direction: Vector::create(0.0, 0.0, 1.0)
        }
    }

    pub(crate) fn intensity(&self) -> Color {
        self.color
    }

    pub(crate) fn positions(&self) -> &[Point] {
        &self.center
    }

    pub(crate) fn intensity_at(&self, point: &Point, world: &World) -> f64 {
        let direction = (self.center[0] - *point).normalize();
        //let dot_product = self.direction ^ direction;
        let intersects_disk = self.intersects_disk(self.center[0], direction);
        if !intersects_disk {
            return 0.0;
        }
        if world.is_shadowed(self.center[0], *point) {
            return 0.0;
        }
        1.0
    }

    fn intersects_disk(&self, origin: Point, direction: Vector) -> bool {
        let a = direction.x().powi(2) - direction.y().powi(2) + direction.z().powi(2);
        let b = 2.0 * origin.x() * direction.x() -
            2.0 * origin.y() * direction.y() +
            2.0 * origin.z() * direction.z();
        let c = origin.x().powi(2) - origin.y().powi(2) + origin.z().powi(2);

        if a == 0.0 && b != 0.0 {
            return true;
        }

        let disc = b.powi(2) - 4.0 * a * c;
        if disc < 0.0 {
            return false;
        }

        let t0 = (-b - disc.sqrt()) / (2.0 * a);
        let t1 = (-b + disc.sqrt()) / (2.0 * a);

        let y0 = origin.y() + t0 * direction.y();
        if self.radius_start < y0 && y0 < self.radius_end {
            return true;
        }

        let y1 = origin.y() + t1 * direction.y();
        if self.radius_start < y1 && y1 < self.radius_end {
            return true;
        }

        let t = (self.radius_end - origin.y()) / direction.y();
        let x = origin.x() + t * direction.x();
        let z = origin.z() + t * direction.z();

        if (x.powi(2) + z.powi(2)) <= self.radius_end.powi(2) {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {}
