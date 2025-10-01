use vecmath::{Vector3, vec3_add, vec3_scale};

#[derive(Debug, Clone)]
pub struct Particle {
    m_id: u64,
    m_position: Vector3<f64>,
    m_velocity: Vector3<f64>,
    m_acceleration: Vector3<f64>,
    m_mass: f64,
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            m_id: 0,
            m_position: [0.0, 0.0, 0.0],
            m_velocity: [0.0, 0.0, 0.0],
            m_acceleration: [0.0, 0.0, 0.0],
            m_mass: 0.0,
        }
    }
}

impl Particle {
    pub fn new(
        id: u64,
        pos: Vector3<f64>,
        velocity: Vector3<f64>,
        acceleration: Vector3<f64>,
        mut mass: f64,
    ) -> Self {
        if mass <= 0.0 {
            eprintln!("mass must be > 0");
            mass = 0.1;
        }
        Self {
            m_id: id,
            m_position: pos,
            m_velocity: velocity,
            m_acceleration: acceleration,
            m_mass: mass,
        }
    }

    pub fn generate_random() -> Self {
        let mut rand_pos: Vector3<f64> = Default::default();
        let mut rand_velocity: Vector3<f64> = Default::default();
        let mut rand_acceleration: Vector3<f64> = Default::default();

        for x in rand_velocity.iter_mut() {
            *x = rand::random();
        }

        for x in rand_pos.iter_mut() {
            *x = rand::random();
        }

        for x in rand_acceleration.iter_mut() {
            *x = 0.0;
        }

        Self {
            m_id: rand::random(),
            m_position: rand_pos,
            m_velocity: rand_velocity,
            m_acceleration: rand_acceleration,
            m_mass: rand::random::<f64>().clamp(f64::MIN_POSITIVE, 1.0),
        }
    }

    pub fn id(&self) -> u64 {
        self.m_id
    }

    pub fn pos(&self) -> Vector3<f64> {
        self.m_position
    }

    pub fn velocity(&self) -> Vector3<f64> {
        self.m_velocity
    }

    pub fn acceleration(&self) -> Vector3<f64> {
        self.m_acceleration
    }

    pub fn mass(&self) -> f64 {
        self.m_mass
    }

    pub fn id_mut(&mut self) -> &mut u64 {
        &mut self.m_id
    }

    pub fn pos_mut(&mut self) -> &mut Vector3<f64> {
        &mut self.m_position
    }

    pub fn velocity_mut(&mut self) -> &mut Vector3<f64> {
        &mut self.m_velocity
    }

    pub fn acceleration_mut(&mut self) -> &mut Vector3<f64> {
        &mut self.m_acceleration
    }

    pub fn mass_mut(&mut self) -> &mut f64 {
        &mut self.m_mass
    }

