#![allow(dead_code)] //Delete later

use vecmath::Vector3;
use vecmath::Vector4;

pub struct Particle {
    m_id: u64,
    m_position: Vector3<i64>,
    m_velocity: Vector4<i64>,
    m_acceleration: Vector4<i64>,
    m_mass: f32,
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            m_id: 0,
            m_position: [0, 0, 0],
            m_velocity: [0, 0, 0, 0],
            m_acceleration: [0, 0, 0, 0],
            m_mass: 0.0,
        }
    }
}

impl Particle {
    pub fn new(
        id: u64,
        pos: Vector3<i64>,
        velocity: Vector4<i64>,
        acceleration: Vector4<i64>,
        mass: f32) -> Self {

        Self {
            m_id: id,
            m_position: pos,
            m_velocity: velocity,
            m_acceleration: acceleration,
            m_mass: mass,
        }
    }

    pub fn generate_random() -> Self {
        let mut rand_pos: Vector3<i64> = Default::default();
        let mut rand_velocity: Vector4<i64> = Default::default();
        let mut rand_acceleration: Vector4<i64> = Default::default();

        for x in rand_velocity.iter_mut() {
            *x = rand::random();
        }

        for x in rand_pos.iter_mut() {
            *x = rand::random();
        }

        for x in rand_acceleration.iter_mut() {
            *x = rand::random();
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
        return self.m_id;
    }

    pub fn pos(&self) -> Vector3<i64> {
        return self.m_position;
    }

    pub fn velocity(&self) -> Vector4<i64> {
        return self.m_velocity;
    }

    pub fn acceleration(&self) -> Vector4<i64> {
        return self.m_acceleration;
    }

    pub fn mass(&self) -> f32 {
        return self.m_mass;
    }

    pub fn id_mut(&mut self) -> &mut u64 {
        return &mut self.m_id;
    }

    pub fn pos_mut(&mut self) -> &mut Vector3<i64> {
        return &mut self.m_position;
    }

    pub fn velocity_mut(&mut self) -> &mut Vector4<i64> {
        return &mut self.m_velocity;
    }

    pub fn acceleration_mut(&mut self) -> &mut Vector4<i64> {
        return &mut self.m_acceleration;
    }

    pub fn mass_mut(&mut self) -> &mut f32 {
        return &mut self.m_mass;
    }
}
