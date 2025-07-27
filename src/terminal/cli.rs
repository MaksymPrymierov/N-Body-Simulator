use crate::types::nbodysystem::NBodySignal;
use crate::types::nbodysystem::NBodySystem;
use crate::types::particle::Particle;
use clap::Parser;

/// N Body Problem
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Select the type of data output
    #[arg(short, long, default_value = "none", value_parser = [ "none", "terminal", "plain_text"])]
    output: String,

    /// Limit the output of information about particles to once every N steps
    #[arg(short, long, default_value_t = 1000)]
    limit: u16,

    /// Set a file name to save the results
    #[arg(short, long, default_value = "output.txt")]
    file: String,
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
        let mut receiver = self.m_n_body_system.subscribe();
        let file_name = self.m_args.file.clone();
        let output_type = self.m_args.output.clone();

        std::thread::spawn(move || {
            while let Ok(signal) = receiver.blocking_recv() {
                match signal {
                    NBodySignal::LimitReached { particles } => match output_type.as_str() {
                        "none" => {}
                        "terminal" => {
                            let writer = create_terminal_writer();
                            writer(&particles);
                        }
                        "plain_text" => {
                            let writer = create_plain_text_writer();
                            writer(&particles, &file_name);
                        }
                        _ => println!("Incorrect output type"),
                    },
                }
            }
        });
    }
}

fn create_terminal_writer() -> impl Fn(&Vec<Particle>) + Send {
    move |particles: &Vec<Particle>| {
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

fn create_plain_text_writer() -> impl Fn(&Vec<Particle>, &String) + Send {
    move |particles: &Vec<Particle>, file_name: &String| {
        let mut output = String::new();
        println!("Write to file: {file_name}");
        for particle in particles {
            output += format!(
                "Particle: [{}] POS: {:?}; Velocity: {:?};\n",
                particle.id(),
                particle.pos(),
                particle.velocity()
            )
            .as_str();
        }
        std::fs::write(file_name.as_str(), output).expect("Unable to write to file");
    }
}
