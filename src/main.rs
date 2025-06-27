mod sim;

use sim::{vec3::Vec3, particle::Particle, sparse_grid::SparseGrid};

const MAX_PARTICLES_PER_VOXEL: usize = 32;

fn main() {
    let mut grid: SparseGrid<MAX_PARTICLES_PER_VOXEL> = SparseGrid::new();

    for i in 0..100 {
        let p = Particle::new(
            Vec3::new(i as f32 * 0.1, 0.0, 0.0),
            Vec3::zero(),
            1.0,
        );
        grid.insert(p);
    }

    println!("Inserted {} voxels", grid.voxel_count());
}
