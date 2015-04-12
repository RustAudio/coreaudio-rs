
use bindings::audio_unit as au;
use super::audio_format::AudioFormat;

/// Representation of the AudioStreamBasicDescription.
#[derive(Clone, Debug)]
pub struct StreamFormat {
    pub sample_rate: f64,
    pub audio_format: AudioFormat,
    pub bytes_per_packet: u32,
    pub frames_per_packet: u32,
    pub bytes_per_frame: u32,
    pub channels_per_frame: u32,
    pub bits_per_channel: u32,
}

impl StreamFormat {

    // /// Construct a new StreamFormat.
    // pub fn new(channels: u32, sample_rate: f64) -> StreamFormat {
    // }

    /// Convert an AudioStreamBasicDescription into a StreamFormat.
    #[allow(non_snake_case)]
    pub fn from_asbd(asbd: au::AudioStreamBasicDescription) -> StreamFormat {
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
        StreamFormat {
            sample_rate: mSampleRate,
            audio_format: match AudioFormat::from_format_and_flag(mFormatID, Some(mFormatFlags)) {
                Some(audio_format) => audio_format,
                None => panic!("Failed to convert the ASBD's format and flag to AudioFormat"),
            },
            bytes_per_packet: mBytesPerPacket,
            frames_per_packet: mFramesPerPacket,
            bytes_per_frame: mBytesPerFrame,
            channels_per_frame: mChannelsPerFrame,
            bits_per_channel: mBitsPerChannel,
        }
    }

    /// Convert a StreamFormat into an AudioStreamBasicDescription.
    pub fn to_asbd(self) -> au::AudioStreamBasicDescription {
        let StreamFormat {
            sample_rate,
            audio_format,
            bytes_per_packet,
            frames_per_packet,
            bytes_per_frame,
            channels_per_frame,
            bits_per_channel,
        } = self;
        let (format, maybe_flag) = audio_format.to_format_and_flag();
        let flag = maybe_flag.unwrap_or(::std::u32::MAX -2147483647);
        au::AudioStreamBasicDescription {
            mSampleRate: sample_rate,
            mFormatID: format,
            mFormatFlags: flag,
            mBytesPerPacket: bytes_per_packet,
            mFramesPerPacket: frames_per_packet,
            mBytesPerFrame: bytes_per_frame,
            mChannelsPerFrame: channels_per_frame,
            mBitsPerChannel: bits_per_channel,
            mReserved: 0,
        }
    }

    /// Return the number of channels.
    pub fn num_channels(&self) -> u32 {
        self.channels_per_frame
    }

}



