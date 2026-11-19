/*

Here is a complete, zero-dependency DSP test script for src/main.rs that demonstrates structs (state),
traits (polymorphic audio processing), and iterators (zero-allocation sample pipelines).

*/

use std::f32::consts::TAU; // TAU = 2 * PI

/// 1. TRAIT: Defines a common ***interface*** for any audio processor (like a VST process block).
trait AudioProcessor {
    fn process_sample(&mut self, input: f32) -> f32;

    // Default trait method: processes an entire audio buffer in-place using an iterator.
    fn process_buffer(&mut self, buffer: &mut [f32]) {
        for sample in buffer.iter_mut() {
            *sample = self.process_sample(*sample);
        }
    }
}
/// 2. STRUCT (Generator): Maintains phase state for a sine wave oscillator.
struct SineOscillator {
    frequency: f32,
    sample_rate: f32,
    phase: f32,
}

impl SineOscillator {
    fn new(frequency: f32, sample_rate: f32) -> Self {
        Self {
            frequency,
            sample_rate,
            phase: 0.0,
        }
    }
}

impl AudioProcessor for SineOscillator {
    fn process_sample(&mut self, _input: f32) -> f32 {
        let output = (self.phase * TAU).sin();
        // Advance phase and wrap cleanly between 0.0 and 1.0
        self.phase = (self.phase + self.frequency / self.sample_rate) % 1.0;
        output
    }
}

/// 3. STRUCT (Effect): A simple gain and hard-clipping overdrive processor.
struct Overdrive {
    drive: f32,
    threshold: f32,
}

impl AudioProcessor for Overdrive {
    fn process_sample(&mut self, input: f32) -> f32 {
        (input * self.drive).clamp(-self.threshold, self.threshold)
    }
}

fn main() {
    println!("Hello, world!");

    let sample_rate = 48_000.0;
    let block_size = 16; // Small buffer so we can inspect every sample in the console

    // Allocate a simulated real-time audio buffer (stack-allocated array)
    let mut audio_buffer = [0.0_f32; 16];

    // Instantiate our oscillator (A4 = 440Hz, or 3000Hz here so one cycle fits in 16 samples)
    let mut osc = SineOscillator::new(3_000.0, sample_rate);
    let mut drive = Overdrive {
        drive: 1.5,
        threshold: 0.8,
    };

    // Run the DSP chain: Generate -> Process -> Inspect
    osc.process_buffer(&mut audio_buffer);
    println!("Raw Sine Buffer:\n{:.3?}\n", audio_buffer);

    drive.process_buffer(&mut audio_buffer);
    println!("Clipped Overdrive Buffer:\n{:.3?}\n", audio_buffer);

    // 4. ITERATORS: Calculate RMS (Root Mean Square) loudness in one functional chain
    // Zero heap allocations; compiles down to tight SIMD-friendly machine code.
    let sum_of_squares: f32 = audio_buffer.iter().map(|&s| s * s).sum();
    let rms = (sum_of_squares / block_size as f32).sqrt();

    // Find peak amplitude using iterator folding
    let peak = audio_buffer
        .iter()
        .fold(0.0_f32, |max_val, &s| max_val.max(s.abs()));

    println!("Buffer Analysis -> Peak: {:.3} | RMS: {:.3}", peak, rms);
}
