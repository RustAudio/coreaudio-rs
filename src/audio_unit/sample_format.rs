use super::audio_format::{self, LinearPcmFlags};

/// Dynamic representation of audio data sample format.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SampleFormat {
    F32,
    I32,
    I16,
    I8,
}

impl SampleFormat {
    pub fn does_match_flags(&self, flags: audio_format::LinearPcmFlags) -> bool {
        let is_float = flags.contains(LinearPcmFlags::IS_FLOAT);
        let is_signed_integer = flags.contains(LinearPcmFlags::IS_SIGNED_INTEGER);
        match *self {
            SampleFormat::F32 => is_float && !is_signed_integer,
            SampleFormat::I32 | SampleFormat::I16 | SampleFormat::I8 => {
                is_signed_integer && !is_float
            }
        }
    }

    pub fn from_flags_and_bits_per_sample(
        flags: audio_format::LinearPcmFlags,
        bits_per_sample: u32,
    ) -> Option<Self> {
        // All the currently supported formats are packed.
        if !flags.contains(LinearPcmFlags::IS_PACKED) {
            return None;
        }
        let sample_format = if flags.contains(LinearPcmFlags::IS_FLOAT) {
            match bits_per_sample {
                32 => SampleFormat::F32,
                _ => return None,
            }
        } else if flags.contains(LinearPcmFlags::IS_SIGNED_INTEGER) {
            match bits_per_sample {
                8 => SampleFormat::I8,
                16 => SampleFormat::I16,
                32 => SampleFormat::I32,
                _ => return None,
            }
        } else {
            // TODO: Check whether or not we need to consider other formats, like unsigned ints.
            return None;
        };
        Some(sample_format)
    }

    pub fn size_in_bytes(&self) -> usize {
        use std::mem::size_of;
        match *self {
            SampleFormat::F32 => size_of::<f32>(),
            SampleFormat::I32 => size_of::<i32>(),
            SampleFormat::I16 => size_of::<i16>(),
            SampleFormat::I8 => size_of::<i8>(),
        }
    }
}

/// Audio data sample types.
pub trait Sample {
    /// Dynamic representation of audio data sample format.
    fn sample_format() -> SampleFormat;
}

/// Simplified implementation of the `Sample` trait for sample types.
macro_rules! impl_sample {
    ($($T:ident $format:ident),* $(,)*) => {
        $(
            impl Sample for $T {
                fn sample_format() -> SampleFormat {
                    SampleFormat::$format
                }
            }
        )*
    }
}

impl_sample!(f32 F32, i32 I32, i16 I16, i8 I8);
