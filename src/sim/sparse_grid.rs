use std::collections::HashMap;

use super::{particle::Particle, voxel::Voxel};

pub struct SparseGrid<const N: usize> {
    voxels: HashMap<(i32, i32, i32), Voxel<N>>,
}

impl<const N: usize> SparseGrid<N> {
    pub fn new() -> Self {
        Self { voxels: HashMap::new() }
    }

    pub fn insert(&mut self, particle: Particle) -> bool {
        let key = (
            particle.position.x().floor() as i32,
            particle.position.y().floor() as i32,
            particle.position.z().floor() as i32,
        );
        let voxel = self.voxels.entry(key).or_insert_with(Voxel::new);
        voxel.particles.push(particle).is_ok()
    }

    pub fn voxel(&self, key: (i32, i32, i32)) -> Option<&Voxel<N>> {
        self.voxels.get(&key)
    }

    pub fn voxel_count(&self) -> usize {
        self.voxels.len()
    }
}
