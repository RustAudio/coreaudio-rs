//! This module is an attempt at rustifying the OSStatus result.

pub use self::audio::Error as AudioError;
pub use self::audio_codec::Error as AudioCodecError;
pub use self::audio_format::Error as AudioFormatError;
pub use self::audio_unit::Error as AudioUnitError;
use crate::OSStatus;

use objc2_audio_toolbox::{
    kAudioServicesSystemSoundClientTimedOutError, kAudioServicesSystemSoundUnspecifiedError,
};

pub mod audio {
    use crate::OSStatus;
    use objc2_core_audio_types::{
        kAudio_BadFilePathError, kAudio_FileNotFoundError, kAudio_FilePermissionError,
        kAudio_MemFullError, kAudio_ParamError, kAudio_TooManyFilesOpenError,
        kAudio_UnimplementedError,
    };

    #[derive(Copy, Clone, Debug)]
    pub enum Error {
        Unimplemented = kAudio_UnimplementedError as isize,
        FileNotFound = kAudio_FileNotFoundError as isize,
        FilePermission = kAudio_FilePermissionError as isize,
        TooManyFilesOpen = kAudio_TooManyFilesOpenError as isize,
        BadFilePath = kAudio_BadFilePathError as isize,
        Param = kAudio_ParamError as isize,
        MemFull = kAudio_MemFullError as isize,
        Unknown,
    }

    impl Error {
        pub fn from_os_status(os_status: OSStatus) -> Result<(), Error> {
            match os_status {
                0 => Ok(()),
                _ if os_status == kAudio_UnimplementedError => Err(Error::Unimplemented),
                _ if os_status == kAudio_FileNotFoundError => Err(Error::FileNotFound),
                _ if os_status == kAudio_FilePermissionError => Err(Error::FilePermission),
                _ if os_status == kAudio_TooManyFilesOpenError => Err(Error::TooManyFilesOpen),
                _ if os_status == kAudio_BadFilePathError => Err(Error::BadFilePath),
                _ if os_status == kAudio_ParamError => Err(Error::Param),
                _ if os_status == kAudio_MemFullError => Err(Error::MemFull),
                _ => Err(Error::Unknown),
            }
        }

        pub fn as_os_status(&self) -> OSStatus {
            *self as OSStatus
        }
    }

    impl std::error::Error for Error {}

    impl ::std::fmt::Display for Error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
            let description = match *self {
                Error::Unimplemented => "Unimplemented",
                Error::FileNotFound => "File not found",
                Error::FilePermission => "File permission",
                Error::TooManyFilesOpen => "Too many files open",
                Error::BadFilePath => "Bad file path",
                Error::Param => "Param",
                Error::MemFull => "Memory full",
                Error::Unknown => "An unknown error occurred",
            };
            write!(f, "{description}")
        }
    }
}

pub mod audio_codec {
    use crate::OSStatus;
    use objc2_audio_toolbox::{
        kAudioCodecBadDataError, kAudioCodecBadPropertySizeError, kAudioCodecIllegalOperationError,
        kAudioCodecNotEnoughBufferSpaceError, kAudioCodecStateError,
        kAudioCodecUnknownPropertyError, kAudioCodecUnspecifiedError,
        kAudioCodecUnsupportedFormatError,
    };

    #[derive(Copy, Clone, Debug)]
    pub enum Error {
        Unspecified = kAudioCodecUnspecifiedError as isize,
        UnknownProperty = kAudioCodecUnknownPropertyError as isize,
        BadPropertySize = kAudioCodecBadPropertySizeError as isize,
        IllegalOperation = kAudioCodecIllegalOperationError as isize,
        UnsupportedFormat = kAudioCodecUnsupportedFormatError as isize,
        State = kAudioCodecStateError as isize,
        NotEnoughBufferSpace = kAudioCodecNotEnoughBufferSpaceError as isize,
        BadData = kAudioCodecBadDataError as isize,
        Unknown,
    }

    impl Error {
        pub fn from_os_status(os_status: OSStatus) -> Result<(), Error> {
            match os_status {
                0 => Ok(()),
                _ if os_status == kAudioCodecUnspecifiedError => Err(Error::Unspecified),
                _ if os_status == kAudioCodecUnknownPropertyError => Err(Error::UnknownProperty),
                _ if os_status == kAudioCodecBadPropertySizeError => Err(Error::BadPropertySize),
                _ if os_status == kAudioCodecIllegalOperationError => Err(Error::IllegalOperation),
                _ if os_status == kAudioCodecUnsupportedFormatError => {
                    Err(Error::UnsupportedFormat)
                }
                _ if os_status == kAudioCodecStateError => Err(Error::State),
                _ if os_status == kAudioCodecNotEnoughBufferSpaceError => {
                    Err(Error::NotEnoughBufferSpace)
                }
                _ if os_status == kAudioCodecBadDataError => Err(Error::BadData),
                _ => Err(Error::Unknown),
            }
        }

