use n_body_simulator::physics::gravity::G;
use n_body_simulator::types::nbodysystem::NBodySystem;
use n_body_simulator::types::particle::Particle;
use vecmath::{vec3_square_len, vec3_sub};

fn mk_particle(id: u64, pos: [f64; 3], vel: [f64; 3], mass: f64) -> Particle {
    Particle::new(id, pos, vel, [0.0, 0.0, 0.0], mass)
}

fn norm(v: [f64; 3]) -> f64 {
    vec3_square_len::<f64>(v).sqrt()
}

fn snapshot_particles(sys: &mut NBodySystem) -> Vec<([f64; 3], [f64; 3], f64)> {
    let mut out = Vec::with_capacity(sys.len());
    for i in 0..sys.len() {
        let p = sys.get_particle_by_index(i).unwrap();
        out.push((p.pos(), p.velocity(), p.mass()));
    }
    out
}

fn energy_total(sys: &mut NBodySystem) -> f64 {
    let snap = snapshot_particles(sys);

    let mut ke = 0.0;
    for &(_, v, m) in &snap {
        ke += 0.5 * m * vec3_square_len::<f64>(v);
    }

    let mut pe = 0.0;
    for i in 0..snap.len() {
        for j in (i + 1)..snap.len() {
            let (pi, _, mi) = snap[i];
            let (pj, _, mj) = snap[j];
            let r = vec3_square_len::<f64>(vec3_sub(pj, pi)).sqrt();
            if r > 0.0 {
                pe += -G * mi * mj / r;
            }
        }
    }

    ke + pe
}

fn step_system(sys: &mut NBodySystem, dt: f64) {
    let forces = sys.compute_all_forces();
    for (i, f) in forces.into_iter().enumerate() {
        let p = sys.get_particle_by_index(i).expect("index valid");
        p.update_particle_euler(f, dt);
    }
}

fn rel_err(a: f64, b: f64) -> f64 {
    let denom = b.abs().max(1.0);
    ((a - b) / denom).abs()
}

fn tolerances(dt: f64) -> (f64, f64, f64) {
    match () {
        _ if dt <= 0.01 => (5e-3, 5e-3, 5e-3),
        _ if dt <= 0.1 => (2e-2, 2e-2, 2e-2),
        _ => (8e-2, 8e-2, 8e-2),
    }
}

#[test]
fn binary_star_energy_and_positions_stability_for_various_dt() {
    let m = 1.0e22;
    let d = 1.0e6;
    let r = d / 2.0;
    let v0 = (G * m / (2.0 * d)).sqrt();

    for &dt in &[0.01_f64, 0.1_f64, 1.0_f64] {
        let (tol_dist, tol_energy, tol_vel) = tolerances(dt);

        let mut sys = NBodySystem::default();
        sys.add_particle(mk_particle(1, [-r, 0.0, 0.0], [0.0, v0, 0.0], m));
        sys.add_particle(mk_particle(2, [r, 0.0, 0.0], [0.0, -v0, 0.0], m));

        let p1_0 = sys.get_particle_by_index(0).unwrap().pos();
        let p2_0 = sys.get_particle_by_index(1).unwrap().pos();
        let v1_0 = sys.get_particle_by_index(0).unwrap().velocity();
        let v2_0 = sys.get_particle_by_index(1).unwrap().velocity();
        let d0 = norm(vec3_sub(p2_0, p1_0));
        let e0 = energy_total(&mut sys);

        let omega = (G * (2.0 * m) / (d * d * d)).sqrt();
        let t_period = std::f64::consts::TAU / omega;

        let steps = ((2.0 * t_period) / dt).ceil() as usize;

        for _ in 0..steps {
            step_system(&mut sys, dt);

            for i in 0..sys.len() {
                let pi = sys.get_particle_by_index(i).unwrap();
                assert!(
                    pi.pos().iter().all(|x| x.is_finite()),
                    "NaN/Inf pos at dt={dt}"
                );
                assert!(
                    pi.velocity().iter().all(|x| x.is_finite()),
                    "NaN/Inf vel at dt={dt}"
                );
            }
        }

        let p1 = sys.get_particle_by_index(0).unwrap().pos();
        let p2 = sys.get_particle_by_index(1).unwrap().pos();
        let v1 = sys.get_particle_by_index(0).unwrap().velocity();
        let v2 = sys.get_particle_by_index(1).unwrap().velocity();
        let d1 = norm(vec3_sub(p2, p1));
        let e1 = energy_total(&mut sys);

        let dist_rel_err = rel_err(d1, d0);
        assert!(
            dist_rel_err <= tol_dist,
            "dt={dt}: distance drift too big: rel_err={dist_rel_err} (tol={tol_dist}) d0={d0}, d1={d1}"
        );

        let energy_rel_err = rel_err(e1, e0);
        assert!(
            energy_rel_err <= tol_energy,
            "dt={dt}: energy drift too big: rel_err={energy_rel_err} (tol={tol_energy}) E0={e0}, E1={e1}"
        );

        let vmag0 = 0.5 * (norm(v1_0) + norm(v2_0));
        let vmag1 = 0.5 * (norm(v1) + norm(v2));
        let vel_rel_err = rel_err(vmag1, vmag0);
        assert!(
            vel_rel_err <= tol_vel,
            "dt={dt}: velocity magnitude drift too big: rel_err={vel_rel_err} (tol={tol_vel}) v0={vmag0}, v1={vmag1}"
        );

        println!(
            "[dt={dt}] d0={:.6e} d1={:.6e} |E0|={:.6e} |E1|={:.6e} rel_d={:.3e} rel_E={:.3e} rel_v={:.3e}",
            d0,
            d1,
            e0.abs(),
            e1.abs(),
            dist_rel_err,
            energy_rel_err,
            vel_rel_err
        );
    }
}

