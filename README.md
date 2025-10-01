![Build Status](https://github.com/N-Body-Project/N-Body-Simulator/actions/workflows/rust.yml/badge.svg?branch=main)

# N-Body Simulator

A Rust implementation of the N-body problem, featuring real-time visualization using [macroquad](https://github.com/not-fl3/macroquad).  
The project simulates gravitational interactions between particles, updates their states via numerical integration, and supports both CLI logging and interactive rendering.

https://github.com/user-attachments/assets/5813a521-858d-400a-af17-99c9ed7d1b44

## 🔧 Features
- Newtonian gravity: *F = G·m₁·m₂ / r²* (direction along the unit vector from one body to another).
- Particle state updates via semi-implicit (symplectic) Euler.
- Parallel force computation with **rayon**.
- Real-time visualization with **macroquad** (WASD to pan, mouse wheel to zoom, space to add new random particle).
- CLI output modes: `none`, `terminal`, `plain_text`, `csv`, `json`.
- Broadcast channel via `tokio::sync::broadcast` to throttle and log snapshots every *N* steps.
- Test suite:
    - **Unit tests** for `particle`, `gravity`, `nbodysystem`, CLI writers.
    - **Integration tests** for orbital stability, energy drift, and multi-body behavior with varying `dt`.

---

## ✅ Requirements
- Stable Rust (latest stable recommended).
- OS with graphics support for macroquad (Windows/macOS/Linux).
- Optional dev-deps used by tests: `tokio` (rt + macros), `tempfile`, `serde_json`, `csv`.

---

## ▶️ Run the Simulation
### Build & run (release recommended)
```bash
cargo run --release
```

### CLI options
```bash
cargo run -- --help
```
Example:
```bash
cargo run -- --output csv --file results.csv --limit 500
```

### In-app controls (macroquad)
- **W/A/S/D** — pan the camera
- **Mouse wheel** — zoom (clamped between `MIN_ZOOM` and `MAX_ZOOM`)
- **Space** — add a random particle
- **R** — remove all particles
- Fullscreen is enabled at startup

---

## 🧪 Running Tests
Run all tests:
```bash
cargo test
```

Run only the orbital integration tests and show logs:
```bash
cargo test --test orbit_integration -- --nocapture
```

---

## 🧠 Physics Notes
- Gravitational constant **G**: `6.67430e-11` (SI units).
- **r = 0** handling: `gravitational_force` returns `[0,0,0]` to avoid division by zero.  
  For physically consistent setups, avoid overlapping initial positions.
- Semi-implicit (symplectic) Euler:
    - `v_{t+Δt} = v_t + (F/m)·Δt`
    - `x_{t+Δt} = x_t + v_{t+Δt}·Δt`
- Larger `dt` increases energy drift. For better long-term stability, consider Velocity Verlet / Leapfrog or Runge–Kutta.
