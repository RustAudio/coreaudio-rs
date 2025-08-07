//! Core Audio's various const audio unit types identifiers represented as typesafe enums.
//!
//! Original documentation [here](https://developer.apple.com/library/prerelease/mac/documentation/AudioUnit/Reference/AUComponentServicesReference/index.html#//apple_ref/doc/constant_group/Audio_Unit_Types).

#![allow(deprecated)]

//#[cfg(target_os = "ios")]
//use objc2_audio_toolbox::kAudioUnitSubType_RemoteIO;
use objc2_audio_toolbox::{
    kAudioUnitSubType_3DMixer, kAudioUnitSubType_AUConverter, kAudioUnitSubType_AUFilter,
    kAudioUnitSubType_AUiPodTimeOther, kAudioUnitSubType_AudioFilePlayer,
    kAudioUnitSubType_BandPassFilter, kAudioUnitSubType_DLSSynth, kAudioUnitSubType_DefaultOutput,
    kAudioUnitSubType_DeferredRenderer, kAudioUnitSubType_Delay, kAudioUnitSubType_Distortion,
    kAudioUnitSubType_DynamicsProcessor, kAudioUnitSubType_GenericOutput,
    kAudioUnitSubType_GraphicEQ, kAudioUnitSubType_HALOutput, kAudioUnitSubType_HighPassFilter,
    kAudioUnitSubType_HighShelfFilter, kAudioUnitSubType_LowPassFilter,
    kAudioUnitSubType_LowShelfFilter, kAudioUnitSubType_MatrixMixer,
    kAudioUnitSubType_MatrixReverb, kAudioUnitSubType_Merger,
    kAudioUnitSubType_MultiBandCompressor, kAudioUnitSubType_MultiChannelMixer,
    kAudioUnitSubType_NBandEQ, kAudioUnitSubType_NetSend, kAudioUnitSubType_NewTimePitch,
    kAudioUnitSubType_ParametricEQ, kAudioUnitSubType_PeakLimiter, kAudioUnitSubType_Pitch,
    kAudioUnitSubType_RogerBeep, kAudioUnitSubType_SampleDelay, kAudioUnitSubType_Sampler,
    kAudioUnitSubType_ScheduledSoundPlayer, kAudioUnitSubType_Splitter,
    kAudioUnitSubType_StereoMixer, kAudioUnitSubType_SystemOutput, kAudioUnitSubType_TimePitch,
    kAudioUnitSubType_Varispeed, kAudioUnitSubType_VoiceProcessingIO, kAudioUnitType_Effect,
    kAudioUnitType_FormatConverter, kAudioUnitType_Generator, kAudioUnitType_MIDIProcessor,
    kAudioUnitType_Mixer, kAudioUnitType_MusicDevice, kAudioUnitType_MusicEffect,
    kAudioUnitType_OfflineEffect, kAudioUnitType_Output, kAudioUnitType_Panner,
};

