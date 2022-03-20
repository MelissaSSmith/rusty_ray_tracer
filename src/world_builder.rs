use crate::features::color::consts::WHITE;
use crate::features::lights::point_light::PointLight;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::shapes::shape::Object;
use crate::features::world::World;

pub fn create_default_world_with_group(group: Object) -> World {
    let light_source = PointLight::create(WHITE, Point::create(-5.0, 25.0, -15.0));
    let objects = vec![group];
    World::create_world(light_source, objects)
}