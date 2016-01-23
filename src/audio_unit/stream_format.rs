//! A rustification of the `AudioStreamBasicDescription` type.
//!
//! Find the original `AudioStreamBasicDescription` reference [here](https://developer.apple.com/library/mac/documentation/MusicAudio/Reference/CoreAudioDataTypesRef/#//apple_ref/c/tdef/AudioStreamBasicDescription).

use bindings::audio_unit as au;
use error::{self, Error};
use super::audio_format::AudioFormat;
use super::SampleFormat;

/// A representation of the AudioStreamBasicDescription specifically for use with the AudioUnit API.
///
/// By using a type specific to the audio unit API, we can remove a lot of unnecessary boilerplate
/// that is normally associated with the AudioStreamBasicDescription.
///
/// Seeing as `LinearPCM` data (the `AudioFormat` used by the `AudioUnit` API) implies a single
/// frame per packet, we can infer many of the fields in an ASBD from the sample type.
///
/// `bytes_per_packet` = size_of::<S>()
/// `bytes_per_frame` = size_of::<S>()
/// `frames_per_packet` = 1
/// `bits_per_channel` = size_of::<S>() * 8
#[derive(Copy, Clone, Debug)]
pub struct StreamFormat {
    pub sample_rate: f64,
    pub sample_format: SampleFormat,
    pub flags: super::audio_format::LinearPCMFlags,
    pub channels_per_frame: u32,
}

impl StreamFormat {

    /// Convert an AudioStreamBasicDescription into a StreamFormat.
    ///
    /// Note: `audio_unit::StreamFormat` exclusively uses the `LinearPCM` `AudioFormat`. This is as
    /// specified in the documentation:
    ///
    /// > Specify kAudioFormatLinearPCM for the mFormatID field. Audio units use uncompressed audio
    /// data, so this is the correct format identifier to use whenever you work with audio units.
    ///
    /// [*Audio Unit Hosting Guide for iOS*](https://developer.apple.com/library/ios/documentation/MusicAudio/Conceptual/AudioUnitHostingGuide_iOS/AudioUnitHostingFundamentals/AudioUnitHostingFundamentals.html)
    ///
    /// Returns an `Error` if the `AudioFormat` inferred by the ASBD is not `LinearPCM`.
    ///
    /// Returns an `Error` if the sample format type kkkkkkkkkkkkkkkkkkkkkk
    pub fn from_asbd(asbd: au::AudioStreamBasicDescription) -> Result<StreamFormat, Error> {
        use super::audio_format::linear_pcm_flags;

        const NOT_SUPPORTED: Error = Error::AudioUnit(error::audio_unit::Error::FormatNotSupported);

        let au::Struct_AudioStreamBasicDescription {
            mSampleRate,
            mFormatID,
            mFormatFlags,
            mBytesPerPacket,
            mFramesPerPacket,
            mBytesPerFrame,
            mChannelsPerFrame,
            mBitsPerChannel,
            ..
        } = asbd;

        // Retrieve the LinearPCM flags.
        let flags = match AudioFormat::from_format_and_flag(mFormatID, Some(mFormatFlags)) {
            Some(AudioFormat::LinearPCM(flags)) => flags,
            _ => return Err(NOT_SUPPORTED),
        };

        // Determine the `SampleFormat` to use.
        let sample_format = match SampleFormat::from_flags_and_bytes_per_frame(flags, mBytesPerFrame) {
            Some(sample_format) => sample_format,
            None => return Err(NOT_SUPPORTED),
        };

        Ok(StreamFormat {
            sample_rate: mSampleRate,
            flags: flags,
            sample_format: sample_format,
            channels_per_frame: mChannelsPerFrame,
        })
    }

    /// Convert a StreamFormat into an AudioStreamBasicDescription.
    pub fn to_asbd(self) -> au::AudioStreamBasicDescription {
        let StreamFormat {
            sample_rate,
            flags,
            sample_format,
            channels_per_frame,
        } = self;

        let (format, maybe_flag) = AudioFormat::LinearPCM(flags).to_format_and_flag();

        let flag = maybe_flag.unwrap_or(::std::u32::MAX -2147483647);

        let bytes_per_frame = sample_format.size_in_bytes() as u32;
        const FRAMES_PER_PACKET: u32 = 1;
        let bytes_per_packet = bytes_per_frame * FRAMES_PER_PACKET;
        let bits_per_channel = bytes_per_frame * 8;

        au::AudioStreamBasicDescription {
            mSampleRate: sample_rate,
            mFormatID: format,
            mFormatFlags: flag,
            mBytesPerPacket: bytes_per_packet,
            mFramesPerPacket: FRAMES_PER_PACKET,
            mBytesPerFrame: bytes_per_frame,
            mChannelsPerFrame: channels_per_frame,
            mBitsPerChannel: bits_per_channel,
            mReserved: 0,
        }
    }

}
