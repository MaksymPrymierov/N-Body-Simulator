#![allow(dead_code)] //Delete later

use super::particle::Particle;

pub struct NBodySystem {
    m_particles: Vec<Particle>
}

impl Default for NBodySystem {
    fn default() -> Self {
        Self {
            m_particles: Default::default()
        }
    }
}

impl NBodySystem {
    pub fn add_particle(&mut self, particle: Particle) {
        self.m_particles.push(particle);
    }

    pub fn add_random_particle(&mut self) -> u64 {
        let particle: Particle = Particle::generate_random();

        let part_id = particle.id();

        self.m_particles.push(particle);

        return part_id;
    }

    pub fn get_particle_by_id(&mut self, id: u64) -> Option<&mut Particle>{
        for particle in self.m_particles.iter_mut() {
            if particle.id() == id {
                return Some(particle);
            }
        }

        None
    }

    pub fn get_particle_by_index(&mut self, index: usize) -> Option<&mut Particle> {
        if index >= self.m_particles.len()   {
            return None;
        }

        return Some(&mut self.m_particles[index]);
    }

    pub fn remove_particle_by_id(&mut self, id: u64) {
        self.m_particles.retain(|value: &Particle| value.id() != id);
    }

    pub fn remove_particle_by_index(&mut self, index: usize) {
        self.m_particles.remove(index);
    }

    pub fn len(&self) -> usize {
        return self.m_particles.len();
    }

    pub fn is_empty(&self) -> bool {
        return self.m_particles.is_empty();
    }
}