/// Represents the different kinds of Audio Units that are available.
///
/// Original documentation [here](https://developer.apple.com/library/prerelease/mac/documentation/AudioUnit/Reference/AUComponentServicesReference/index.html#//apple_ref/doc/constant_group/Audio_Unit_Types).
#[derive(Copy, Clone, Debug)]
pub enum Type {
    /// Provides input, output, or both input and output simultaneously.
    ///
    /// It can be used as the head of an audio unit processing graph.
    ///
    /// **Available** in OS X v10.2 and later.
    IO(IOType),
    /// An instrument unit can be used as a software musical instrument, such as a sampler or
    /// synthesizer.
    ///
    /// It responds to MIDI (Musical Instrument Digital Interface) control signals and can create
    /// notes.
    ///
    /// **Available** in OS X v10.2 and later.
    MusicDevice(MusicDeviceType),
    /// An effect unit that can respond to MIDI control messages, typically through a mapping of
    /// MIDI messages to parameters of the audio unit's DSP algorithm.
    ///
    /// **Available** in OS X v10.2 and later.
    MusicEffect,
    /// A format converter unit can transform audio formats, such as performing sample rate
    /// conversion.
    ///
    /// A format converter is also appropriate for deferred rendering and for effects such as
    /// varispeed.
    ///
    /// A format converter unit can ask for as much or as little audio input as it needs to produce
    /// a given output, while still completing its rendering within the time represented by the
    /// output buffer.
    ///
    /// For effect-like format converters, such as pitch shifters, it is common to provide both a
    /// real-time and an offline version. OS X, for example, includes Time-Pitch and Varispeed
    /// audio units in both real-time and offline versions.
    ///
    /// **Available** in OS X v10.2 and later.
    FormatConverter(FormatConverterType),
    /// An effect unit repeatedly processes a number of audio input samples to produce the same
    /// number of audio output samples.
    ///
    /// Most commonly, an effect unit has a single input and a single output.
    ///
    /// Some effects take side-chain inputs as well.
    ///
    /// Effect units can be run offline, such as to process a file without playing it, but are
    /// expected to run in real-time.
    ///
    /// **Available** in OS X v10.2 and later.
    Effect(EffectType),
    /// A mixer unit takes a number of input channels and mixes them to provide one or more output
    /// channels.
    ///
    /// For example, the **StereoMixer** **SubType** in OS X takes multiple mono or stereo inputs
    /// and produces a single stereo output.
    ///
    /// **Available** in OS X v10.2 and later.
    Mixer(MixerType),
    /// A panner unit is a specialised effect unit that distributes one or more channels in a
    /// single input to one or more channels in a single output.
    ///
    /// Panner units must support a set of standard audio unit parameters that specify panning
    /// coordinates.
    ///
    /// **Available** in OS X v10.3 and later.
    Panner,
    /// A generator unit provides audio output that has no audio input.
    ///
    /// This audio unit type is appropriate for a tone generator.
    ///
    /// Unlike an instrument unit, a generator unit does not have a control input.
    ///
    /// **Available** in OS X v10.3 and later.
    Generator(GeneratorType),
    /// An offline effect unit provides digital signal processing of a sort that cannot proceed in
    /// real-time.
    ///
    /// For example, level normalisation requires examination of an entire sound, beginning to end,
    /// before the normalisation factor can be calculated.
    ///
    /// As such, offline effect units also have a notion of a priming stage that can be performed
    /// before the actual rendering/processing phase is executed.
    ///
    /// **Available** in OS X v10.3 and later.
    OfflineEffect,
    /// FIXME: Could not find any documenation for this type - it seems it was added very recently
    /// (around 2013) and Apple's documentation doesn't seem to have updated to include it.
    MidiProcessor,
}

impl Type {
    /// Convert the `Type` to its associated `u32` for compatibility with original API.
    pub fn as_u32(&self) -> u32 {
        match *self {
            Type::IO(_) => kAudioUnitType_Output,
            Type::MusicDevice(_) => kAudioUnitType_MusicDevice,
            Type::MusicEffect => kAudioUnitType_MusicEffect,
            Type::FormatConverter(_) => kAudioUnitType_FormatConverter,
            Type::Effect(_) => kAudioUnitType_Effect,
            Type::Mixer(_) => kAudioUnitType_Mixer,
            Type::Panner => kAudioUnitType_Panner,
            Type::Generator(_) => kAudioUnitType_Generator,
            Type::OfflineEffect => kAudioUnitType_OfflineEffect,
            Type::MidiProcessor => kAudioUnitType_MIDIProcessor,
        }
    }

    /// Convert the `Type` to the const `u32` that is associated with its subtype.
    pub fn as_subtype_u32(&self) -> Option<u32> {
        match *self {
            Type::IO(ty) => Some(ty as u32),
            Type::MusicDevice(ty) => Some(ty as u32),
            Type::FormatConverter(ty) => Some(ty as u32),
            Type::Effect(ty) => Some(ty as u32),
            Type::Mixer(ty) => Some(ty as u32),
            Type::Generator(ty) => Some(ty as u32),
            _ => None,
        }
    }
}

impl From<EffectType> for Type {
    fn from(ty: EffectType) -> Self {
        Type::Effect(ty)
    }
}

impl From<FormatConverterType> for Type {
    fn from(ty: FormatConverterType) -> Self {
        Type::FormatConverter(ty)
    }
}

impl From<MixerType> for Type {
    fn from(ty: MixerType) -> Self {
        Type::Mixer(ty)
    }
}

impl From<GeneratorType> for Type {
    fn from(ty: GeneratorType) -> Self {
        Type::Generator(ty)
    }
}

impl From<MusicDeviceType> for Type {
    fn from(ty: MusicDeviceType) -> Self {
        Type::MusicDevice(ty)
    }
}

impl From<IOType> for Type {
    fn from(ty: IOType) -> Self {
        Type::IO(ty)
    }
}

