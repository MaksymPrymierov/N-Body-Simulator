use crate::gui::engine::NBodyEngine;
use crate::types::particle::Particle;

pub mod gui;
pub mod physics;
pub mod types;

const MASS_CENTRAL: f64 = 1e11;
const MASS_SECOND: f64 = 5e7;
const MASS_THIRD: f64 = 5e5;
const MASS_NEGATIVE: f64 = -1e-16;

const POS_CENTRAL: [f64; 3] = [1000.0, 500.0, 0.0];
const POS_SECOND: [f64; 3] = [500.0, 500.0, 0.0];
const POS_THIRD: [f64; 3] = [1500.26, 1200.0, 0.0];

const VEL_ZERO: [f64; 3] = [0.0, 0.0, 0.0];
const VEL_SECOND: [f64; 3] = [-0.0, 0.1, 0.0];
const VEL_THIRD: [f64; 3] = [0.1, 0.0, 0.0];

const PARTICLE_COUNT: usize = 1000;
const PARTICLE_START_X: f64 = 500.26;
const PARTICLE_SPACING: f64 = 30.0;
const PARTICLE_Y: f64 = 1000.0;
const PARTICLE_VEL: [f64; 3] = [0.1, 0.0, 0.0];

#[macroquad::main("N Body Problem")]
async fn main() {
    let mut engine = NBodyEngine::new();

    engine.add_particle(Particle::new(
        0,
        POS_CENTRAL,
        VEL_ZERO,
        VEL_ZERO,
        MASS_CENTRAL,
    ));
    engine.add_particle(Particle::new(
        1,
        POS_SECOND,
        VEL_SECOND,
        VEL_ZERO,
        MASS_SECOND,
    ));
    engine.add_particle(Particle::new(2, POS_THIRD, VEL_THIRD, VEL_ZERO, MASS_THIRD));

    for i in 0..PARTICLE_COUNT {
        let position = [
            PARTICLE_START_X + (i as f64 * PARTICLE_SPACING),
            PARTICLE_Y,
            0.0,
        ];

        engine.add_particle(Particle::new(
            (i + 3).try_into().unwrap(),
            position,
            PARTICLE_VEL,
            VEL_ZERO,
            MASS_NEGATIVE,
        ));
    }

    engine.update().await;
}
