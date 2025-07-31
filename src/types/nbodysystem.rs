use super::particle::Particle;
use crate::physics::gravity::gravitational_force;
use rayon::prelude::*;
use tokio::sync::broadcast;
use vecmath::{Vector3, vec3_add, vec3_neg};

#[derive(Clone)]
pub enum NBodySignal {
    LimitReached { particles: Vec<Particle> },
}

pub struct NBodySystem {
    m_particles: Vec<Particle>,
    m_limit: u16,
    m_step: u16,
    signal_sender: broadcast::Sender<NBodySignal>,
}

impl Default for NBodySystem {
    fn default() -> Self {
        let (sender, _) = broadcast::channel(16);
        Self {
            m_particles: Vec::new(),
            m_limit: 1000,
            m_step: 0,
            signal_sender: sender,
        }
    }
}

impl NBodySystem {
    pub fn subscribe(&self) -> broadcast::Receiver<NBodySignal> {
        self.signal_sender.subscribe()
    }

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
        if index < self.m_particles.len() {
            self.m_particles.remove(index);
        } else {
            println!("Index out of bounds");
        }
    }

    pub fn remove_all_particles(&mut self) {
        self.m_particles.clear();
    }

    pub fn len(&self) -> usize {
        self.m_particles.len()
    }

    pub fn is_empty(&self) -> bool {
        self.m_particles.is_empty()
    }

    pub fn compute_all_forces(&mut self) -> Vec<Vector3<f64>> {
        if self.m_particles.is_empty() {
            return Default::default();
        }

        self.check_limit_reached();

        let particle_count = self.m_particles.len();

        // Each thread calculates forces independently using a local vector (`local_forces`) to store intermediate results.
        // This avoids contention on shared data structures (like Mutex or Atomic variables).
        // After all threads complete their computations, the results from each thread's local vector are merged into a single global vector (`gravity_forces`) using Rayon’s `reduce` operation.
        // This approach ensures thread safety and minimizes synchronization overhead, improving performance for large systems.
        (0..particle_count)
            .into_par_iter()
            .map(|i| {
                let mut force_on_i = Vector3::default();
                let mut local_forces = vec![Vector3::default(); particle_count];

                for (j, force_on_j) in local_forces
                    .iter_mut()
                    .enumerate()
                    .take(particle_count)
                    .skip(i + 1)
                {
                    let force = gravitational_force(&self.m_particles[i], &self.m_particles[j]);

                    // Update local forces
                    force_on_i = vec3_add(force_on_i, force);
                    *force_on_j = vec3_add(*force_on_j, vec3_neg(force));
                }
                local_forces[i] = force_on_i;

                local_forces
            })
            .reduce(
                || vec![Vector3::default(); particle_count],
                |mut acc, local_forces| {
                    // Combine local results into a single vector
                    for (i, force) in local_forces.into_iter().enumerate() {
                        acc[i] = vec3_add(acc[i], force);
                    }
                    acc
                },
            )
    }

    pub fn set_limit(&mut self, limit: u16) {
        self.m_limit = limit;
    }

    fn check_limit_reached(&mut self) {
        self.m_step += 1;
        if self.m_step >= self.m_limit {
            self.m_step = 0;

            let particles_copy = self
                .m_particles
                .par_iter()
                .map(|p| p.clone())
                .collect::<Vec<_>>();

            let _ = self.signal_sender.send(NBodySignal::LimitReached {
                particles: particles_copy,
            });
        }
    }
}
