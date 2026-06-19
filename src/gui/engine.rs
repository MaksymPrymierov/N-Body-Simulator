use crate::types::nbodysystem::NBodySystem;
use crate::types::particle::Particle;
use macroquad::camera::{Camera3D, set_camera};
use macroquad::color::{BLACK, WHITE};
use macroquad::input::{
    KeyCode, get_keys_down, get_keys_pressed, mouse_delta_position, set_cursor_grab, show_mouse,
};
use macroquad::math::vec3;
use macroquad::prelude::{clear_background, draw_sphere, next_frame, set_fullscreen};

// --- Configuration parameters (rendering / integration) ---

const DEFAULT_ZOOM: f32 = 1000.0;
const DEFAULT_X: f32 = 1000.0;
const DEFAULT_Y: f32 = 850.0;

// Base camera moving speed factor.
const CAMERA_MOVE_SPEED: f32 = 2.0;

// Radius of drawn particles.
const PARTICLE_RADIUS: f32 = 5.0;

/// Integration time step `dt` (simulation seconds per frame).
const TIME_STEP: f64 = 100.0;

/// Real-time visualization and driver for an [`NBodySystem`].
///
/// UI bindings:
/// - **Space**: add a random particle
/// - **R**:     remove all particles
/// - **W/A/S/D**: fly camera (First-Person)
/// - **Mouse**: look around
/// - **Esc**: toggle mouse lock
pub struct NBodyEngine<'a> {
    m_system: &'a mut NBodySystem,
    m_zoom: f32,
    m_x: f32,
    m_y: f32,
    m_yaw: f32,
    m_pitch: f32,
    m_mouse_locked: bool,
}

impl<'a> NBodyEngine<'a> {
    /// Creates a new engine bound to an existing [`NBodySystem`].
    pub fn new(nbody_system: &'a mut NBodySystem) -> Self {
        Self {
            m_system: nbody_system,
            m_zoom: DEFAULT_ZOOM,
            m_x: DEFAULT_X,
            m_y: DEFAULT_Y,
            m_yaw: std::f32::consts::PI / 2.0,
            m_pitch: -89.0_f32.to_radians(),
            m_mouse_locked: false,
        }
    }

    /// Adds a particle to the underlying system.
    pub fn add_particle(&mut self, particle: Particle) {
        self.m_system.add_particle(particle);
    }

    /// Adds a randomly initialized particle to the system.
    pub fn add_random_particle(&mut self) {
        self.m_system.add_random_particle();
    }

    /// Initializes the window and clears the background.
    pub fn create_window(&mut self) {
        set_fullscreen(false);
        clear_background(BLACK);

        set_cursor_grab(false);
        show_mouse(true);
    }

    /// Main render/update loop.
    pub async fn update(&mut self) -> ! {
        loop {
            clear_background(BLACK);

            let keys_down = get_keys_down();
            let keys_pressed = get_keys_pressed();

            if keys_pressed.contains(&KeyCode::Escape) {
                self.m_mouse_locked = !self.m_mouse_locked;
                set_cursor_grab(self.m_mouse_locked);
                show_mouse(!self.m_mouse_locked);
            }

            if self.m_mouse_locked {
                let mouse_delta = mouse_delta_position();
                let sensitivity = 20.0_f32.to_radians();

                self.m_yaw += mouse_delta.x * sensitivity;
                self.m_pitch += mouse_delta.y * sensitivity;

                let max_pitch = 89.5_f32.to_radians();
                self.m_pitch = self.m_pitch.clamp(-max_pitch, max_pitch);
            }

            let front = vec3(
                self.m_pitch.cos() * self.m_yaw.cos(),
                self.m_pitch.cos() * self.m_yaw.sin(),
                self.m_pitch.sin(),
            )
            .normalize();

            let world_up = vec3(0.0, 0.0, 1.0);
            let right = front.cross(world_up).normalize();
            let up = right.cross(front).normalize();

            let speed = CAMERA_MOVE_SPEED * (self.m_zoom.abs() / 200.0).max(0.5);

            if keys_down.contains(&KeyCode::W) {
                self.m_x += front.x * speed;
                self.m_y += front.y * speed;
                self.m_zoom += front.z * speed;
            }
            if keys_down.contains(&KeyCode::S) {
                self.m_x -= front.x * speed;
                self.m_y -= front.y * speed;
                self.m_zoom -= front.z * speed;
            }
            if keys_down.contains(&KeyCode::D) {
                self.m_x += right.x * speed;
                self.m_y += right.y * speed;
                self.m_zoom += right.z * speed;
            }
            if keys_down.contains(&KeyCode::A) {
                self.m_x -= right.x * speed;
                self.m_y -= right.y * speed;
                self.m_zoom -= right.z * speed;
            }

            if keys_pressed.contains(&KeyCode::Space) {
                self.m_system.add_random_particle();
            }

            if keys_pressed.contains(&KeyCode::R) {
                self.m_system.remove_all_particles();
            }

            let pos = vec3(self.m_x, self.m_y, self.m_zoom);
            set_camera(&Camera3D {
                position: pos,
                up,
                target: pos + front,
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
                let z = p.pos()[2];

                draw_sphere(
                    vec3(x as f32, y as f32, z as f32),
                    PARTICLE_RADIUS,
                    None,
                    WHITE,
                );
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
