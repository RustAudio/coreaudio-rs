//! 
//! This module is an attempt to provide a friendly, rust-esque interface to
//! Apple's Audio Unit API.
//!
//! An audio unit can be constructed with the builder pattern.
//!
//! Learn more about the Audio Unit API here:
//! https://developer.apple.com/library/mac/documentation/MusicAudio/Conceptual/AudioUnitProgrammingGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40003278-CH1-SW2
//!

use bindings::audio_unit as au;
use error::{Error, AudioUnitError};
use libc;
use std::mem;

/// Represents the input and output scope.
#[derive(Copy, Clone, Debug)]
pub enum Scope {
    Output = 0,
    Input  = 1,
}

/// Represents the input and output elements .
#[derive(Copy, Clone, Debug)]
pub enum Element {
    Output = 0,
    Input  = 1,
}

/// Represents the different types of Audio Units.
#[derive(Copy, Clone, Debug)]
pub enum Type {
    Output          = 1635086197,
    MusicDevice     = 1635085685,
    MusicEffect     = 1635085670,
    FormatConverter = 1635083875,
    Effect          = 1635083896,
    Mixer           = 1635085688,
    Panner          = 1635086446,
    Generator       = 1635084142,
    OfflineEffect   = 1635086188,
    MidiProcessor   = 1635085673,
}

/// Represents the different audio unit sub types.
#[derive(Copy, Clone, Debug)]
pub enum SubType {
    GenericOutput        = 1734700658,
    HalOutput            = 1634230636,
    DefaultOutput        = 1684366880,
    SystemOutput         = 1937339168,
    VoiceProcessingIO    = 1987078511,
    DLSSynth             = 1684828960,
    Sampler              = 1935764848,
    MIDISynth            = 1836284270,
    AUConverter          = 1668247158,
    Varispeed            = 1986097769,
    DeferredRenderer     = 1684366962,
    Splitter             = 1936747636,
    Merger               = 1835364967,
    NewTimePitch         = 1853191280,
    AUiPodTimeOther      = 1768977519,
    RoundTripAAC         = 1918984547,
    PeakLimiter          = 1819112562,
    DynamicsProcessor    = 1684237680,
    LowPassFilter        = 1819304307,
    HighPassFilter       = 1752195443,
    BandPassFilter       = 1651532147,
    HighShelfFilter      = 1752393830,
    LowShelfFilter       = 1819502694,
    ParametricEQ         = 1886217585,
    Distortion           = 1684632436,
    Delay                = 1684368505,
    SampleDelay          = 1935961209,
    GraphicEQ            = 1735550321,
    MultiBandCompressor  = 1835232624,
    MatrixReverb         = 1836213622,
    Pitch                = 1953329268,
    AUFilter             = 1718185076,
    NetSend              = 1853058660,
    RogerBeep            = 1919903602,
    NBandEQ              = 1851942257,
    MultiChannelMixer    = 1835232632,
    MatrixMixer          = 1836608888,
    SpatialMixer         = 862217581,
    StereoMixer          = 1936554098,
    Mixer3D              = 862219640,
    SphericalHeadPanner  = 1936746610,
    VectorPanner         = 1986158963,
    SoundFieldPanner     = 1634558569,
    HRTFPanner           = 1752331366,
    NetReceive           = 1852990326,
    ScheduledSoundPlayer = 1936945260,
    AudioFilePlayer      = 1634103404,
}

/// A rust representation of the au::AudioUnit.
pub struct AudioUnit {
    audio_unit: au::AudioUnit,
}

impl AudioUnit {

    /// Construct a new AudioUnit.
    pub fn new(au_type: Type, sub_type: SubType) -> AudioUnitBuilder {

        // A description of the audio unit we desire.
        let desc = au::AudioComponentDescription {
            componentType         : au_type as libc::c_uint,
            componentSubType      : sub_type as libc::c_uint,
            componentManufacturer : au::kAudioUnitManufacturer_Apple,
            componentFlags        : 0,
            componentFlagsMask    : 0,
        };

        unsafe {
            use std::ptr::null_mut;
            // Find the default audio unit for the description.
            let component_result = match au::AudioComponentFindNext(null_mut(), &desc as *const _) {
                component if component == null_mut() => Err(Error::NoMatchingDefaultAudioUnitFound),
                component                            => Ok(component),
            };

            // Get an instance of the default audio unit using the component.
            let mut audio_unit: au::AudioUnit = mem::uninitialized();

            let audio_unit_result = match component_result {
                Ok(component) => {
                    au::AudioComponentInstanceNew(component, &mut audio_unit as *mut au::AudioUnit);
                    Ok(audio_unit)
                },
                Err(err) => Err(err),
            };
            AudioUnitBuilder { audio_unit_result: audio_unit_result }
        }
    }

