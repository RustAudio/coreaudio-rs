//! coreaudio-rs
//! ------------
//!
//! A friendly rust interface for Apple's CoreAudio API.
//!
//! Read the CoreAudio Overview [here](https://developer.apple.com/library/mac/documentation/MusicAudio/Conceptual/CoreAudioOverview/Introduction/Introduction.html).
//!
//! Currently, work has only been started on the [audio_unit](./audio_unit/index.html) module, but
//! eventually we'd like to cover at least the majority of the C API.

#[macro_use]
extern crate bitflags;

pub use error::Error;

#[cfg(feature = "audio_toolbox")]
pub mod audio_unit;
pub mod error;

pub mod sys {
    #[cfg(feature = "audio_toolbox")]
    pub use objc2_audio_toolbox::*;
    #[cfg(feature = "core_audio")]
    pub use objc2_core_audio::*;
    #[cfg(feature = "core_audio")]
    pub use objc2_core_audio_types::*;
    #[cfg(feature = "core_midi")]
    pub use objc2_core_midi::*;

    // MacTypes.h
    pub type OSStatus = i32;
}
