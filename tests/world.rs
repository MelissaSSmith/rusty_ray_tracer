use rusty_ray_tracer::features::point::Point;
use rusty_ray_tracer::features::vector::Vector;

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
    let env_vector = env.gravity.add(env.wind);
    let new_velocity = projectile.velocity.add(env_vector);
    let position = projectile.position.add(projectile.velocity);
    Projectile::create(position, new_velocity)
}