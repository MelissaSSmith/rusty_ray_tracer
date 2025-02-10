use linear_algebra::point::Point;
use linear_algebra::vector::Vector;

pub struct Projectile {
    pub position: Point,
    pub velocity: Vector
}

impl Projectile {
    pub fn create(position: Point, velocity: Vector) -> Projectile {
        Projectile{position, velocity}
    }
}

#[derive(Clone, Copy)]
pub struct Environment {
    gravity: Vector,
    wind: Vector
}

impl Environment {
    pub fn create(gravity: Vector, wind: Vector) -> Environment {
        Environment{gravity, wind}
    }
}

pub fn tick(env: Environment, projectile: Projectile) -> Projectile {
    let env_vector = env.gravity + env.wind;
    let new_velocity = projectile.velocity + env_vector;
    let position = projectile.position + projectile.velocity;
    Projectile::create(position, new_velocity)
}