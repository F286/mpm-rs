mod sim;

use std::io::{self, Write};

use sim::{vec3::Vec3, particle::Particle, simulation::Simulation};

const MAX_PARTICLES_PER_VOXEL: usize = 32;

fn main() {
    let mut sim: Simulation<MAX_PARTICLES_PER_VOXEL> =
        Simulation::new(0.1, Vec3::new(0.0, -9.81, 0.0));

    // initial particle
    sim.add_particle(Particle::new(Vec3::new(0.0, 4.0, 0.0), Vec3::zero(), 1.0));

    println!("Simple MPM simulation. Commands: step [n], add x y, quit");
    loop {
        sim.display(-10..=10, -5..=5, 0);
        print!("command> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }
        let mut parts = input.split_whitespace();
        match parts.next() {
            Some("step") => {
                let n: u32 = parts.next().unwrap_or("1").parse().unwrap_or(1);
                for _ in 0..n {
                    sim.step();
                }
            }
            Some("add") => {
                if let (Some(x), Some(y)) = (parts.next(), parts.next()) {
                    if let (Ok(xv), Ok(yv)) = (x.parse::<f32>(), y.parse::<f32>()) {
                        sim.add_particle(Particle::new(
                            Vec3::new(xv, yv, 0.0),
                            Vec3::zero(),
                            1.0,
                        ));
                    }
                }
            }
            Some("quit") | Some("exit") => break,
            _ => {
                println!("commands: step [n], add x y, quit");
            }
        }
    }
}
