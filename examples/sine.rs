//! A basic output stream example, using an Output AudioUnit to generate a sine wave.

#![feature(box_syntax, core, std_misc, thread_sleep)]

extern crate coreaudio_rs as coreaudio;

use coreaudio::audio_unit::{AudioUnit, Type, SubType};
use std::iter::iterate;
use std::num::Float;
use std::f64::consts::PI_2;

fn main() {

    // 440hz sine wave generator.
    let mut samples = iterate(0.0, move |phase| phase + 440.0 / 44_100.0)
        .map(|phase| (phase * PI_2).sin() as f32 * 0.15);

    // Construct an Output audio unit.
    let audio_unit = AudioUnit::new(Type::Output, SubType::HalOutput)
        .render_callback(box move |buffer, num_frames| {
            for frame in (0..num_frames) {
                let sample = samples.next().unwrap();
                for channel in buffer.iter_mut() {
                    channel[frame] = sample;
                }
            }
            Ok(())
        })
        .start()
        .unwrap();

    ::std::thread::sleep(::std::time::duration::Duration::seconds(3));

    audio_unit.close();

}