        pub fn as_os_status(&self) -> OSStatus {
            *self as OSStatus
        }
    }

    impl std::error::Error for Error {}

    impl ::std::fmt::Display for Error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
            let description = match *self {
                Error::Unspecified => "Unspecified",
                Error::UnknownProperty => "Unknown property",
                Error::BadPropertySize => "Bad property size",
                Error::IllegalOperation => "Illegal operation",
                Error::UnsupportedFormat => "Unsupported format",
                Error::State => "State",
                Error::NotEnoughBufferSpace => "Not enough buffer space",
                Error::BadData => "Bad data",
                Error::Unknown => "Unknown error occurred",
            };
            write!(f, "{description}")
        }
    }
}

pub mod audio_format {
    use crate::OSStatus;
    use objc2_audio_toolbox::{
        kAudioFormatBadPropertySizeError, kAudioFormatBadSpecifierSizeError,
        kAudioFormatUnknownFormatError, kAudioFormatUnspecifiedError,
        kAudioFormatUnsupportedDataFormatError, kAudioFormatUnsupportedPropertyError,
    };

    // TODO: Finish implementing these values.
    #[derive(Copy, Clone, Debug)]
    pub enum Error {
        Unspecified = kAudioFormatUnspecifiedError as isize,
        UnsupportedProperty = kAudioFormatUnsupportedPropertyError as isize,
        BadPropertySize = kAudioFormatBadPropertySizeError as isize,
        BadSpecifierSize = kAudioFormatBadSpecifierSizeError as isize,
        UnsupportedDataFormat = kAudioFormatUnsupportedDataFormatError as isize,
        UnknownFormat = kAudioFormatUnknownFormatError as isize,
        Unknown,
    }

    impl Error {
        pub fn from_os_status(os_status: OSStatus) -> Result<(), Error> {
            match os_status {
                0 => Ok(()),
                _ if os_status == kAudioFormatUnspecifiedError => Err(Error::Unspecified),
                _ if os_status == kAudioFormatUnsupportedPropertyError => {
                    Err(Error::UnsupportedProperty)
                }
                _ if os_status == kAudioFormatBadPropertySizeError => Err(Error::BadPropertySize),
                _ if os_status == kAudioFormatBadSpecifierSizeError => Err(Error::BadSpecifierSize),
                _ if os_status == kAudioFormatUnsupportedDataFormatError => {
                    Err(Error::UnsupportedDataFormat)
                }
                _ if os_status == kAudioFormatUnknownFormatError => Err(Error::UnknownFormat),
                _ => Err(Error::Unknown),
            }
        }

        pub fn as_os_status(&self) -> OSStatus {
            *self as OSStatus
        }
    }

    impl std::error::Error for Error {}

    impl ::std::fmt::Display for Error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
            let description = match *self {
                Error::Unspecified => "An unspecified error",
                Error::UnsupportedProperty => "The specified property is not supported",
                Error::BadPropertySize => "Bad property size",
                Error::BadSpecifierSize => "Bad specifier size",
                Error::UnsupportedDataFormat => "The specified data format is not supported",
                Error::UnknownFormat => "The specified data format is not a known format",
                Error::Unknown => "Unknown error occurred",
            };
            write!(f, "{description}")
        }
    }
}

pub mod audio_unit {
    use crate::OSStatus;
    use objc2_audio_toolbox::{
        kAudioUnitErr_CannotDoInCurrentContext, kAudioUnitErr_FailedInitialization,
        kAudioUnitErr_FormatNotSupported, kAudioUnitErr_Initialized, kAudioUnitErr_InvalidElement,
        kAudioUnitErr_InvalidFile, kAudioUnitErr_InvalidOfflineRender,
        kAudioUnitErr_InvalidParameter, kAudioUnitErr_InvalidProperty,
        kAudioUnitErr_InvalidPropertyValue, kAudioUnitErr_InvalidScope, kAudioUnitErr_NoConnection,
        kAudioUnitErr_PropertyNotInUse, kAudioUnitErr_PropertyNotWritable,
        kAudioUnitErr_TooManyFramesToProcess, kAudioUnitErr_Unauthorized,
        kAudioUnitErr_Uninitialized,
    };

