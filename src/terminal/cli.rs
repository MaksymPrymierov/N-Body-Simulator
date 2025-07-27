use crate::types::nbodysystem::NBodySignal;
use crate::types::nbodysystem::NBodySystem;
use clap::Parser;

/// N Body Problem
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Print the velocities and coordinates
    #[arg(short, long)]
    output: bool,

    /// Limit the output of information about particles to once every N steps
    #[arg(short, long, default_value_t = 1000)]
    limit: u16,
}

pub struct NBodyCli<'a> {
    m_args: Args,
    m_n_body_system: &'a mut NBodySystem,
}

impl<'a> NBodyCli<'a> {
    pub fn new(n_body_system: &'a mut NBodySystem) -> Self {
        Self {
            m_args: Args::parse(),
            m_n_body_system: n_body_system,
        }
    }

    pub async fn handle_args(&mut self) {
        self.m_n_body_system.set_limit(self.m_args.limit);

        if self.m_args.output {
            let mut receiver = self.m_n_body_system.subscribe();

            std::thread::spawn(move || {
                while let Ok(signal) = receiver.blocking_recv() {
                    match signal {
                        NBodySignal::LimitReached { particles } => {
                            for particle in particles {
                                println!(
                                    "Particle: [{}] POS: {:?}; Velocity: {:?};",
                                    particle.id(),
                                    particle.pos(),
                                    particle.velocity()
                                );
                            }
                        }
                    }
                }
            });
        }
    }
}
