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

    pub fn is_empty(&self) -> bool {
        self.particles.len() == 0
    }

    pub fn remove(&mut self, idx: usize) -> Option<Particle> {
        self.particles.swap_remove(idx)
    }
}

