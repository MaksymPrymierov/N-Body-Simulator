use crate::types::particle::Particle;
use vecmath::Vector3;
use vecmath::vec3_normalized;
use vecmath::vec3_scale;
use vecmath::vec3_square_len;
use vecmath::vec3_sub;

/// Universal gravitational constant `G` in SI units (m³·kg⁻¹·s⁻²).
///
/// Value from CODATA 2018: `6.67430e-11`
pub const G: f64 = 6.67430e-11;

/// Computes the gravitational force **on `p1` due to `p2`**.
///
/// Uses the inverse-square law:
/// `F = G * (m1 * m2) / r²` along the direction from `p1` to `p2`.
///
/// # Parameters
/// - `p1`: particle experiencing the force.
/// - `p2`: source particle applying the gravitational pull.
///
/// # Returns
/// A 3D force vector acting on `p1`. If the two particles occupy the same
/// position (distance squared `< f64::EPSILON`), returns the zero vector to
/// avoid division by zero.
///
/// # Units
/// Assumes SI-like units: meters (position), seconds (time), kilograms (mass),
/// and Newtons for the returned force.
///
/// # Notes
/// - The returned vector satisfies Newton’s third law when used symmetrically:
///   `F(p1←p2) = -F(p2←p1)`.
/// - No softening term is applied; extreme masses/close separations may require
///   a softening strategy in a real simulation to improve stability.
///
/// # Complexity
/// O(1) for a single pair evaluation.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::particle::Particle;
    use vecmath::{Vector3, vec3_normalized};

    const EPS: f64 = 1e-12;

    #[inline]
    fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() <= eps
    }

    #[inline]
    fn vec_approx_eq(a: Vector3<f64>, b: Vector3<f64>, eps: f64) -> bool {
        approx_eq(a[0], b[0], eps) && approx_eq(a[1], b[1], eps) && approx_eq(a[2], b[2], eps)
    }

    #[inline]
    fn magnitude(v: Vector3<f64>) -> f64 {
        (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
    }

    fn mk_particle(id: u64, pos: Vector3<f64>, mass: f64) -> Particle {
        Particle::new(id, pos, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], mass)
    }

    #[test]
    fn zero_distance_returns_zero_vector() {
        let pos = [1.0, -2.0, 3.0];
        let p1 = mk_particle(1, pos, 5.0);
        let p2 = mk_particle(2, pos, 7.0);

        let f = gravitational_force(&p1, &p2);
        assert!(vec_approx_eq(f, [0.0, 0.0, 0.0], EPS));
    }

    #[test]
    fn correct_magnitude_and_direction_on_x_axis() {
        let m1 = 2.0;
        let m2 = 3.0;
        let p1 = mk_particle(1, [0.0, 0.0, 0.0], m1);
        let p2 = mk_particle(2, [1.0, 0.0, 0.0], m2);

        let f = gravitational_force(&p1, &p2);

        let expected_mag = G * m1 * m2 / 1.0_f64; // r=1
        let expected = [expected_mag, 0.0, 0.0];

        assert!(
            vec_approx_eq(f, expected, 1e-18),
            "f={f:?}, expected={expected:?}"
        );
        assert!(approx_eq(magnitude(f), expected_mag, 1e-18));
    }

    #[test]
    fn newtons_third_law_antisymmetry() {
        let p1 = mk_particle(1, [0.0, 0.0, 0.0], 10.0);
        let p2 = mk_particle(2, [0.0, 2.0, 0.0], 5.0);

        let f12 = gravitational_force(&p1, &p2);
        let f21 = gravitational_force(&p2, &p1);

        assert!(
            vec_approx_eq(f12, [-f21[0], -f21[1], -f21[2]], 1e-18),
            "f12={f12:?}, f21={f21:?}"
        );
    }

    #[test]
    fn inverse_square_scaling() {
        let m1 = 1.5;
        let m2 = 4.0;

        let p1_r = mk_particle(1, [0.0, 0.0, 0.0], m1);
        let p2_r = mk_particle(2, [1.0, 1.0, 1.0], m2);
        let p1_2r = mk_particle(3, [0.0, 0.0, 0.0], m1);
        let p2_2r = mk_particle(4, [2.0, 2.0, 2.0], m2);

        let f_r = gravitational_force(&p1_r, &p2_r);
        let f_2r = gravitational_force(&p1_2r, &p2_2r);

        let mag_r = magnitude(f_r);
        let mag_2r = magnitude(f_2r);

        assert!(
            approx_eq(mag_r, 4.0 * mag_2r, 1e-12),
            "Expected |F(r)| ≈ 4*|F(2r)|, got |F(r)|={mag_r}, |F(2r)|={mag_2r}"
        );

        let dir_r = vec3_normalized::<f64>(f_r);
        let dir_2r = vec3_normalized::<f64>(f_2r);
        assert!(
            vec_approx_eq(dir_r, dir_2r, 1e-12),
            "Directions differ: {dir_r:?} vs {dir_2r:?}"
        );
    }

    #[test]
    fn force_is_zero_when_particles_overlap() {
        let p1 = Particle::new(1, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 5.0);
        let p2 = Particle::new(2, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 10.0);

        let f = gravitational_force(&p1, &p2);

        assert_eq!(f, [0.0, 0.0, 0.0], "Force must be zero at r=0");
    }
}
