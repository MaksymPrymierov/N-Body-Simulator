use crate::types::nbodysystem::NBodySystem;
use crate::types::particle::Particle;

pub mod physics;
pub mod types;

fn main() {
    let mut system = NBodySystem::default();

    const PARTICLE_MASS: f64 = 1.5e14;
    system.add_particle(Particle::new(
        0,
        [-50.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        PARTICLE_MASS,
    ));
    system.add_particle(Particle::new(
        1,
        [50.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        PARTICLE_MASS,
    ));
    system.add_particle(Particle::new(
        2,
        [100.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        PARTICLE_MASS,
    ));

    let forces = system.compute_all_forces();
    println!("{:?}", forces);

    for (i, force) in forces.iter().enumerate().take(3) {
        let p = system.get_particle_by_index(i).expect("REASON");
        p.update_particle_euler(*force, 0.1);
        println!("Velocity result {:?}", p.velocity());
        println!("     Pos result {:?}", p.pos());
    }
}