/// Effect (digital signal processing) audio unit subtypes for audio units provided by Apple.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EffectType {
    /// An audio unit that enforces an upper dynamic limit on an audio signal.
    ///
    /// **Available** in OS X v10.2 and later.
    PeakLimiter = kAudioUnitSubType_PeakLimiter as isize,
    /// An audio unit that provides dynamic compression or expansion.
    ///
    /// **Available** in OS X v10.3 and later.
    DynamicsProcessor = kAudioUnitSubType_DynamicsProcessor as isize,
    /// An audio unit that passes frequencies below a specified cutoff frequency and blocks
    /// frequencies above that cutoff frequency.
    ///
    /// **Available** in OS X v10.2 and later.
    LowPassFilter = kAudioUnitSubType_LowPassFilter as isize,
    /// An audio unit that passes frequencies above a specified cutoff frequency and blocks
    /// frequencies below that cutoff frequency.
    ///
    /// **Available** in OS X v10.2 and later.
    HighPassFilter = kAudioUnitSubType_HighPassFilter as isize,
    /// An audio unit that passes frequencies between specified upper and lower cutoff frequencies,
    /// and blocks frequencies outside that band.
    ///
    /// **Available** in OS X v10.2 and later.
    BandPassFilter = kAudioUnitSubType_BandPassFilter as isize,
    /// An audio unit suitable for implementing a treble control in an audio playback or recording
    /// system.
    ///
    /// **Available** in OS X v10.2 and later.
    HighShelfFilter = kAudioUnitSubType_HighShelfFilter as isize,
    /// An audio unit suitable for implementing a bass control in an audio playback or recording
    /// system.
    ///
    /// **Available** in OS X v10.2 and later.
    LowShelfFilter = kAudioUnitSubType_LowShelfFilter as isize,
    /// An audio unit that provides a filter whose center frequency, boost/cut level, and Q can be
    /// adjusted.
    ///
    /// **Available** in OS X v10.2 and later.
    ParametricEQ = kAudioUnitSubType_ParametricEQ as isize,
    /// An audio unit that provides a distortion effect.
    ///
    /// **Available** in OS X v10.5 and later.
    Distortion = kAudioUnitSubType_Distortion as isize,
    /// An audio unit that introduces a time delay to a signal.
    ///
    /// **Available** in OS X v10.2 and later.
    Delay = kAudioUnitSubType_Delay as isize,
    /// An audio unit that provides a time delay for a specified number of samples.
    ///
    /// **Available** in OS X v10.4 and later.
    SampleDelay = kAudioUnitSubType_SampleDelay as isize,
    /// An audio unit that provides a 10- or 31-band graphic equalizer.
    ///
    /// Available in OS X v10.2 and later.
    GraphicEQ = kAudioUnitSubType_GraphicEQ as isize,
    /// An audio unit that provides four-bands of dynamic compression or expansion.
    ///
    /// **Available** in OS X v10.3 and later.
    MultiBandCompressor = kAudioUnitSubType_MultiBandCompressor as isize,
    /// An audio unit that provides a reverberation effect that can be used to simulate a variety
    /// of acoustic spaces.
    ///
    /// **Available** in OS X v10.2 and later.
    MatrixReverb = kAudioUnitSubType_MatrixReverb as isize,
    /// An audio unit for modifying the pitch of a signal.
    ///
    /// **Available** in OS X v10.4 and later.
    Pitch = kAudioUnitSubType_Pitch as isize,
    /// An audio unit that provides a combination of five filters: low-frequency, three
    /// mid-frequencies, and high-frequency.
    ///
    /// **Available** in OS X v10.4 and later.
    AUFilter = kAudioUnitSubType_AUFilter as isize,
    /// An audio unit for use in conjunction with a kAudioUnitSubType_NetReceive audio unit for
    /// sending audio across a network or from one application to another.
    ///
    /// **Available** in OS X v10.4 and later.
    NetSend = kAudioUnitSubType_NetSend as isize,
    /// An audio unit that detects gaps between segments of speech and fills the gaps with a short
    /// tone, simulating the sound of a walkie-talkie communication device.
    ///
    /// **Available** in OS X v10.5 and later.
    RogerBeep = kAudioUnitSubType_RogerBeep as isize,
    /// A multi-band equalizer with specifiable filter type for each band.
    ///
    /// **Available** in OS X v10.9 and later.
    NBandEQ = kAudioUnitSubType_NBandEQ as isize,
}

