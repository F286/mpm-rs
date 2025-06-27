use super::{particle::Particle, sparse_grid::SparseGrid, vec3::Vec3, voxel::Voxel};

pub struct Simulation<const N: usize> {
    pub grid: SparseGrid<N>,
    gravity: Vec3,
    dt: f32,
}

impl<const N: usize> Simulation<N> {
    pub fn new(dt: f32, gravity: Vec3) -> Self {
        Self { grid: SparseGrid::new(), gravity, dt }
    }

    pub fn add_particle(&mut self, particle: Particle) {
        self.grid.insert(particle);
    }

    pub fn step(&mut self) {
        let mut moves: Vec<((i32, i32, i32), usize, (i32, i32, i32), Particle)> = Vec::new();
        for (key, voxel) in self.grid.iter_mut() {
            let len = voxel.particles.len();
            for i in 0..len {
                if let Some(p) = voxel.particles.get_mut(i) {
                    p.velocity += self.gravity * self.dt;
                    p.position += p.velocity * self.dt;
                    let new_key = (
                        p.position.x().floor() as i32,
                        p.position.y().floor() as i32,
                        p.position.z().floor() as i32,
                    );
                    if new_key != *key {
                        moves.push((*key, i, new_key, *p));
                    }
                }
            }
        }
        for (old_key, idx, new_key, particle) in moves.into_iter().rev() {
            if let Some(voxel) = self.grid.voxel_mut(old_key) {
                voxel.remove(idx);
                if voxel.is_empty() {
                    self.grid.remove_voxel(old_key);
                }
            }
            let voxel = self.grid.voxels.entry(new_key).or_insert_with(Voxel::new);
            let _ = voxel.particles.push(particle);
        }
    }

    pub fn display(&self, x_range: std::ops::RangeInclusive<i32>, y_range: std::ops::RangeInclusive<i32>, z: i32) {
        for y in y_range.clone().rev() {
            for x in x_range.clone() {
                let key = (x, y, z);
                if let Some(voxel) = self.grid.voxel(key) {
                    let count = voxel.particles.len();
                    if count > 0 {
                        print!("{}", count.min(9));
                    } else {
                        print!(".");
                    }
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}
