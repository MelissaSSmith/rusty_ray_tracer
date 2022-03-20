use crate::features::color::Color;
use crate::features::color::consts::BLACK;
use crate::features::lights::area_light::AreaLight;
use crate::features::lights::point_light::PointLight;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;

pub mod point_light;
pub mod area_light;

#[derive(Clone, Copy)]
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

    pub fn position(&self) -> Point {
        match self {
            Light::PointLight(point_light) => { point_light.position }
            Light::AreaLight(_) => { Point::zero() }
        }
    }

    pub fn intensity(&self) -> Color {
        match self {
            Light::PointLight(point_light) => { point_light.intensity }
            Light::AreaLight(_) => { BLACK }
        }
    }
}