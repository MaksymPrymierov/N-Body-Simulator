use vecmath::{vec3_add, vec3_scale, Vector3};

#[derive(Debug)]
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
        mass: f64,
    ) -> Self {
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
            m_mass: rand::random(),
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
        self.m_velocity = vec3_add(
            self.m_velocity,
            vec3_scale(vec3_scale(force, 1.0 / self.m_mass), dt),
        );
        self.m_position = vec3_add(self.m_position, vec3_scale(self.m_velocity, dt));
    }
}