    /// Close the audio unit.
    pub fn close(self) {}

}

impl Drop for AudioUnit {
    fn drop(&mut self) {
        unsafe {
            Error::from_os_status(au::AudioOutputUnitStop(self.audio_unit)).unwrap();
            Error::from_os_status(au::AudioUnitUninitialize(self.audio_unit)).unwrap();
        }
    }
}

/// A context on which to build the audio unit.
pub struct AudioUnitBuilder {
    audio_unit_result: Result<au::AudioUnit, Error>,
}

impl AudioUnitBuilder {

    /// Pass a render callback (aka "Input Procedure") to the audio unit.
    /// This will be called every time the AudioUnit requests audio.
    /// f is a boxed FnMut whose arg is [frames[channels]].
    #[inline]
    pub fn render_callback(self, f: Box<FnMut(&mut[&mut[f32]], NumFrames) -> Result<(), String>>) -> AudioUnitBuilder {
        let audio_unit_result = match self.audio_unit_result {
            Err(err) => Err(err),
            Ok(audio_unit) => {
                let size_of_render_callback_struct = mem::size_of::<au::AURenderCallbackStruct>() as u32;

                let callback = box RenderCallback { f: f };
                unsafe {
                    // Setup render callback.
                    let render_callback = au::AURenderCallbackStruct {
                        inputProc: Some(input_proc), // TODO
                        inputProcRefCon: mem::transmute(callback),
                    };

                    match Error::from_os_status(au::AudioUnitSetProperty(
                                                audio_unit,
                                                au::kAudioUnitProperty_SetRenderCallback,
                                                Scope::Input as libc::c_uint,
                                                Element::Output as libc::c_uint,
                                                &render_callback as *const _ as *const libc::c_void,
                                                size_of_render_callback_struct)) {
                        Ok(()) => Ok(audio_unit),
                        Err(err) => Err(err),
                    }
                }
            },
        };
        AudioUnitBuilder { audio_unit_result: audio_unit_result }
    }

    /// Finish building the audio unit, intialise it and start it.
    pub fn start(self) -> Result<AudioUnit, Error> {
        let audio_unit = try!(self.audio_unit_result);
        unsafe {
            // Initialise the audio unit!
            try!(Error::from_os_status(au::AudioUnitInitialize(audio_unit)));
            try!(Error::from_os_status(au::AudioOutputUnitStart(audio_unit)));
        }
        Ok(AudioUnit { audio_unit: audio_unit })
    }

}

pub type NumFrames = usize;

/// A struct in which we will pass the callback to the AudioUnit's render callback.
pub struct RenderCallback {
    f: Box<FnMut(&mut[&mut[f32]], NumFrames) -> Result<(), String>>,
}

/// Callback procedure that will be called each time our audio_unit requests audio.
extern "C" fn input_proc(in_ref_con: *mut libc::c_void,
                         _io_action_flags: *mut au::AudioUnitRenderActionFlags,
                         _in_time_stamp: *const au::AudioTimeStamp,
                         _in_bus_number: au::UInt32,
                         in_number_frames: au::UInt32,
                         io_data: *mut au::AudioBufferList) -> au::OSStatus {
    let callback: *mut RenderCallback = in_ref_con as *mut _;
    unsafe {
        let num_channels = (*io_data).mNumberBuffers as usize;
        let mut channels: Vec<&mut [f32]> =
            (0..num_channels)
                .map(|i| {
                    let slice_ptr = (*io_data).mBuffers[i].mData as *mut libc::c_float;
                    ::std::slice::from_raw_parts_mut(slice_ptr, in_number_frames as usize)
                })
                .collect();
        match (*(*callback).f)(&mut channels[..], in_number_frames as usize) {
            Ok(()) => 0,
            Err(description) => {
                println!("{:?}", description);
                AudioUnitError::NoConnection as au::OSStatus
            },
        }
    }
}

