extern crate rand;

use crate::types::nbodysystem::NBodySystem;
use crate::types::particle::Particle;
use macroquad::prelude::*;

pub mod physics;
pub mod types;

#[macroquad::main("N Body Problem")]
async fn main() {
    let mut system = NBodySystem::default();
    let particle_count: usize = 1003;

    system.add_particle(Particle::new(
        0,
        [1000.0, 500.0, 0.0],
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        100000000000.0,
    ));

    system.add_particle(Particle::new(
        1,
        [500.0, 500.0, 0.0],
        [-0.0000, 0.1, 0.0],
        [0.0, 0.0, 0.0],
        50000000.0,
    ));

    system.add_particle(Particle::new(
        2,
        [1500.26, 1200.0, 0.0],
        [0.1, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        500000.0,
    ));

    for i in 0..1000 {
        system.add_particle(Particle::new(
            i + 3,
            [500.26 + (i as f64 * 30.0), 1000.0, 0.0],
            [0.1, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            -0.0000000000000001,
        ));
    }

    set_fullscreen(true);

    loop {
        clear_background(BLACK);

        let forces = system.compute_all_forces();

        for (i, force) in forces.iter().enumerate().take(particle_count) {
            let p = system.get_particle_by_index(i).expect("REASON");
            p.update_particle_euler(*force, 100.0);
        }

        for i in 0..particle_count {
            let x_cord = system.get_particle_by_index(i).unwrap().pos()[0];
            let y_cord = system.get_particle_by_index(i).unwrap().pos()[1];
            if i == 0 {
                draw_circle(x_cord as f32, y_cord as f32, 100.0, YELLOW);
            } else if i == 1 {
                draw_circle(x_cord as f32, y_cord as f32, 10.0, BLUE);
            } else if i == 2 {
                draw_circle(x_cord as f32, y_cord as f32, 5.0, RED);
            } else if i % 2 > 0 {
                draw_circle(x_cord as f32, y_cord as f32, 2.0, ORANGE);
            } else {
                draw_circle(x_cord as f32, y_cord as f32, 1.0, WHITE);
            }
        }

        next_frame().await
    }
}
