use crate::features::color::Color;
use crate::features::lights::area_light::AreaLight;
use crate::features::lights::point_light::PointLight;
use crate::features::primitives::point::Point;
use crate::features::world::World;

pub mod point_light;
pub mod area_light;

#[derive(Clone)]
pub enum Light {
    PointLight(PointLight),
    AreaLight(AreaLight)
}

impl Light {
    pub fn create_point_light(point_light: PointLight) -> Light {
        Light::PointLight(point_light)
    }

    pub fn create_area_light(area_light: AreaLight) -> Light {
        Light::AreaLight(area_light)
    }

    pub fn positions(&self) -> &[Point] {
        match self {
            Light::PointLight(point_light) => { point_light.positions() }
            Light::AreaLight(area_light) => { area_light.positions() }
        }
    }

    pub fn intensity(&self) -> Color {
        match self {
            Light::PointLight(point_light) => { point_light.intensity }
            Light::AreaLight(area_light) => { area_light.intensity() }
        }
    }

    pub fn intensity_at(&self, point: &Point, world: &World) -> f64 {
        match self {
            Light::PointLight(point_light) => { point_light.intensity_at(point, world)}
            Light::AreaLight(area_light) => { area_light.intensity_at(point, world) }
        }
    }
}