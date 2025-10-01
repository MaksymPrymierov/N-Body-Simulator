use super::particle::Particle;
use crate::physics::gravity::gravitational_force;
use rayon::prelude::*;
use tokio::sync::broadcast;
use vecmath::{Vector3, vec3_add, vec3_neg};

#[derive(Clone, Debug)]
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

impl From<Vec<Particle>> for NBodySystem {
    fn from(m_particles: Vec<Particle>) -> Self {
        let (sender, _) = broadcast::channel(16);
        Self {
            m_limit: m_particles.len() as u16,
            m_particles,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::gravity::G;
    use crate::types::particle::Particle;
    use tokio::sync::broadcast::error::TryRecvError;
    use vecmath::{
        Vector3, vec3_add, vec3_neg, vec3_normalized, vec3_scale, vec3_square_len, vec3_sub,
    };

    const EPS: f64 = 1e-12;

    fn mk_particle(id: u64, pos: Vector3<f64>, mass: f64) -> Particle {
        Particle::new(id, pos, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], mass)
    }

    fn approx(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() <= eps
    }
    fn vapprox(a: Vector3<f64>, b: Vector3<f64>, eps: f64) -> bool {
        approx(a[0], b[0], eps) && approx(a[1], b[1], eps) && approx(a[2], b[2], eps)
    }
    fn vmag(v: Vector3<f64>) -> f64 {
        (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
    }

    fn compute_forces_serial(parts: &[Particle]) -> Vec<Vector3<f64>> {
        let n = parts.len();
        let mut forces = vec![[0.0, 0.0, 0.0]; n];
        for i in 0..n {
            for j in (i + 1)..n {
                let r_vec = vec3_sub(parts[j].pos(), parts[i].pos());
                let dist_sq = vec3_square_len::<f64>(r_vec);
                if dist_sq < f64::EPSILON {
                    continue;
                }
                let f_mag = G * (parts[i].mass() * parts[j].mass()) / dist_sq;
                let dir = vec3_normalized::<f64>(r_vec);
                let f = vec3_scale::<f64>(dir, f_mag);
                forces[i] = vec3_add(forces[i], f);
                forces[j] = vec3_add(forces[j], vec3_neg(f));
            }
        }
        forces
    }

    #[test]
    fn default_is_empty_and_forces_empty() {
        let mut sys = NBodySystem::default();
        assert_eq!(sys.len(), 0);
        assert!(sys.is_empty());

        let f = sys.compute_all_forces();
        assert!(f.is_empty());
    }

    #[test]
    fn from_vec_initializes_len_and_limit() {
        let parts = vec![
            mk_particle(1, [0.0, 0.0, 0.0], 1.0),
            mk_particle(2, [1.0, 0.0, 0.0], 1.0),
            mk_particle(3, [0.0, 1.0, 0.0], 1.0),
        ];
        let sys = NBodySystem::from(parts.clone());
        assert_eq!(sys.len(), 3);
    }

    #[test]
    fn add_get_remove_particles_work() {
        let mut sys = NBodySystem::default();

        let p1 = mk_particle(1, [0.0, 0.0, 0.0], 2.0);
        let p2 = mk_particle(2, [1.0, 0.0, 0.0], 3.0);

        sys.add_particle(p1);
        sys.add_particle(p2);

        assert_eq!(sys.len(), 2);
        assert!(!sys.is_empty());

        let got2 = sys.get_particle_by_id(2).unwrap();
        assert!(approx(got2.mass(), 3.0, EPS));

        let got0 = sys.get_particle_by_index(0).unwrap();
        assert!(vapprox(got0.pos(), [0.0, 0.0, 0.0], EPS));

        sys.remove_particle_by_id(1);
        assert_eq!(sys.len(), 1);
        assert!(sys.get_particle_by_id(1).is_none());

        sys.remove_particle_by_index(0);
        assert_eq!(sys.len(), 0);

        sys.remove_particle_by_index(123);
        assert_eq!(sys.len(), 0);
    }

    #[test]
    fn add_random_particle_returns_existing_id_and_increments_len() {
        let mut sys = NBodySystem::default();
        let before = sys.len();
        let new_id = sys.add_random_particle();
        assert_eq!(sys.len(), before + 1);
        assert!(sys.get_particle_by_id(new_id).is_some());
    }

    #[test]
    fn single_particle_has_zero_force() {
        let mut sys = NBodySystem::default();
        sys.add_particle(mk_particle(1, [0.0, 0.0, 0.0], 5.0));
        let f = sys.compute_all_forces();
        assert_eq!(f.len(), 1);
        assert!(vapprox(f[0], [0.0, 0.0, 0.0], 1e-18));
    }

    #[test]
    fn two_particles_newtons_third_law_and_expected_magnitude() {
        let mut sys = NBodySystem::default();
        let m1 = 2.0;
        let m2 = 3.0;
        let r = 1.0;
        sys.add_particle(mk_particle(1, [0.0, 0.0, 0.0], m1));
        sys.add_particle(mk_particle(2, [r, 0.0, 0.0], m2));

        let f = sys.compute_all_forces();
        assert_eq!(f.len(), 2);

        let expected_mag = G * m1 * m2 / (r * r);
        assert!(approx(vmag(f[0]), expected_mag, 1e-18));
        assert!(vapprox(f[1], vec3_neg(f[0]), 1e-18));
    }

    #[test]
    fn parallel_algorithm_matches_serial_for_three_particles() {
        let parts = vec![
            mk_particle(1, [0.0, 0.0, 0.0], 2.0),
            mk_particle(2, [1.0, 0.0, 0.0], 3.0),
            mk_particle(3, [0.0, 1.0, 0.0], 4.0),
        ];
        let mut sys = NBodySystem::from(parts.clone());

        let par = sys.compute_all_forces();
        let ser = compute_forces_serial(&parts);

        assert_eq!(par.len(), ser.len());
        for i in 0..par.len() {
            assert!(
                vapprox(par[i], ser[i], 1e-12),
                "i={i}, par={:?}, ser={:?}",
                par[i],
                ser[i]
            );
        }
    }

    #[test]
    fn total_force_is_zero() {
        let mut sys = NBodySystem::default();
        sys.add_particle(mk_particle(1, [0.0, 0.0, 0.0], 5.0));
        sys.add_particle(mk_particle(2, [1.0, 0.0, 0.0], 7.0));
        sys.add_particle(mk_particle(3, [0.0, 2.0, 0.0], 4.0));

        let f = sys.compute_all_forces();
        let total = f
            .into_iter()
            .fold([0.0, 0.0, 0.0], |acc, v| vec3_add(acc, v));
        assert!(vapprox(total, [0.0, 0.0, 0.0], 1e-10));
    }

    #[test]
    fn emits_limit_reached_with_particles_snapshot() {
        let mut sys = NBodySystem::default();
        sys.add_particle(mk_particle(1, [0.0, 0.0, 0.0], 5.0));
        sys.add_particle(mk_particle(2, [1.0, 0.0, 0.0], 7.0));
        sys.add_particle(mk_particle(3, [0.0, 2.0, 0.0], 4.0));

        let mut rx = sys.subscribe();
        sys.set_limit(2);

        let _ = sys.compute_all_forces();
        match rx.try_recv() {
            Err(TryRecvError::Empty) => {}
            other => panic!("unexpected receive on step 1: {:?}", other),
        }

        let _ = sys.compute_all_forces();
        match rx.try_recv() {
            Ok(NBodySignal::LimitReached { particles }) => {
                assert_eq!(particles.len(), sys.len());
                let ids_src: Vec<u64> = (0..sys.len())
                    .map(|i| sys.get_particle_by_index(i).unwrap().id())
                    .collect();
                let ids_msg: Vec<u64> = particles.iter().map(|p| p.id()).collect();
                assert_eq!(ids_src, ids_msg);
            }
            Err(e) => panic!("expected LimitReached, got error: {:?}", e),
        }

        let _ = sys.compute_all_forces();
        match rx.try_recv() {
            Err(TryRecvError::Empty) => {}
            other => panic!("unexpected receive after reset: {:?}", other),
        }
    }

    #[test]
    fn real_world_reference_earth_moon_and_sun_earth() {
        use crate::physics::gravity::G;

        const M_EARTH: f64 = 5.972e24;
        const M_MOON: f64 = 7.342e22;
        const D_EM: f64 = 384_400_000.0;

        const M_SUN: f64 = 1.9885e30;
        const D_SE: f64 = 149_597_870_700.0;

        fn mk(id: u64, pos: [f64; 3], mass: f64) -> Particle {
            Particle::new(id, pos, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], mass)
        }
        fn vmag(v: [f64; 3]) -> f64 {
            (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
        }
        fn rel_close(a: f64, b: f64, rel: f64) -> bool {
            let denom = b.abs().max(1.0);
            ((a - b) / denom).abs() <= rel
        }

        let cases = [(M_EARTH, M_MOON, D_EM), (M_SUN, M_EARTH, D_SE)];

        for (i, &(m1, m2, r)) in cases.iter().enumerate() {
            let mut sys = NBodySystem::default();
            sys.add_particle(mk(1, [0.0, 0.0, 0.0], m1));
            sys.add_particle(mk(2, [r, 0.0, 0.0], m2));

            let forces = sys.compute_all_forces();
            assert_eq!(forces.len(), 2, "case {i}: forces len");

            let mag = vmag(forces[0]);
            let expected = G * m1 * m2 / (r * r);

            assert!(
                rel_close(mag, expected, 1e-5),
                "case {i}: |{mag} - {expected}| / max(|expected|,1) > 1e-5 (r={r})"
            );

            assert!(
                forces[0][0].is_sign_positive(),
                "case {i}: X component not positive"
            );
            assert!(
                forces[0][1].abs() <= 1e-9 && forces[0][2].abs() <= 1e-9,
                "case {i}: not aligned with X"
            );
            assert!(
                (forces[1][0] + forces[0][0]).abs() < 1e-9
                    && (forces[1][1] + forces[0][1]).abs() < 1e-9
                    && (forces[1][2] + forces[0][2]).abs() < 1e-9,
                "case {i}: Newton's 3rd law failed"
            );
        }
    }
}
