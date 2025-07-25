extern crate rand;

use crate::gui::engine::NBodyEngine;
use crate::types::particle::Particle;

pub mod gui;
pub mod physics;
pub mod types;

#[macroquad::main("N Body Problem")]
async fn main() {
    let mut engine = NBodyEngine::new();

    engine.add_particle(Particle::new(
        0,
        [1000.0, 500.0, 0.0],
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        100000000000.0,
    ));

    engine.add_particle(Particle::new(
        1,
        [500.0, 500.0, 0.0],
        [-0.0000, 0.1, 0.0],
        [0.0, 0.0, 0.0],
        50000000.0,
    ));

    engine.add_particle(Particle::new(
        2,
        [1500.26, 1200.0, 0.0],
        [0.1, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        500000.0,
    ));

    for i in 0..1000 {
        engine.add_particle(Particle::new(
            i + 3,
            [500.26 + (i as f64 * 30.0), 1000.0, 0.0],
            [0.1, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            -0.0000000000000001,
        ));
    }

    engine.update().await
}
