//! A basic output stream example, using an Output AudioUnit to generate a sine wave.

extern crate coreaudio_rs as coreaudio;
extern crate num;

use coreaudio::audio_unit::{AudioUnit, IOType};
use num::Float;
use std::f64::consts::PI;


// NOTE: temporary replacement for unstable `std::iter::iterate`
struct Iter {
    value: f64,
}
impl Iterator for Iter {
    type Item = f64;
    fn next(&mut self) -> Option<f64> {
        self.value += 440.0 / 44_100.0;
        Some(self.value)
    }
}


fn main() {

    // 440hz sine wave generator.
    let mut samples = Iter { value: 0.0 }
        .map(|phase| (phase * PI * 2.0).sin() as f32 * 0.15);

    // Construct an Output audio unit.
    let mut audio_unit = AudioUnit::new(IOType::HalOutput).unwrap();

    // Pass the audio unit a callback for filling the buffer.
    audio_unit.set_render_callback(Some(Box::new(move |buffer, num_frames| {
        for frame in 0..num_frames {
            let sample = samples.next().unwrap();
            for channel in buffer.iter_mut() {
                channel[frame] = sample;
            }
        }
        Ok(())
    }))).ok();

    audio_unit.start().unwrap();

    ::std::thread::sleep_ms(3000);
}
