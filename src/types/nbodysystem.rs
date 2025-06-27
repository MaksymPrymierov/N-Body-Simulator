use super::particle::Particle;
use crate::physics::gravity::gravitational_force;
use vecmath::{vec3_add, Vector3};

#[derive(Default)]
pub struct NBodySystem {
    m_particles: Vec<Particle>,
}

impl NBodySystem {
    pub fn add_particle(&mut self, particle: Particle) {
        self.m_particles.push(particle);
    }

    pub fn add_random_particle(&mut self) -> u64 {
        let particle: Particle = Particle::generate_random();

        let part_id = particle.id();

        self.m_particles.push(particle);

        part_id
    }

    pub fn get_particle_by_id(&mut self, id: u64) -> Option<&mut Particle> {
        self.m_particles
            .iter_mut()
            .find(|particle| particle.id() == id)
    }

    pub fn get_particle_by_index(&mut self, index: usize) -> Option<&mut Particle> {
        if index >= self.m_particles.len() {
            return None;
        }

        Some(&mut self.m_particles[index])
    }

    pub fn remove_particle_by_id(&mut self, id: u64) {
        self.m_particles.retain(|value: &Particle| value.id() != id);
    }

    pub fn remove_particle_by_index(&mut self, index: usize) {
        self.m_particles.remove(index);
    }

    pub fn len(&self) -> usize {
        self.m_particles.len()
    }

    pub fn is_empty(&self) -> bool {
        self.m_particles.is_empty()
    }

    pub fn compute_all_forces(&self) -> Vec<Vector3<f64>> {
        if self.m_particles.is_empty() {
            println!("No particles in system");
            return Default::default();
        }

        let mut gravity_forces: Vec<Vector3<f64>> =
            vec![Default::default(); self.m_particles.len()];
        for (body1_counter, body1) in self.m_particles.iter().clone().enumerate() {
            let mut body2_counter = body1_counter + 1;
            for body2 in self.m_particles.iter().skip(body2_counter).clone() {
                let gravy_force = gravitational_force(body1, body2);
                gravity_forces[body1_counter] =
                    vec3_add(gravity_forces[body1_counter], gravy_force);
                gravity_forces[body2_counter] = vec3_add(
                    gravity_forces[body2_counter],
                    vecmath::vec3_neg(gravy_force),
                );
                body2_counter += 1;
            }
        }
        gravity_forces
    }
}
