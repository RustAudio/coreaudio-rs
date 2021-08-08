//! A basic input + output stream example, copying the mic input stream to the default output stream

extern crate coreaudio;

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use coreaudio::audio_unit::audio_format::LinearPcmFlags;
use coreaudio::audio_unit::render_callback::{self, data};
use coreaudio::audio_unit::{AudioUnit, Element, SampleFormat, Scope, StreamFormat};
use coreaudio::sys::*;

type S = f32;
const SAMPLE_FORMAT: SampleFormat = SampleFormat::F32;
// type S = i32; const SAMPLE_FORMAT: SampleFormat = SampleFormat::I32;
// type S = i16; const SAMPLE_FORMAT: SampleFormat = SampleFormat::I16;
// type S = i8; const SAMPLE_FORMAT: SampleFormat = SampleFormat::I8;

pub fn run_example() -> Result<(), coreaudio::Error> {
    let mut input_audio_unit = AudioUnit::new(coreaudio::audio_unit::IOType::RemoteIO)?;
    let mut output_audio_unit = AudioUnit::new(coreaudio::audio_unit::IOType::RemoteIO)?;

    // Read device sample rate off the output stream
    let id = kAudioUnitProperty_StreamFormat;
    let asbd: AudioStreamBasicDescription =
        output_audio_unit.get_property(id, Scope::Output, Element::Output)?;
    let sample_rate = asbd.mSampleRate;

    // iOS doesn't let you reconfigure an "initialized" audio unit, so uninitialize them
    input_audio_unit.uninitialize()?;
    output_audio_unit.uninitialize()?;

    configure_for_recording(&mut input_audio_unit)?;

    let format_flag = match SAMPLE_FORMAT {
        SampleFormat::F32 => LinearPcmFlags::IS_FLOAT,
        SampleFormat::I32 | SampleFormat::I16 | SampleFormat::I8 => {
            LinearPcmFlags::IS_SIGNED_INTEGER
        }
    };

    // Using IS_NON_INTERLEAVED everywhere because data::Interleaved is commented out / not implemented
    let in_stream_format = StreamFormat {
        sample_rate: sample_rate,
        sample_format: SAMPLE_FORMAT,
        flags: format_flag | LinearPcmFlags::IS_PACKED | LinearPcmFlags::IS_NON_INTERLEAVED,
        // audio_unit.set_input_callback is hardcoded to 1 buffer, and when using non_interleaved
        // we are forced to 1 channel
        channels: 1,
    };

    let out_stream_format = StreamFormat {
        sample_rate: sample_rate,
        sample_format: SAMPLE_FORMAT,
        flags: format_flag | LinearPcmFlags::IS_PACKED | LinearPcmFlags::IS_NON_INTERLEAVED,
        // you can change this to 1
        channels: 2,
    };

    println!("input={:#?}", &in_stream_format);
    println!("output={:#?}", &out_stream_format);
    println!("input_asbd={:#?}", &in_stream_format.to_asbd());
    println!("output_asbd={:#?}", &out_stream_format.to_asbd());

    let id = kAudioUnitProperty_StreamFormat;
    input_audio_unit.set_property(
        id,
        Scope::Output,
        Element::Input,
        Some(&in_stream_format.to_asbd()),
    )?;
    output_audio_unit.set_property(
        id,
        Scope::Input,
        Element::Output,
        Some(&out_stream_format.to_asbd()),
    )?;

    let buffer_left = Arc::new(Mutex::new(VecDeque::<S>::new()));
    let producer_left = buffer_left.clone();
    let consumer_left = buffer_left.clone();
    let buffer_right = Arc::new(Mutex::new(VecDeque::<S>::new()));
    let producer_right = buffer_right.clone();
    let consumer_right = buffer_right.clone();

    // seed roughly 1 second of data to create a delay in the feedback loop for easier testing
    for buffer in vec![buffer_left, buffer_right] {
        let mut buffer = buffer.lock().unwrap();
        for _ in 0..(out_stream_format.sample_rate as i32) {
            buffer.push_back(0 as S);
        }
    }

    type Args = render_callback::Args<data::NonInterleaved<S>>;

    println!("set_input_callback");
    input_audio_unit.set_input_callback(move |args| {
        let Args {
            num_frames,
            mut data,
            ..
        } = args;
        let buffer_left = producer_left.lock().unwrap();
        let buffer_right = producer_right.lock().unwrap();
        let mut buffers = vec![buffer_left, buffer_right];
        for i in 0..num_frames {
            for (ch, channel) in data.channels_mut().enumerate() {
                let value: S = channel[i];
                buffers[ch].push_back(value);
            }
        }
        Ok(())
    })?;
    input_audio_unit.initialize()?;
    input_audio_unit.start()?;

    println!("set_render_callback");
    output_audio_unit.set_render_callback(move |args: Args| {
        let Args {
            num_frames,
            mut data,
            ..
        } = args;

        let buffer_left = consumer_left.lock().unwrap();
        let buffer_right = consumer_right.lock().unwrap();
        let mut buffers = vec![buffer_left, buffer_right];
        for i in 0..num_frames {
            // Default other channels to copy value from first channel as a fallback
            let zero: S = 0 as S;
            let f: S = *buffers[0].front().unwrap_or(&zero);
            for (ch, channel) in data.channels_mut().enumerate() {
                let sample: S = buffers[ch].pop_front().unwrap_or(f);
                channel[i] = sample;
            }
        }
        Ok(())
    })?;
    output_audio_unit.initialize()?;
    output_audio_unit.start()?;

    // for the purposes of this demo, leak these so that after returning the audio units will
    // keep running
    std::mem::forget(input_audio_unit);
    std::mem::forget(output_audio_unit);

    Ok(())
}

fn configure_for_recording(audio_unit: &mut AudioUnit) -> Result<(), coreaudio::Error> {
    println!("Configure audio unit for recording");

    // Enable mic recording
    let enable_input = 1u32;
    audio_unit.set_property(
        kAudioOutputUnitProperty_EnableIO,
        Scope::Input,
        Element::Input,
        Some(&enable_input),
    )?;

    // Disable output
    let disable_output = 0u32;
    audio_unit.set_property(
        kAudioOutputUnitProperty_EnableIO,
        Scope::Output,
        Element::Output,
        Some(&disable_output),
    )?;

    Ok(())
}
