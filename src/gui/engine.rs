use crate::types::nbodysystem::NBodySystem;
use crate::types::particle::Particle;
use macroquad::camera::{Camera2D, set_camera};
use macroquad::color::{BLACK, WHITE};
use macroquad::input::{KeyCode, get_keys_down, get_keys_pressed, mouse_wheel};
use macroquad::math::vec2;
use macroquad::prelude::{
    clear_background, draw_circle, next_frame, screen_height, screen_width, set_fullscreen,
};

pub struct NBodyEngine {
    m_system: NBodySystem,
    m_zoom: f32,
    m_x: f32,
    m_y: f32,
}

impl Default for NBodyEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl NBodyEngine {
    pub fn new() -> Self {
        Self {
            m_system: NBodySystem::default(),
            m_zoom: 1000.0,
            m_x: 1000.0,
            m_y: 850.0,
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

            if get_keys_down().contains(&KeyCode::A) {
                self.m_x -= 1.0 * self.m_zoom / screen_width();
            }

            if get_keys_down().contains(&KeyCode::D) {
                self.m_x += 1.0 * self.m_zoom / screen_width();
            }

            if get_keys_down().contains(&KeyCode::W) {
                self.m_y += 1.0 * self.m_zoom / screen_height();
            }

            if get_keys_down().contains(&KeyCode::S) {
                self.m_y -= 1.0 * self.m_zoom / screen_height();
            }

            if get_keys_pressed().contains(&KeyCode::Space) {
                self.m_system.add_random_particle();
            }

            if get_keys_pressed().contains(&KeyCode::R) {
                self.m_system = NBodySystem::default();
            }

            let wheel = mouse_wheel().1;

            if wheel != 0.0 {
                self.m_zoom *= 1.0 - wheel * 0.0001;
                self.m_zoom = self.m_zoom.clamp(100.0, 10000.0);
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
                let p = self.m_system.get_particle_by_index(i).expect("REASON");
                p.update_particle_euler(*force, 100.0);

                let x_cord = self.m_system.get_particle_by_index(i).unwrap().pos()[0];
                let y_cord = self.m_system.get_particle_by_index(i).unwrap().pos()[1];

                draw_circle(x_cord as f32, y_cord as f32, 2.0, WHITE);
            }

            next_frame().await
        }
    }
}
