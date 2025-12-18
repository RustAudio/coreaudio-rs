//! Audio example for iOS and tvOS.
//!
//! Both platforms play a 440Hz sine wave (A4 note) to demonstrate audio output.

extern crate coreaudio;

use std::f64::consts::PI;

use coreaudio::audio_unit::audio_format::LinearPcmFlags;
use coreaudio::audio_unit::render_callback::{self, data};
use coreaudio::audio_unit::{AudioUnit, Element, SampleFormat, Scope, StreamFormat};
use objc2_audio_toolbox::kAudioUnitProperty_StreamFormat;
use objc2_core_audio_types::AudioStreamBasicDescription;

type S = f32;
const SAMPLE_FORMAT: SampleFormat = SampleFormat::F32;

struct SineWaveGenerator {
    time: f64,
    sample_rate: f64,
    /// Generated frequency in Hz
    freq: f64,
    /// Magnitude of generated signal
    volume: f64,
}

impl SineWaveGenerator {
    fn new(sample_rate: f64, freq: f64, volume: f64) -> Self {
        SineWaveGenerator {
            time: 0.,
            sample_rate,
            freq,
            volume,
        }
    }
}

impl Iterator for SineWaveGenerator {
    type Item = S;
    fn next(&mut self) -> Option<S> {
        self.time += 1. / self.sample_rate;
        let output = ((self.freq * self.time * PI * 2.).sin() * self.volume) as S;
        Some(output)
    }
}

pub fn run_example() -> Result<(), coreaudio::Error> {
    let mut audio_unit = AudioUnit::new(coreaudio::audio_unit::IOType::RemoteIO)?;

    // Read device sample rate off the output stream
    let id = kAudioUnitProperty_StreamFormat;
    let asbd: AudioStreamBasicDescription =
        audio_unit.get_property(id, Scope::Output, Element::Output)?;
    let sample_rate = asbd.mSampleRate;

    println!("Sample rate: {}", sample_rate);

    // iOS/tvOS don't let you reconfigure an "initialized" audio unit, so uninitialize it
    audio_unit.uninitialize()?;

    let format_flag = match SAMPLE_FORMAT {
        SampleFormat::F32 => LinearPcmFlags::IS_FLOAT,
        SampleFormat::I32 | SampleFormat::I16 | SampleFormat::I8 => {
            LinearPcmFlags::IS_SIGNED_INTEGER
        }
        SampleFormat::I24 => {
            unimplemented!("Not implemented for I24")
        }
    };

    let out_stream_format = StreamFormat {
        sample_rate,
        sample_format: SAMPLE_FORMAT,
        flags: format_flag | LinearPcmFlags::IS_PACKED | LinearPcmFlags::IS_NON_INTERLEAVED,
        channels: 2,
    };

    println!("output={:#?}", &out_stream_format);
    println!("output_asbd={:#?}", &out_stream_format.to_asbd());

    let id = kAudioUnitProperty_StreamFormat;
    audio_unit.set_property(
        id,
        Scope::Input,
        Element::Output,
        Some(&out_stream_format.to_asbd()),
    )?;

    // Create sine wave generator: 440Hz (A4 note) at 30% volume
    let mut samples = SineWaveGenerator::new(sample_rate, 440., 0.3);

    type Args = render_callback::Args<data::NonInterleaved<S>>;

    println!("set_render_callback");
    audio_unit.set_render_callback(move |args: Args| {
        let Args {
            num_frames,
            mut data,
            ..
        } = args;

        for i in 0..num_frames {
            let sample = samples.next().unwrap();
            for channel in data.channels_mut() {
                channel[i] = sample;
            }
        }
        Ok(())
    })?;

    audio_unit.initialize()?;
    audio_unit.start()?;

    println!("Audio started - playing 440Hz sine wave");

    // For the purposes of this demo, leak the audio unit so it keeps running
    std::mem::forget(audio_unit);

    Ok(())
}
