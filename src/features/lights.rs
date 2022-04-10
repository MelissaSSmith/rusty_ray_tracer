use crate::features::color::Color;
use crate::features::lights::area_light::AreaLight;
use crate::features::lights::point_light::PointLight;
use crate::features::lights::spot_light::SpotLight;
use crate::features::primitives::point::Point;
use crate::features::world::World;

pub mod point_light;
pub mod area_light;
pub mod spot_light;

#[derive(Clone)]
pub enum Light {
    PointLight(PointLight),
    AreaLight(AreaLight),
    SpotLight(SpotLight)
}

impl Light {
    pub fn create_point_light(point_light: PointLight) -> Light {
        Light::PointLight(point_light)
    }

    pub fn create_area_light(area_light: AreaLight) -> Light {
        Light::AreaLight(area_light)
    }

    pub fn create_spot_light(spot_light: SpotLight) -> Light {
        Light::SpotLight(spot_light)
    }

    pub fn positions(&self) -> &[Point] {
        match self {
            Light::PointLight(point_light) => { point_light.positions() }
            Light::AreaLight(area_light) => { area_light.positions() }
            Light::SpotLight(spot_light) => { spot_light.positions() }
        }
    }

    pub fn intensity(&self) -> Color {
        match self {
            Light::PointLight(point_light) => { point_light.intensity }
            Light::AreaLight(area_light) => { area_light.intensity() }
            Light::SpotLight(spot_light) => { spot_light.intensity() }
        }
    }

    pub fn intensity_at(&self, point: &Point, world: &World) -> f64 {
        match self {
            Light::PointLight(point_light) => { point_light.intensity_at(point, world)}
            Light::AreaLight(area_light) => { area_light.intensity_at(point, world) }
            Light::SpotLight(spot_light) => { spot_light.intensity_at(point, world) }
        }
    }
}