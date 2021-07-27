//! A basic output stream example, using an Output AudioUnit to generate a sine wave.

extern crate coreaudio;

use coreaudio::audio_unit::render_callback::{self, data};
use coreaudio::audio_unit::{AudioUnit, Element, SampleFormat, Scope, StreamFormat};
use coreaudio::audio_unit::audio_format::LinearPcmFlags;
use coreaudio::sys::{
    AudioDeviceID,
    AudioObjectGetPropertyData,
    AudioObjectPropertyAddress,
    kAudioHardwarePropertyDefaultOutputDevice,
    kAudioObjectPropertyScopeGlobal,
    kAudioObjectPropertyElementMaster,
    kAudioHardwareNoError, 
    kAudioUnitProperty_StreamFormat,  
    kAudioOutputUnitProperty_CurrentDevice,
    kAudioOutputUnitProperty_EnableIO,
    kAudioObjectSystemObject
};
use std::f64::consts::PI;
use std::mem;
use std::ptr::null;

type S = f32;
const SAMPLE_FORMAT: SampleFormat = SampleFormat::F32;
// type S = i32; const SAMPLE_FORMAT: SampleFormat = SampleFormat::I32;
// type S = i16; const SAMPLE_FORMAT: SampleFormat = SampleFormat::I16;
// type S = i8; const SAMPLE_FORMAT: SampleFormat = SampleFormat::I8;

const SAMPLE_RATE: f64 = 48000.0;

const INTERLEAVED: bool = true;


struct SineWaveGenerator {
    time: f64,
    /// generated frequency in Hz
    freq: f64,
    /// magnitude of generated signal
    volume: f64,
}

impl SineWaveGenerator {
    fn new(freq: f64, volume: f64) -> Self {
        SineWaveGenerator {
            time: 0.,
            freq,
            volume,
        }
    }
}

impl Iterator for SineWaveGenerator {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        self.time += 1. / SAMPLE_RATE;
        let output = ((self.freq * self.time * PI * 2.).sin() * self.volume) as f32;
        Some(output)
    }
}

/// Copied from cpal
pub fn default_output_device() -> Option<AudioDeviceID> {
    let property_address = AudioObjectPropertyAddress {
        mSelector: kAudioHardwarePropertyDefaultOutputDevice,
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: kAudioObjectPropertyElementMaster,
    };

    let audio_device_id: AudioDeviceID = 0;
    let data_size = mem::size_of::<AudioDeviceID>();
    let status = unsafe {
        AudioObjectGetPropertyData(
            kAudioObjectSystemObject,
            &property_address as *const _,
            0,
            null(),
            &data_size as *const _ as *mut _,
            &audio_device_id as *const _ as *mut _,
        )
    };
    if status != kAudioHardwareNoError as i32 {
        return None;
    }

    Some(audio_device_id)
}

/// Copied from cpal
fn audio_unit_from_device(
    device_id: AudioDeviceID,
    input: bool,
) -> Result<AudioUnit, coreaudio::Error> {
    let mut audio_unit = AudioUnit::new(coreaudio::audio_unit::IOType::HalOutput)?;

    if input {
        // Enable input processing.
        let enable_input = 1u32;
        audio_unit.set_property(
            kAudioOutputUnitProperty_EnableIO,
            Scope::Input,
            Element::Input,
            Some(&enable_input),
        )?;

        // Disable output processing.
        let disable_output = 0u32;
        audio_unit.set_property(
            kAudioOutputUnitProperty_EnableIO,
            Scope::Output,
            Element::Output,
            Some(&disable_output),
        )?;
    }

    audio_unit.set_property(
        kAudioOutputUnitProperty_CurrentDevice,
        Scope::Global,
        Element::Output,
        Some(&device_id),
    )?;

    Ok(audio_unit)
}

fn main() -> Result<(), coreaudio::Error> {
    let frequency_hz_l = 1000.;
    let frequency_hz_r = 1200.;
    let volume = 0.95;
    let mut samples_l = SineWaveGenerator::new(frequency_hz_l, volume);
    let mut samples_r = SineWaveGenerator::new(frequency_hz_r, volume);

    // Construct an Output audio unit that delivers audio to the default output device.
    let mut audio_unit = audio_unit_from_device(default_output_device().unwrap(), false)?;

    let mut format_flag = match SAMPLE_FORMAT {
        SampleFormat::F32 => LinearPcmFlags::IS_FLOAT,
        SampleFormat::I32 | SampleFormat::I16 | SampleFormat::I8 => {
            LinearPcmFlags::IS_SIGNED_INTEGER
        }
    };

    if !INTERLEAVED {
        format_flag = format_flag | LinearPcmFlags::IS_NON_INTERLEAVED;
    }

    let stream_format = StreamFormat {
        sample_rate: SAMPLE_RATE,
        sample_format: SAMPLE_FORMAT,
        flags: format_flag | LinearPcmFlags::IS_PACKED,
        // you can change this to 1
        channels: 2,
    };

    println!("stream format={:#?}", &stream_format);
    println!("asbd={:#?}", &stream_format.to_asbd());

    println!("set audio unit properties");
    let id = kAudioUnitProperty_StreamFormat;
    let asbd = stream_format.to_asbd();
    audio_unit.set_property(id, Scope::Input, Element::Output, Some(&asbd))?;

    

    // For this example, our sine wave expects `f32` data.
    assert!(SampleFormat::F32 == stream_format.sample_format);

    if INTERLEAVED {
        println!("Register interleaved callback");
        type Args = render_callback::Args<data::Interleaved<f32>>;
        audio_unit.set_render_callback(move |args| {
            let Args {
                num_frames,
                data,
                ..
            } = args;
            println!("frames: {}", num_frames);
            for i in 0..num_frames {
                let sample_l = samples_l.next().unwrap();
                let sample_r = samples_r.next().unwrap();
                data.buffer[2*i] = sample_l;
                data.buffer[2*i+1] = sample_r;
            }
            Ok(())
        })?;
    }
    else {
        println!("Register non-interleaved callback");
        type Args = render_callback::Args<data::NonInterleaved<f32>>;
        audio_unit.set_render_callback(move |args| {
            let Args {
                num_frames,
                mut data,
                ..
            } = args;
            for i in 0..num_frames {
                let sample_l = samples_l.next().unwrap();
                let sample_r = samples_r.next().unwrap();
                let mut channels=data.channels_mut();
                let left = channels.next().unwrap();
                left[i] = sample_l;
                let right = channels.next().unwrap();
                right[i] = sample_r;
            }
            Ok(())
        })?;
    }
    audio_unit.start()?;

    std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(())
}
