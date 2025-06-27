use super::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Particle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub mass: f32,
}

impl Particle {
    pub fn new(position: Vec3, velocity: Vec3, mass: f32) -> Self {
        Self { position, velocity, mass }
    }
}