    pub fn update_particle_euler(&mut self, force: Vector3<f64>, dt: f64) {
        assert!(self.m_mass > 0.0, "mass must be > 0");
        self.m_velocity = vec3_add(
            self.m_velocity,
            vec3_scale(vec3_scale(force, 1.0 / self.m_mass), dt),
        );
        self.m_position = vec3_add(self.m_position, vec3_scale(self.m_velocity, dt));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vecmath::{vec3_add, vec3_scale};

    const EPS: f64 = 1e-12;

    fn approx(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() <= eps
    }
    fn vapprox(a: Vector3<f64>, b: Vector3<f64>, eps: f64) -> bool {
        approx(a[0], b[0], eps) && approx(a[1], b[1], eps) && approx(a[2], b[2], eps)
    }

    #[test]
    fn new_and_getters_work() {
        let id = 42;
        let pos = [1.0, -2.0, 3.5];
        let vel = [0.1, 0.2, 0.3];
        let acc = [-0.5, 0.0, 0.5];
        let mass = 7.25;

        let p = Particle::new(id, pos, vel, acc, mass);

        assert_eq!(p.id(), id);
        assert!(vapprox(p.pos(), pos, EPS));
        assert!(vapprox(p.velocity(), vel, EPS));
        assert!(vapprox(p.acceleration(), acc, EPS));
        assert!(approx(p.mass(), mass, EPS));
    }

    #[test]
    fn mutators_allow_modification() {
        let mut p = Particle::default();

        *p.id_mut() = 7;
        *p.pos_mut() = [9.0, 8.0, 7.0];
        *p.velocity_mut() = [1.0, 2.0, 3.0];
        *p.acceleration_mut() = [0.5, 0.0, -0.5];
        *p.mass_mut() = 11.0;

        assert_eq!(p.id(), 7);
        assert!(vapprox(p.pos(), [9.0, 8.0, 7.0], EPS));
        assert!(vapprox(p.velocity(), [1.0, 2.0, 3.0], EPS));
        assert!(vapprox(p.acceleration(), [0.5, 0.0, -0.5], EPS));
        assert!(approx(p.mass(), 11.0, EPS));
    }

    #[test]
    fn generate_random_in_expected_ranges_and_zero_accel() {
        let p = Particle::generate_random();

        for c in p.pos() {
            assert!((0.0..1.0).contains(&c), "pos component out of [0,1): {c}");
        }
        for c in p.velocity() {
            assert!((0.0..1.0).contains(&c), "vel component out of [0,1): {c}");
        }
        for c in p.acceleration() {
            assert!(approx(c, 0.0, EPS), "acc component not zero: {c}");
        }
        let m = p.mass();
        assert!((0.0..1.0).contains(&m), "mass out of [0,1): {m}");
    }

    #[test]
    fn euler_update_zero_force_keeps_state() {
        let id = 1;
        let pos0 = [1.0, 2.0, 3.0];
        let vel0 = [0.5, -0.25, 0.0];
        let acc0 = [0.0, 0.0, 0.0];
        let mass = 2.0;
        let dt = 0.1;

        let mut p = Particle::new(id, pos0, vel0, acc0, mass);
        p.update_particle_euler([0.0, 0.0, 0.0], dt);

        let expected_pos = vec3_add(pos0, vec3_scale(vel0, dt));
        let expected_vel = vel0;

        assert!(vapprox(p.velocity(), expected_vel, EPS));
        assert!(vapprox(p.pos(), expected_pos, EPS));
    }

    #[test]
    fn euler_update_applies_force_over_dt() {
        let id = 2;
        let pos0 = [0.0, 0.0, 0.0];
        let vel0 = [0.0, 0.0, 0.0];
        let acc0 = [0.0, 0.0, 0.0];
        let mass = 4.0;
        let dt = 0.5;
        let force = [2.0, -4.0, 6.0];

        let mut p = Particle::new(id, pos0, vel0, acc0, mass);

        let a = vec3_scale(force, 1.0 / mass);
        let v1 = vec3_add(vel0, vec3_scale(a, dt));
        let x1 = vec3_add(pos0, vec3_scale(v1, dt));

        p.update_particle_euler(force, dt);

        assert!(
            vapprox(p.velocity(), v1, 1e-12),
            "vel={:?}, expected={:?}",
            p.velocity(),
            v1
        );
        assert!(
            vapprox(p.pos(), x1, 1e-12),
            "pos={:?}, expected={:?}",
            p.pos(),
            x1
        );
    }

    #[test]
    fn generate_random_always_positive_mass() {
        for _ in 0..1000 {
            let p = Particle::generate_random();
            assert!(p.mass() > 0.0, "random mass must be > 0");
        }
    }

    #[test]
    #[should_panic(expected = "mass must be > 0")]
    fn euler_update_panics_if_mass_non_positive() {
        let mut p = Particle::new(3, [0.0, 0.0, 0.0], [0.0; 3], [0.0; 3], 1.0);
        *p.mass_mut() = 0.0;
        p.update_particle_euler([1.0, 0.0, 0.0], 0.1);
    }
}
