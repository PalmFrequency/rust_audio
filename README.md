# rust_audio

Open-source audio Digital Signal Processing (DSP) tools, experiments, and DAW integration utilities built in Rust.

**rust_audio** is developed under the [PalmFrequency](https://github.com) organization with a focus on real-time safety, clarity, and accessibility for musicians, producers, students, and developers exploring audio programming.

## Goals
* **Real-Time Safe DSP:** Pure Rust audio processing algorithms free of allocations on the audio thread.
* **DAW & Plugin Workflows:** Modular architecture supporting standalone testing, audio analysis, and plugin formats (VST3 / CLAP) compatible with DAWs such as Ableton Live.
* **Learning & Collaboration:** Well-documented, modular code intended to be readable and hackable for anyone learning Rust and DSP.

## Prerequisites
* [Rust toolchain](https://rust-lang.org) (stable, via `rustup`)
* Git

## Quickstart

Clone the repository and run the test suite:

```bash
git clone https://github.com/rust_audio.git
cd rust_audio
cargo test
```

Run the main test binary:

```bash
cargo run
```

## Project Structure
```text
rust_audio/
├── src/           # Core DSP modules and entry points
├── Cargo.toml     # Dependencies and package metadata
└── LICENSE        # GNU General Public License v3.0
```

## Contributing
Contributions, bug reports, and educational examples are welcome! Please open an issue to discuss ideas or submit a pull request.

## License
This project is licensed under the **GNU General Public License v3.0 (GPLv3)** — see the [LICENSE](LICENSE) file for details.