    #[derive(Copy, Clone, Debug)]
    pub enum Error {
        InvalidProperty = kAudioUnitErr_InvalidProperty as isize,
        InvalidParameter = kAudioUnitErr_InvalidParameter as isize,
        InvalidElement = kAudioUnitErr_InvalidElement as isize,
        NoConnection = kAudioUnitErr_NoConnection as isize,
        FailedInitialization = kAudioUnitErr_FailedInitialization as isize,
        TooManyFramesToProcess = kAudioUnitErr_TooManyFramesToProcess as isize,
        InvalidFile = kAudioUnitErr_InvalidFile as isize,
        FormatNotSupported = kAudioUnitErr_FormatNotSupported as isize,
        Uninitialized = kAudioUnitErr_Uninitialized as isize,
        InvalidScope = kAudioUnitErr_InvalidScope as isize,
        PropertyNotWritable = kAudioUnitErr_PropertyNotWritable as isize,
        CannotDoInCurrentContext = kAudioUnitErr_CannotDoInCurrentContext as isize,
        InvalidPropertyValue = kAudioUnitErr_InvalidPropertyValue as isize,
        PropertyNotInUse = kAudioUnitErr_PropertyNotInUse as isize,
        Initialized = kAudioUnitErr_Initialized as isize,
        InvalidOfflineRender = kAudioUnitErr_InvalidOfflineRender as isize,
        Unauthorized = kAudioUnitErr_Unauthorized as isize,
        Unknown,
    }

    impl Error {
        pub fn from_os_status(os_status: OSStatus) -> Result<(), Error> {
            match os_status {
                _ if os_status == kAudioUnitErr_InvalidProperty => Err(Error::InvalidProperty),
                _ if os_status == kAudioUnitErr_InvalidParameter => Err(Error::InvalidParameter),
                _ if os_status == kAudioUnitErr_InvalidElement => Err(Error::InvalidElement),
                _ if os_status == kAudioUnitErr_NoConnection => Err(Error::NoConnection),
                _ if os_status == kAudioUnitErr_FailedInitialization => {
                    Err(Error::FailedInitialization)
                }
                _ if os_status == kAudioUnitErr_TooManyFramesToProcess => {
                    Err(Error::TooManyFramesToProcess)
                }
                _ if os_status == kAudioUnitErr_InvalidFile => Err(Error::InvalidFile),
                _ if os_status == kAudioUnitErr_FormatNotSupported => {
                    Err(Error::FormatNotSupported)
                }
                _ if os_status == kAudioUnitErr_Uninitialized => Err(Error::Uninitialized),
                _ if os_status == kAudioUnitErr_InvalidScope => Err(Error::InvalidScope),
                _ if os_status == kAudioUnitErr_PropertyNotWritable => {
                    Err(Error::PropertyNotWritable)
                }
                _ if os_status == kAudioUnitErr_CannotDoInCurrentContext => {
                    Err(Error::CannotDoInCurrentContext)
                }
                _ if os_status == kAudioUnitErr_InvalidPropertyValue => {
                    Err(Error::InvalidPropertyValue)
                }
                _ if os_status == kAudioUnitErr_PropertyNotInUse => Err(Error::PropertyNotInUse),
                _ if os_status == kAudioUnitErr_Initialized => Err(Error::Initialized),
                _ if os_status == kAudioUnitErr_InvalidOfflineRender => {
                    Err(Error::InvalidOfflineRender)
                }
                _ if os_status == kAudioUnitErr_Unauthorized => Err(Error::Unauthorized),
                _ => Err(Error::Unknown),
            }
        }

        pub fn as_os_status(&self) -> OSStatus {
            *self as OSStatus
        }
    }

    impl std::error::Error for Error {}

    impl ::std::fmt::Display for Error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
            let description = match *self {
                Error::InvalidProperty => "Invalid property",
                Error::InvalidParameter => "Invalid parameter",
                Error::InvalidElement => "Invalid element",
                Error::NoConnection => "No connection",
                Error::FailedInitialization => "Failed initialization",
                Error::TooManyFramesToProcess => "Too many frames to process",
                Error::InvalidFile => "Invalid file",
                Error::FormatNotSupported => "Format not supported",
                Error::Uninitialized => "Uninitialized",
                Error::InvalidScope => "Invalid scope",
                Error::PropertyNotWritable => "Property not writable",
                Error::CannotDoInCurrentContext => "Cannot do in current context",
                Error::InvalidPropertyValue => "Invalid property value",
                Error::PropertyNotInUse => "Property not in use",
                Error::Initialized => "Initialized",
                Error::InvalidOfflineRender => "Invalid offline render",
                Error::Unauthorized => "Unauthorized",
                Error::Unknown => "Unknown error occurred",
            };
            write!(f, "{description}")
        }
    }
}

