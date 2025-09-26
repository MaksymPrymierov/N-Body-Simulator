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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::nbodysystem::NBodySystem;
    use crate::types::particle::Particle;
    use std::io::Read;
    use std::time::{Duration, Instant};
    use tempfile::NamedTempFile;

    fn mk_particle(id: u64, pos: [f64; 3], vel: [f64; 3], mass: f64) -> Particle {
        Particle::new(id, pos, vel, [0.0, 0.0, 0.0], mass)
    }

    fn read_to_string(path: &str) -> String {
        let mut f = std::fs::File::open(path).expect("open");
        let mut s = String::new();
        f.read_to_string(&mut s).expect("read");
        s
    }

    #[test]
    fn plain_text_writer_writes_expected_content() {
        let p1 = mk_particle(1, [1.0, 2.0, 3.0], [0.1, 0.2, 0.3], 5.0);
        let p2 = mk_particle(2, [4.0, 5.0, 6.0], [0.4, 0.5, 0.6], 6.0);
        let parts = vec![p1.clone(), p2.clone()];

        let tmp = NamedTempFile::new().unwrap();
        let path = tmp.path().to_string_lossy().to_string();

        let writer = super::create_plain_text_writer();
        writer(&parts, &path);

        let out = read_to_string(&path);
        let expected1 = format!(
            "Particle: [{}] POS: {:?}; Velocity: {:?};\n",
            p1.id(),
            p1.pos(),
            p1.velocity()
        );
        let expected2 = format!(
            "Particle: [{}] POS: {:?}; Velocity: {:?};\n",
            p2.id(),
            p2.pos(),
            p2.velocity()
        );
        assert!(out.contains(&expected1));
        assert!(out.contains(&expected2));
    }

    #[test]
    fn csv_writer_outputs_header_and_rows() {
        let p = mk_particle(10, [0.0, 1.0, 2.0], [3.0, 4.0, 5.0], 1.0);
        let parts = vec![p.clone()];

        let tmp = NamedTempFile::new().unwrap();
        let path = tmp.path().to_string_lossy().to_string();

        let writer = super::create_csv_writer();
        writer(&parts, &path);

        let content = read_to_string(&path);
        assert!(
            content
                .lines()
                .next()
                .unwrap()
                .contains("ID,Position_X,Position_Y,Position_Z,Velocity_X,Velocity_Y,Velocity_Z")
        );
        assert!(content.contains(&p.id().to_string()));
        assert!(content.contains(&p.pos()[0].to_string()));
        assert!(content.contains(&p.velocity()[2].to_string()));
    }

    #[test]
    fn json_writer_serializes_particle_data() {
        let p1 = mk_particle(5, [1.0, 0.0, -1.0], [0.0, 0.0, 0.0], 2.0);
        let p2 = mk_particle(6, [2.5, 3.5, 4.5], [0.1, 0.2, 0.3], 3.0);
        let parts = vec![p1.clone(), p2.clone()];

        let tmp = NamedTempFile::new().unwrap();
        let path = tmp.path().to_string_lossy().to_string();

        let writer = super::create_json_writer();
        writer(&parts, &path);

        let json = read_to_string(&path);
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        let arr = v.as_array().unwrap();
        assert_eq!(arr.len(), 2);
        assert_eq!(arr[0]["id"], p1.id());
        assert_eq!(arr[1]["position"][2], p2.pos()[2]);
        assert_eq!(arr[1]["velocity"][1], p2.velocity()[1]);
    }

    #[test]
    fn terminal_writer_prints_without_panic() {
        let parts = vec![mk_particle(1, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 1.0)];
        let writer = super::create_terminal_writer();
        writer(&parts);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn handle_args_spawns_listener_and_writes_json() {
        let mut system = NBodySystem::default();
        system.add_particle(mk_particle(1, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 1.0));
        system.add_particle(mk_particle(2, [1.0, 0.0, 0.0], [0.0, 0.0, 0.0], 1.0));

        let tmp = NamedTempFile::new().unwrap();
        let path = tmp.path().to_string_lossy().to_string();

        let args =
            super::Args::parse_from(["prog", "--output", "json", "--limit", "1", "--file", &path]);

        let mut cli = super::NBodyCli {
            m_args: args,
            m_n_body_system: &mut system,
        };

        cli.handle_args().await;

        let _ = cli.m_n_body_system.compute_all_forces();

        let start = Instant::now();
        loop {
            if tmp.path().exists()
                && std::fs::metadata(&path)
                    .map(|m| m.len() > 0)
                    .unwrap_or(false)
            {
                break;
            }
            if start.elapsed() > Duration::from_secs(2) {
                panic!("writer thread did not produce output in time");
            }
            std::thread::sleep(Duration::from_millis(20));
        }

        let json = read_to_string(&path);
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(v.is_array());
        let arr = v.as_array().unwrap();
        assert_eq!(arr.len(), 2);
        assert_eq!(arr[0]["id"], 1);
        assert_eq!(arr[1]["id"], 2);
        assert!(arr[0]["position"].is_array());
        assert!(arr[0]["velocity"].is_array());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn handle_args_writes_csv() {
        let mut system = NBodySystem::default();
        system.add_particle(mk_particle(7, [1.0, 2.0, 3.0], [0.1, 0.2, 0.3], 1.0));

        let tmp = NamedTempFile::new().unwrap();
        let path = tmp.path().to_string_lossy().to_string();

        let args =
            super::Args::parse_from(["prog", "--output", "csv", "--limit", "1", "--file", &path]);

        let mut cli = super::NBodyCli {
            m_args: args,
            m_n_body_system: &mut system,
        };
        cli.handle_args().await;
        let _ = cli.m_n_body_system.compute_all_forces();

        let start = Instant::now();
        loop {
            if tmp.path().exists()
                && std::fs::metadata(&path)
                    .map(|m| m.len() > 0)
                    .unwrap_or(false)
            {
                break;
            }
            if start.elapsed() > Duration::from_secs(2) {
                panic!("no csv written");
            }
            std::thread::sleep(Duration::from_millis(20));
        }

        let content = read_to_string(&path);
        assert!(
            content
                .contains("ID,Position_X,Position_Y,Position_Z,Velocity_X,Velocity_Y,Velocity_Z")
        );
        assert!(content.contains("7"));
        assert!(content.contains("1"));
        assert!(content.contains("0.1"));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn handle_args_writes_plain_text() {
        let mut system = NBodySystem::default();
        system.add_particle(mk_particle(9, [9.0, 8.0, 7.0], [0.0, 0.0, 0.0], 1.0));

        let tmp = NamedTempFile::new().unwrap();
        let path = tmp.path().to_string_lossy().to_string();

        let args = super::Args::parse_from([
            "prog",
            "--output",
            "plain_text",
            "--limit",
            "1",
            "--file",
            &path,
        ]);

        let mut cli = super::NBodyCli {
            m_args: args,
            m_n_body_system: &mut system,
        };
        cli.handle_args().await;
        let _ = cli.m_n_body_system.compute_all_forces();

        let start = Instant::now();
        loop {
            if tmp.path().exists()
                && std::fs::metadata(&path)
                    .map(|m| m.len() > 0)
                    .unwrap_or(false)
            {
                break;
            }
            if start.elapsed() > Duration::from_secs(2) {
                panic!("no txt written");
            }
            std::thread::sleep(Duration::from_millis(20));
        }

        let content = read_to_string(&path);
        assert!(content.contains("Particle: [9]"));
        assert!(content.contains("POS:"));
        assert!(content.contains("Velocity:"));
    }
}
