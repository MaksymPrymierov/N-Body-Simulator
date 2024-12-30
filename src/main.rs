use crate::types::particle::Particle;
use crate::physics::gravity::gravitational_force;

pub mod physics;
pub mod types;

fn main() {
    let line = "-".repeat(150);
    let body1: Particle = Particle::new(0, [-50.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 10.5e14);
    let body2: Particle = Particle::new(0, [50.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 1.5e14);

    println!("First Body: {:?}", body1);
    println!("Second Body: {:?}", body2);
    println!("{}", line);

    let force_on_first_body = gravitational_force(&body1, &body2);

    println!("Force on first object = {:?}", force_on_first_body);
    println!("Force on second object = {:?}", vecmath::vec3_neg(force_on_first_body));
}