/// A wrapper around all possible Core Audio errors.
#[derive(Copy, Clone, Debug)]
pub enum Error {
    Unspecified,
    SystemSoundClientMessageTimedOut,
    NoMatchingDefaultAudioUnitFound,
    RenderCallbackBufferFormatDoesNotMatchAudioUnitStreamFormat,
    NoKnownSubtype,
    NonInterleavedInputOnlySupportsMono,
    UnsupportedSampleRate,
    UnsupportedStreamFormat,
    Audio(AudioError),
    AudioCodec(AudioCodecError),
    AudioFormat(AudioFormatError),
    AudioUnit(AudioUnitError),
    Unknown(OSStatus),
}

impl Error {
    /// Convert an OSStatus to a std Rust Result.
    pub fn from_os_status(os_status: OSStatus) -> Result<(), Error> {
        match os_status {
            0 => Ok(()),
            _ if os_status == kAudioServicesSystemSoundUnspecifiedError => Err(Error::Unspecified),
            _ if os_status == kAudioServicesSystemSoundClientTimedOutError => {
                Err(Error::SystemSoundClientMessageTimedOut)
            }
            _ => {
                match AudioError::from_os_status(os_status) {
                    Ok(()) => return Ok(()),
                    Err(AudioError::Unknown) => (),
                    Err(err) => return Err(Error::Audio(err)),
                }
                match AudioCodecError::from_os_status(os_status) {
                    Ok(()) => return Ok(()),
                    Err(AudioCodecError::Unknown) => (),
                    Err(err) => return Err(Error::AudioCodec(err)),
                }
                match AudioFormatError::from_os_status(os_status) {
                    Ok(()) => return Ok(()),
                    Err(AudioFormatError::Unknown) => (),
                    Err(err) => return Err(Error::AudioFormat(err)),
                }
                match AudioUnitError::from_os_status(os_status) {
                    Ok(()) => return Ok(()),
                    Err(AudioUnitError::Unknown) => (),
                    Err(err) => return Err(Error::AudioUnit(err)),
                }
                Err(Error::Unknown(os_status))
            }
        }
    }

    /// Convert an Error to an OSStatus.
    pub fn as_os_status(&self) -> OSStatus {
        match *self {
            Error::Unspecified => kAudioServicesSystemSoundUnspecifiedError,
            Error::NoMatchingDefaultAudioUnitFound => kAudioServicesSystemSoundUnspecifiedError,
            Error::RenderCallbackBufferFormatDoesNotMatchAudioUnitStreamFormat => {
                kAudioServicesSystemSoundUnspecifiedError
            }
            Error::SystemSoundClientMessageTimedOut => kAudioServicesSystemSoundClientTimedOutError,
            Error::Audio(err) => err as OSStatus,
            Error::AudioCodec(err) => err as OSStatus,
            Error::AudioUnit(err) => err as OSStatus,
            _ => kAudioServicesSystemSoundUnspecifiedError,
        }
    }
}

impl std::error::Error for Error {}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            Error::Unspecified => write!(f, "An unspecified error has occurred"),
            Error::NoMatchingDefaultAudioUnitFound => write!(f, "No matching default audio unit found"),
            Error::RenderCallbackBufferFormatDoesNotMatchAudioUnitStreamFormat =>
                write!(f, "The given render callback buffer format does not match the `AudioUnit` `StreamFormat`"),
            Error::SystemSoundClientMessageTimedOut => write!(f, "The system sound client message timed out"),
            Error::NoKnownSubtype => write!(f, "The type has no known subtypes"),
            Error::NonInterleavedInputOnlySupportsMono => write!(f, "In non-interleaved mode input only supports one channel"),
            Error::UnsupportedSampleRate => write!(f, "The requested sample rate is not available"),
            Error::UnsupportedStreamFormat => write!(f, "The requested stream format is not available"),
            Error::Audio(ref err) => write!(f, "{err}"),
            Error::AudioCodec(ref err) => write!(f, "{err}"),
            Error::AudioFormat(ref err) => write!(f, "{err}"),
            Error::AudioUnit(ref err) => write!(f, "{err}"),
            Error::Unknown(os_status) => write!(f, "An error unknown to the coreaudio-rs API occurred, OSStatus: {os_status}"),

        }
    }
}
