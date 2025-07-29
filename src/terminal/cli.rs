use crate::types::nbodysystem::NBodySignal;
use crate::types::nbodysystem::NBodySystem;
use crate::types::particle::Particle;
use clap::Parser;
use serde::Serialize;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
struct ParticleData {
    id: u64,
    position: [f64; 3],
    velocity: [f64; 3],
}

/// N Body Problem
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Select the type of data output
    #[arg(short, long, default_value = "none", value_parser = [ "none", "terminal", "plain_text", "csv", "json"])]
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
                        "csv" => {
                            let writer = create_csv_writer();
                            writer(&particles, &file_name);
                        }
                        "json" => {
                            let writer = create_json_writer();
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

fn create_csv_writer() -> impl Fn(&Vec<Particle>, &String) + Send {
    move |particles: &Vec<Particle>, file_name: &String| {
        println!("Write to file: {file_name}");
        let mut csv_output = csv::Writer::from_path(file_name).expect("Unable to create file");

        csv_output
            .write_record([
                "ID",
                "Position_X",
                "Position_Y",
                "Position_Z",
                "Velocity_X",
                "Velocity_Y",
                "Velocity_Z",
            ])
            .expect("Unable to write header");

        for particle in particles {
            let pos = particle.pos();
            let vel = particle.velocity();

            csv_output
                .write_record(&[
                    particle.id().to_string(),
                    pos[0].to_string(),
                    pos[1].to_string(),
                    pos[2].to_string(),
                    vel[0].to_string(),
                    vel[1].to_string(),
                    vel[2].to_string(),
                ])
                .expect("Unable to write record");
        }

        csv_output.flush().expect("Unable to flush file");
    }
}

fn create_json_writer() -> impl Fn(&Vec<Particle>, &String) + Send {
    move |particles: &Vec<Particle>, file_name: &String| {
        println!("Write to file: {file_name}");
        let particles_data: Vec<ParticleData> = particles
            .iter()
            .map(|particle| ParticleData {
                id: particle.id(),
                position: particle.pos(),
                velocity: particle.velocity(),
            })
            .collect();

        let json = serde_json::to_string_pretty(&particles_data).expect("Unable to serialize");
        let mut file = File::create(file_name).expect("Unable to create file");
        file.write_all(json.as_bytes())
            .expect("Unable to write to file");
    }
}
