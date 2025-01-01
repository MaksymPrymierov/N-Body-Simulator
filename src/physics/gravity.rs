use crate::types::particle::Particle;
use vecmath::vec3_normalized;
use vecmath::vec3_scale;
use vecmath::vec3_square_len;
use vecmath::vec3_sub;
use vecmath::Vector3;

// gravitational constant in SI units
pub const G: f64 = 6.67430e-11;

// The force of interaction between two bodies is directly proportional to the mass of each body F = G * (m1 * m2) / r^2
pub fn gravitational_force(p1: &Particle, p2: &Particle) -> Vector3<f64> {
    // vector from p1 to p2
    let r_vec = vec3_sub::<f64>(p2.pos(), p1.pos());

    // Square of the distance between two particles
    let distance_sq = vec3_square_len::<f64>(r_vec);

    // Return a zero vector to avoid division by zero
    if distance_sq < f64::EPSILON {
        return [0.0, 0.0, 0.0];
    }

    // Gravity force betweed two particles
    let force_mag = G * (p1.mass() * p2.mass()) / distance_sq;

    // Force direction
    let direction = vec3_normalized::<f64>(r_vec);

    // Force vector for p1
    vec3_scale::<f64>(direction, force_mag)
}
