use crate::types::nbodysystem::NBodySystem;
use crate::types::particle::Particle;
use std::time::Instant;

pub mod physics;
pub mod types;

fn main() {
    let mut system = NBodySystem::default();
    let particle_count: usize = 3;

    for i in 0..particle_count {
        system.add_particle(Particle::new(
            i as u64,
            [i as f64 * 50.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            1.5e14 * (i + 1) as f64,
        ));
    }

    let start = Instant::now();

    let forces = system.compute_all_forces();
    println!("{forces:?}");

    for (i, force) in forces.iter().enumerate().take(particle_count) {
        let p = system.get_particle_by_index(i).expect("REASON");
        p.update_particle_euler(*force, 0.1);
        println!("Velocity result {:?}", p.velocity());
        println!("     Pos result {:?}", p.pos());
    }

    let duration = start.elapsed();

    println!("Total Time: {duration:#?}.");
}
