use super::{particle::Particle, fixed_vec::FixedVec};

pub struct Voxel<const N: usize> {
    pub particles: FixedVec<Particle, N>,
}

impl<const N: usize> Voxel<N> {
    pub const fn new() -> Self {
        Self { particles: FixedVec::new() }
    }

    pub fn is_full(&self) -> bool {
        self.particles.len() >= self.particles.capacity()
    }
}