/// Audio data format converter audio unit subtypes for **AudioUnit**s provided by Apple.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FormatConverterType {
    /// An audio unit that uses an audio converter to do linear PCM conversions, such as changes to
    /// sample rate, bit depth, or interleaving.
    ///
    /// **Available** in OS X v10.2 and later.
    AUConverter = kAudioUnitSubType_AUConverter as isize,
    /// An audio unit that can be used to have independent control of both playback rate and pitch.
    ///
    /// In OS X it provides a generic view, so it can be used in both a UI and programmatic
    /// context.
    ///
    /// It also comes in an offline version for processing audio files.
    ///
    /// **Available** in OS X v10.7 and later.
    NewTimePitch = kAudioUnitSubType_NewTimePitch as isize,
    /// An audio unit that can provide independent control of playback rate and pitch. This subtype
    /// provides a generic view, making it suitable for UI and programmatic context. OS X provides
    /// realtime and offline audio units of this subtype.
    ///
    /// **Available** in OS X v10.3 and later.
    TimePitch = kAudioUnitSubType_TimePitch as isize,
    /// An audio unit that acquires audio input from a separate thread than the thread on which its
    /// render method is called.
    ///
    /// You can use this subtype to introduce multiple threads into an audio unit processing graph.
    ///
    /// There is a delay, equal to the buffer size, introduced between the audio input and output.
    ///
    /// **Available** in OS X v10.4 and later.
    DeferredRenderer = kAudioUnitSubType_DeferredRenderer as isize,
    /// An audio unit with one input bus and two output buses. The audio unit duplicates the input
    /// signal to each of its two output buses.
    ///
    /// **Available** in OS X v10.4 and later.
    Splitter = kAudioUnitSubType_Splitter as isize,
    /// An audio unit with two input buses and one output bus. The audio unit merges the two input
    /// signals to the single output.
    ///
    /// **Available** in OS X v10.4 and later.
    Merger = kAudioUnitSubType_Merger as isize,
    /// An audio unit that can control playback rate. As the playback rate increases, so does
    /// pitch.
    ///
    /// This subtype provides a generic view, making it suitable for UI and programmatic context.
    ///
    /// OS X provides realtime and offline audio units of this subtype.
    ///
    /// **Available** in OS X v10.3 and later.
    Varispeed = kAudioUnitSubType_Varispeed as isize,
    /// **Available** in OS X v10.9 and later.
    AUiPodTimeOther = kAudioUnitSubType_AUiPodTimeOther as isize,
}

/// Audio mixing **AudioUnit** subtypes for **AudioUnit**s provided by Apple.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MixerType {
    /// An audio unit that can have any number of input buses, with any number of channels on each
    /// input bus, and one output bus.
    ///
    /// In OS X, the output bus can have any number of channels.
    ///
    /// In iPhone OS, the output bus always has two channels.
    ///
    /// **Available** in OS X v10.5 and later.
    MultiChannelMixer = kAudioUnitSubType_MultiChannelMixer as isize,
    /// An audio unit that can have any number of input buses, each of which is mono or stereo, and
    /// one stereo output bus.
    ///
    /// **Available** in OS X v10.2 and later.
    StereoMixer = kAudioUnitSubType_StereoMixer as isize,
    /// An audio unit that can have any number of input buses and one output bus.
    ///
    /// Each input bus can be mono, in which case it can be panned using 3D coordinates and
    /// parameters.
    ///
    /// Stereo input buses pass directly through to the output.
    ///
    /// Four-channel ambisonic inputs are rendered to the output configuration.
    ///
    /// The single output bus can be configured with 2, 4, 5, 6, 7 or 8 channels.
    ///
    /// **Available** in OS X v10.3 and later.
    ///
    /// **Deprecated** in OS X v10.10.
    #[deprecated = "Depecated in OS X v10.10"]
    Mixer3D = kAudioUnitSubType_3DMixer as isize,
    /// An audio unit that can have any number of input and output buses with any number of
    /// channels on each bus.
    ///
    /// You configure the mix using a matrix of channels with a separate input level control for
    /// each channel.
    ///
    /// The audio unit also provides individual level control for each
    /// input-channel-to-output-channel combination, as well as level control for each output
    /// channel.
    ///
    /// Finally, the audio unit provides a global level control for the matrix as a whole.
    ///
    /// **Available** in OS X v10.3 and later.
    MatrixMixer = kAudioUnitSubType_MatrixMixer as isize,
}

