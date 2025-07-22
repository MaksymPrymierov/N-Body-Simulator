use macroquad::color::{BLACK, WHITE};
use macroquad::prelude::{clear_background, draw_circle, next_frame, set_fullscreen};
use crate::types::nbodysystem::NBodySystem;
use crate::types::particle::Particle;

pub struct NBodyEngine {
    m_system: NBodySystem,
}

impl NBodyEngine {
    pub fn new() -> Self {
        Self {
            m_system: NBodySystem::default(),
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