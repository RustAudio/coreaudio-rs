//! coreaudio-rs
//! ------------
//!
//! A friendly rust interface for Apple's CoreAudio API.
//!
//! Read the CoreAudio Overview [here](https://developer.apple.com/library/mac/documentation/MusicAudio/Conceptual/CoreAudioOverview/Introduction/Introduction.html).
//!
//! Currently, this crate provides the [audio_unit] module that covers the
//! [Audio Unit framework](https://developer.apple.com/documentation/audiounit)
//! (now part of [Audio Toolbox](https://developer.apple.com/documentation/AudioToolbox)).

#[macro_use]
extern crate bitflags;

pub use error::Error;

#[cfg(feature = "audio_toolbox")]
pub mod audio_unit;
pub mod error;

// MacTypes.h
pub type OSStatus = i32;
