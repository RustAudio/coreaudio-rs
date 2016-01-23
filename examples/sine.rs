//! A basic output stream example, using an Output AudioUnit to generate a sine wave.

extern crate coreaudio;

use coreaudio::audio_unit::{AudioUnit, IOType, SampleFormat};
use coreaudio::audio_unit::render_callback::{self, buffer};
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

type Args<'a> = render_callback::Args<'a, buffer::LinearPcmInterleaved<'a, f32>>;
fn callback<'a, I: Iterator<Item=f32>>(args: Args<'a>, samples: &mut I) -> Result<(), ()> {
    let Args { num_frames, mut buffer, .. } = args;
    let mut idx = 0;
    let channels = buffer.data.len() / num_frames;
    for _ in 0..num_frames {
        let sample = samples.next().unwrap();
        for _ in 0..channels {
            buffer.data[idx] = sample;
            idx += 1;
        }
    }
    Ok(())
}
