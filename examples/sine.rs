//! A basic output stream example, using an Output AudioUnit to generate a sine wave.

extern crate coreaudio;

use coreaudio::audio_unit::{AudioUnit, IOType, SampleFormat};
use coreaudio::audio_unit::render_callback::{self, data};
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
    run().unwrap()
}

fn run() -> Result<(), coreaudio::Error> {

    // 440hz sine wave generator.
    let mut samples = Iter { value: 0.0 }
        .map(|phase| (phase * PI * 2.0).sin() as f32 * 0.15);

    // Construct an Output audio unit.
    //
    // "HAL" stands for "Hardware Abstraction Layer". `HalOutput` is the simplest, light-weight
    // audio unit for creating an output stream.
    let mut audio_unit = try!(AudioUnit::new(IOType::HalOutput));

    let stream_format = try!(audio_unit.stream_format());
    println!("{:#?}", &stream_format);

    // For this example, our sine wave expects `f32` data.
    assert!(SampleFormat::F32 == stream_format.sample_format);

    try!(audio_unit.set_render_callback(move |args| callback(args, &mut samples)));
    try!(audio_unit.start());

    std::thread::sleep_ms(3000);

    Ok(())
}

type Args<'a> = render_callback::Args<'a, data::LinearPcmNonInterleaved<'a, f32>>;
fn callback<'a, I: Iterator<Item=f32>>(args: Args<'a>, samples: &mut I) -> Result<(), ()> {
    let Args { num_frames, mut data, .. } = args;
    for i in 0..num_frames {
        let sample = samples.next().unwrap();
        for channel in data.buffer.channels_mut() {
            channel[i] = sample;
        }
    }
    Ok(())
}