/// Audio units that serve as sound sources.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GeneratorType {
    /// A generator unit that can be used to schedule slices of audio to be played at specified
    /// times.
    ///
    /// The audio is scheduled using the time stamps for the render operation and can be scheduled
    /// from any thread.
    ///
    /// **Available** in OS X v10.4 and later.
    ScheduledSoundPlayer = kAudioUnitSubType_ScheduledSoundPlayer as isize,
    /// A generator unit that is used to play a file. In OS X it presents a custom UI so can be
    /// used in a UI context as well as in a programmatic context.
    ///
    /// **Available** in OS X v10.4 and later.
    AudioFilePlayer = kAudioUnitSubType_AudioFilePlayer as isize,
}

/// Audio units that can be played as musical instruments via MIDI control.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MusicDeviceType {
    /// A multitimbral instrument unit that can use sample banks in either DLS or SoundFont
    /// formats.
    ///
    /// It fully supports GM-MIDI and the basic extensions of GS-MIDI
    ///
    /// **Available** in OS X v10.2 and later.
    DLSSynth = kAudioUnitSubType_DLSSynth as isize,
    /// A monotimbral instrument unit that functions a a sampler-synthesizer and supports full
    /// interactive editing of its state.
    ///
    /// **Available** in OS X v10.7 and later.
    Sampler = kAudioUnitSubType_Sampler as isize,
}

/// Input/output **AudioUnit** subtypes for **AudioUnit**s provided by Apple.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IOType {
    /// An audio unit that responds to start/stop calls and provides basic services for converting
    /// to and from linear PCM formats.
    ///
    /// Use this audio unit when sending the output of an audio processing graph to your
    /// application rather than to the output audio hardware. You would typically use the Generic
    /// Output unit for offline audio processing. Just like the other I/O units, the Generic Output
    /// unit incorporates a Format Converter unit. This lets the Generic Output unit perform format
    /// conversion between the stream format used in an audio processing graph and the format you
    /// want.
    ///
    /// You can also use a Generic Output unit as the final node in a subgraph that you place into
    /// a parent audio processing graph.
    ///
    /// **Available** in OS X v10.2 and later.
    GenericOutput = kAudioUnitSubType_GenericOutput as isize,
    /// An audio unit that can provides input/output connection to an a specified audio device.
    ///
    /// Bus 0 provides output to the audio device and bus 1 accepts input from the audio device.
    ///
    /// **Available** in OS X v10.2 and later.
    HalOutput = kAudioUnitSubType_HALOutput as isize,
    /// A specialized **HalOutput** audio unit that connects to the user’s selected default device
    /// in Sound Preferences.
    ///
    /// **Available** in OS X v10.2 and later.
    DefaultOutput = kAudioUnitSubType_DefaultOutput as isize,
    /// A specialized **HalOutput** audio unit that connects to the user’s selected device for
    /// sound effects, alerts, and other user-interface sounds.
    ///
    /// **Available** in OS X v10.2 and later.
    SystemOutput = kAudioUnitSubType_SystemOutput as isize,
    /// An audio unit that interfaces to the audio inputs and outputs of iPhone OS devices and
    /// provides voice processing features.
    ///
    /// Bus 0 provides output to hardware and bus 1 accepts input from hardware.
    ///
    /// See the [Voice-Processing I/O Audio Unit
    /// Properties](https://developer.apple.com/library/prerelease/mac/documentation/AudioUnit/Reference/AudioUnitPropertiesReference/index.html#//apple_ref/doc/constant_group/Voice_Processing_I_O_Audio_Unit_Properties)
    /// enumeration for the identifiers for this audio unit’s properties.
    ///
    /// **Available** in OS X v10.7 and later.
    VoiceProcessingIO = kAudioUnitSubType_VoiceProcessingIO as isize,
    /// Connects to device hardware for input, output, or simultaneous input and output.
    /// Use it for playback, recording, or low-latency simultaneous input and output where echo
    /// cancellation is not needed.
    ///
    /// See <https://developer.apple.com/library/content/documentation/MusicAudio/Conceptual/AudioUnitHostingGuide_iOS/UsingSpecificAudioUnits/UsingSpecificAudioUnits.html>
    /// **Available** in iOS.
    #[cfg(target_os = "ios")]
    RemoteIO = 1919512419, //kAudioUnitSubType_RemoteIO, only available in the ios sdk,
}