#[test]
fn three_body_smoke_no_blowup_for_various_dt() {
    let m = 1.0e22;
    let d = 1.0e6;
    let r = d / 2.0;
    let v = (G * m / (2.0 * d)).sqrt();
    let m3 = 1.0e18;

    for &dt in &[0.01_f64, 0.1_f64, 1.0_f64] {
        let (tol_dist, tol_energy, ..) = tolerances(dt);

        let mut sys = NBodySystem::default();
        sys.add_particle(mk_particle(1, [-r, 0.0, 0.0], [0.0, v, 0.0], m));
        sys.add_particle(mk_particle(2, [r, 0.0, 0.0], [0.0, -v, 0.0], m));
        sys.add_particle(mk_particle(3, [0.0, 2.0 * r, 0.0], [100.0, 0.0, 0.0], m3));

        let d0 = norm(vec3_sub(
            sys.get_particle_by_index(1).unwrap().pos(),
            sys.get_particle_by_index(0).unwrap().pos(),
        ));
        let e0 = energy_total(&mut sys);

        let omega = (G * (2.0 * m) / (d * d * d)).sqrt();
        let steps = (std::f64::consts::TAU / omega / dt).ceil() as usize;

        for _ in 0..steps {
            step_system(&mut sys, dt);
            for i in 0..sys.len() {
                let pi = sys.get_particle_by_index(i).unwrap();
                assert!(pi.pos().iter().all(|x| x.is_finite()));
                assert!(pi.velocity().iter().all(|x| x.is_finite()));
            }
        }

        let d1 = norm(vec3_sub(
            sys.get_particle_by_index(1).unwrap().pos(),
            sys.get_particle_by_index(0).unwrap().pos(),
        ));
        let e1 = energy_total(&mut sys);

        let dist_rel_err = rel_err(d1, d0);
        assert!(
            dist_rel_err <= tol_dist * 2.0,
            "3-body dt={dt}: distance drift too big: rel_err={dist_rel_err} (tol={})",
            tol_dist * 2.0
        );

        let energy_rel_err = rel_err(e1, e0);
        assert!(
            energy_rel_err <= tol_energy * 2.0,
            "3-body dt={dt}: energy drift too big: rel_err={energy_rel_err} (tol={})",
            tol_energy * 2.0
        );

        println!(
            "[3-body dt={dt}] d0={:.6e} d1={:.6e} |E0|={:.6e} |E1|={:.6e} rel_d={:.3e} rel_E={:.3e}",
            d0,
            d1,
            e0.abs(),
            e1.abs(),
            dist_rel_err,
            energy_rel_err
        );
    }
}
