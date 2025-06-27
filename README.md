# mpm-rs

This project contains an experimental skeleton for a Material Point Method (MPM) simulator in Rust. Particles are stored in a sparse voxel structure to reduce the cost of repeatedly converting between particle and grid representations. Each voxel holds a limited number of particles and is allocated only when needed.

The code currently provides:

- `Vec3` – a simple aligned vector type used by particles
- `Particle` – the fundamental unit of the simulation
- `FixedVec` – a small fixed-capacity container similar to `Vec`
- `Voxel` – a container for particles within a single cell
- `SparseGrid` – a hash‑map based sparse voxel grid

`main.rs` now provides a simple interactive simulation you can run with `cargo run`. The grid is printed as ASCII art and you can step the simulation or add new particles at runtime.

## Why Sparse Voxels?

MPM simulations transfer particle data to and from grid cells every step. Storing particles directly in a grid cell format helps avoid expensive scatter/gather operations. A sparse layout means that only voxels that actually contain particles consume memory.

## Future Work

This is only a starting point. To build a full simulation we plan to:

1. Add physics routines for transferring data between particles and grid nodes.
2. Provide multi-threaded and SIMD accelerated updates.
3. Extend the data structures for efficient neighbor queries and particle sorting.
4. Benchmark and optimize memory access patterns.

Contributions and suggestions are welcome.
