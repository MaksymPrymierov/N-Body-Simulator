use crate::types::nbodysystem::NBodySystem;
use crate::types::particle::Particle;
use macroquad::camera::{Camera2D, set_camera};
use macroquad::color::{BLACK, WHITE};
use macroquad::input::{KeyCode, get_keys_down, get_keys_pressed, mouse_wheel};
use macroquad::math::vec2;
use macroquad::prelude::{
    clear_background, draw_circle, next_frame, screen_height, screen_width, set_fullscreen,
};

const DEFAULT_ZOOM: f32 = 1000.0;
const DEFAULT_X: f32 = 1000.0;
const DEFAULT_Y: f32 = 850.0;
const CAMERA_MOVE_SPEED: f32 = 5.0;
const MIN_ZOOM: f32 = 100.0;
const MAX_ZOOM: f32 = 10000.0;
#[cfg(windows)]
const ZOOM_SENSITIVITY: f32 = 0.0009;
#[cfg(unix)]
const ZOOM_SENSITIVITY: f32 = 0.09;
const PARTICLE_RADIUS: f32 = 2.0;
const TIME_STEP: f64 = 100.0;

pub struct NBodyEngine<'a> {
    m_system: &'a mut NBodySystem,
    m_zoom: f32,
    m_x: f32,
    m_y: f32,
}

impl<'a> NBodyEngine<'a> {
    pub fn new(nbody_system: &'a mut NBodySystem) -> Self {
        Self {
            m_system: nbody_system,
            m_zoom: DEFAULT_ZOOM,
            m_x: DEFAULT_X,
            m_y: DEFAULT_Y,
        }
    }

    pub fn add_particle(&mut self, particle: Particle) {
        self.m_system.add_particle(particle);
    }

    pub fn add_random_particle(&mut self) {
        self.m_system.add_random_particle();
    }

    pub fn create_window(&mut self) {
        set_fullscreen(true);
        clear_background(BLACK);
    }

    pub async fn update(&mut self) {
        loop {
            clear_background(BLACK);

            let keys_down = get_keys_down();

            if keys_down.contains(&KeyCode::A) {
                self.m_x -= CAMERA_MOVE_SPEED * self.m_zoom / screen_width();
            }

            if keys_down.contains(&KeyCode::D) {
                self.m_x += CAMERA_MOVE_SPEED * self.m_zoom / screen_width();
            }

            if keys_down.contains(&KeyCode::W) {
                self.m_y += CAMERA_MOVE_SPEED * self.m_zoom / screen_height();
            }

            if keys_down.contains(&KeyCode::S) {
                self.m_y -= CAMERA_MOVE_SPEED * self.m_zoom / screen_height();
            }

            let keys_pressed = get_keys_pressed();

            if keys_pressed.contains(&KeyCode::Space) {
                self.m_system.add_random_particle();
            }

            if keys_pressed.contains(&KeyCode::R) {
                self.m_system.remove_all_particles();
            }

            let wheel = mouse_wheel().1;
            if wheel != 0.0 {
                self.m_zoom *= 1.0 - wheel * ZOOM_SENSITIVITY;
                self.m_zoom = self.m_zoom.clamp(MIN_ZOOM, MAX_ZOOM);
            }

            set_camera(&Camera2D {
                zoom: vec2(
                    1.0 / self.m_zoom,
                    screen_width() / screen_height() * (1.0 / self.m_zoom),
                ),
                target: vec2(self.m_x, self.m_y),
                offset: vec2(0.0, 0.0),
                ..Default::default()
            });

            let forces = self.m_system.compute_all_forces();

            for (i, force) in forces.iter().enumerate() {
                let p = self
                    .m_system
                    .get_particle_by_index(i)
                    .expect("Invalid particle index");
                p.update_particle_euler(*force, TIME_STEP);

                let x = p.pos()[0];
                let y = p.pos()[1];

                draw_circle(x as f32, y as f32, PARTICLE_RADIUS, WHITE);
            }

            next_frame().await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::nbodysystem::NBodySystem;
    use crate::types::particle::Particle;

    fn mk_particle(id: u64, pos: [f64; 3], mass: f64) -> Particle {
        Particle::new(id, pos, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], mass)
    }

    #[test]
    fn new_initializes_defaults() {
        let mut sys = NBodySystem::default();
        let engine = NBodyEngine::new(&mut sys);

        assert!((engine.m_zoom - super::DEFAULT_ZOOM).abs() < f32::EPSILON);
        assert!((engine.m_x - super::DEFAULT_X).abs() < f32::EPSILON);
        assert!((engine.m_y - super::DEFAULT_Y).abs() < f32::EPSILON);
    }

    #[test]
    fn add_particle_delegates_to_system() {
        let mut sys = NBodySystem::default();
        let mut engine = NBodyEngine::new(&mut sys);

        assert_eq!(engine.m_system.len(), 0);
        engine.add_particle(mk_particle(1, [0.0, 0.0, 0.0], 1.0));
        assert_eq!(engine.m_system.len(), 1);

        engine.add_particle(mk_particle(2, [1.0, 0.0, 0.0], 2.0));
        assert_eq!(engine.m_system.len(), 2);

        let p0 = engine.m_system.get_particle_by_index(0).unwrap();
        assert_eq!(p0.id(), 1);
    }

    #[test]
    fn add_random_particle_delegates_to_system() {
        let mut sys = NBodySystem::default();
        let mut engine = NBodyEngine::new(&mut sys);

        let before = engine.m_system.len();
        engine.add_random_particle();
        assert_eq!(engine.m_system.len(), before + 1);

        engine.add_random_particle();
        assert_eq!(engine.m_system.len(), before + 2);
    }
}
