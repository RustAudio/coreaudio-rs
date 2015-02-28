//! A basic output stream example, using an Output AudioUnit to generate a sine wave.

#![feature(box_syntax, core, old_io, std_misc)]

extern crate "coreaudio-rs" as coreaudio;

use coreaudio::audio_unit::{AudioUnit, Type, SubType};
use std::iter::iterate;
use std::num::Float;
use std::f32::consts::PI_2;

fn main() {

    // 440hz sine wave generator.
    let mut samples = iterate(0.0, |phase| phase + 440.0 / 44_100.0)
        .map(|phase| (phase * PI_2).sin() * 0.05);

    // Construct an Output audio unit.
    let audio_unit = AudioUnit::new(Type::Output, SubType::HalOutput)
        .render_callback(box move |buffer, num_frames| for i in (0..num_frames) {
            let sample = samples.next().unwrap();
            for channel in buffer.iter_mut() {
                channel[i] = sample;
            }
        })
        .start()
        .unwrap();

    ::std::old_io::timer::sleep(::std::time::duration::Duration::seconds(3));

    audio_unit.close();

}

