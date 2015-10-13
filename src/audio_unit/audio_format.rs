//! Typification of the various AudioFormat codes and flags offered by the Core Audio API.
//! 
//! See the Core Audio Data Types Reference
//! [here](https://developer.apple.com/library/mac/documentation/MusicAudio/Reference/CoreAudioDataTypesRef/#//apple_ref/doc/constant_group/Audio_Data_Format_Identifiers) for more info.


use libc;

/// Represents the kAudioFormat types in the form of an enum.
#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum AudioFormat {
    LinearPCM(Option<LinearPCMFlag>),     // = 1819304813,
    AC3,                                  // = 1633889587,
    F60958AC3(Option<StandardFlag>),      // = 1667326771,
    AppleIMA4,                            // = 1768775988,
    MPEG4AAC(Option<Mpeg4Object>),        // = 1633772320,
    MPEG4CELP(Option<Mpeg4Object>),       // = 1667591280,
    MPEG4HVXC(Option<Mpeg4Object>),       // = 1752594531,
    MPEG4TwinVQ(Option<Mpeg4Object>),     // = 1953986161,
    MACE3,                                // = 1296122675,
    MACE6,                                // = 1296122678,
    ULaw,                                 // = 1970037111,
    ALaw,                                 // = 1634492791,
    QDesign,                              // = 1363430723,
    QDesign2,                             // = 1363430706,
    QUALCOMM,                             // = 1365470320,
    MPEGLayer1,                           // = 778924081,
    MPEGLayer2,                           // = 778924082,
    MPEGLayer3,                           // = 778924083,
    TimeCode(Option<AudioTimeStampFlag>), // = 1953066341,
    MIDIStream,                           // = 1835623529,
    ParameterValueStream,                 // = 1634760307,
    AppleLossless,                        // = 1634492771,
    MPEG4AAC_HE,                          // = 1633772392,
    MPEG4AAC_LD,                          // = 1633772396,
    MPEG4AAC_ELD,                         // = 1633772389,
    MPEG4AAC_ELD_SBR,                     // = 1633772390,
    MPEG4AAC_ELD_V2,                      // = 1633772391,
    MPEG4AAC_HE_V2,                       // = 1633772400,
    MPEG4AAC_Spatial,                     // = 1633772403,
    AMR,                                  // = 1935764850,
    AMR_WB,                               // = 1935767394,
    Audible,                              // = 1096107074,
    iLBC,                                 // = 1768710755,
    DVIIntelIMA,                          // = 1836253201,
    MicrosoftGSM,                         // = 1836253233,
    AES3,                                 // = 1634038579,
}

impl AudioFormat {

    /// Convert from C format and flag to Rust enum.
    pub fn from_format_and_flag(format: libc::c_uint, flag: Option<u32>) -> Option<AudioFormat> {
        match (format, flag) {
            (1819304813, Some(i)) => Some(AudioFormat::LinearPCM(LinearPCMFlag::from_u32(i))),
            (1633889587, _)       => Some(AudioFormat::AC3),
            (1667326771, Some(i)) => Some(AudioFormat::F60958AC3(StandardFlag::from_u32(i))),
            (1768775988, _)       => Some(AudioFormat::AppleIMA4),
            (1633772320, Some(i)) => Some(AudioFormat::MPEG4AAC(Mpeg4Object::from_u32(i))),
            (1667591280, Some(i)) => Some(AudioFormat::MPEG4CELP(Mpeg4Object::from_u32(i))),
            (1752594531, Some(i)) => Some(AudioFormat::MPEG4HVXC(Mpeg4Object::from_u32(i))),
            (1953986161, Some(i)) => Some(AudioFormat::MPEG4TwinVQ(Mpeg4Object::from_u32(i))),
            (1296122675, _)       => Some(AudioFormat::MACE3),
            (1296122678, _)       => Some(AudioFormat::MACE6),
            (1970037111, _)       => Some(AudioFormat::ULaw),
            (1634492791, _)       => Some(AudioFormat::ALaw),
            (1363430723, _)       => Some(AudioFormat::QDesign),
            (1363430706, _)       => Some(AudioFormat::QDesign2),
            (1365470320, _)       => Some(AudioFormat::QUALCOMM),
            (778924081, _)        => Some(AudioFormat::MPEGLayer1),
            (778924082, _)        => Some(AudioFormat::MPEGLayer2),
            (778924083, _)        => Some(AudioFormat::MPEGLayer3),
            (1953066341, Some(i)) => Some(AudioFormat::TimeCode(AudioTimeStampFlag::from_u32(i))),
            (1835623529, _)       => Some(AudioFormat::MIDIStream),
            (1634760307, _)       => Some(AudioFormat::ParameterValueStream),
            (1634492771, _)       => Some(AudioFormat::AppleLossless),
            (1633772392, _)       => Some(AudioFormat::MPEG4AAC_HE),
            (1633772396, _)       => Some(AudioFormat::MPEG4AAC_LD),
            (1633772389, _)       => Some(AudioFormat::MPEG4AAC_ELD),
            (1633772390, _)       => Some(AudioFormat::MPEG4AAC_ELD_SBR),
            (1633772391, _)       => Some(AudioFormat::MPEG4AAC_ELD_V2),
            (1633772400, _)       => Some(AudioFormat::MPEG4AAC_HE_V2),
            (1633772403, _)       => Some(AudioFormat::MPEG4AAC_Spatial),
            (1935764850, _)       => Some(AudioFormat::AMR),
            (1935767394, _)       => Some(AudioFormat::AMR_WB),
            (1096107074, _)       => Some(AudioFormat::Audible),
            (1768710755, _)       => Some(AudioFormat::iLBC),
            (1836253201, _)       => Some(AudioFormat::DVIIntelIMA),
            (1836253233, _)       => Some(AudioFormat::MicrosoftGSM),
            (1634038579, _)       => Some(AudioFormat::AES3),
            _                     => None,
        }
    }

    /// Convert from the Rust enum to the C format and flag.
    pub fn to_format_and_flag(&self) -> (libc::c_uint, Option<u32>) {
        match *self {
            AudioFormat::LinearPCM(flag)      => (1819304813, flag.map(|flag| flag as u32)),
            AudioFormat::AC3                  => (1633889587, None),
            AudioFormat::F60958AC3(flag)      => (1667326771, flag.map(|flag| flag as u32)),
            AudioFormat::AppleIMA4            => (1768775988, None),
            AudioFormat::MPEG4AAC(flag)       => (1633772320, flag.map(|flag| flag as u32)),
            AudioFormat::MPEG4CELP(flag)      => (1667591280, flag.map(|flag| flag as u32)),
            AudioFormat::MPEG4HVXC(flag)      => (1752594531, flag.map(|flag| flag as u32)),
            AudioFormat::MPEG4TwinVQ(flag)    => (1953986161, flag.map(|flag| flag as u32)),
            AudioFormat::MACE3                => (1296122675, None),
            AudioFormat::MACE6                => (1296122678, None),
            AudioFormat::ULaw                 => (1970037111, None),
            AudioFormat::ALaw                 => (1634492791, None),
            AudioFormat::QDesign              => (1363430723, None),
            AudioFormat::QDesign2             => (1363430706, None),
            AudioFormat::QUALCOMM             => (1365470320, None),
            AudioFormat::MPEGLayer1           => (778924081, None),
            AudioFormat::MPEGLayer2           => (778924082, None),
            AudioFormat::MPEGLayer3           => (778924083, None),
            AudioFormat::TimeCode(flag)       => (1953066341, flag.map(|flag| flag as u32)),
            AudioFormat::MIDIStream           => (1835623529, None),
            AudioFormat::ParameterValueStream => (1634760307, None),
            AudioFormat::AppleLossless        => (1634492771, None),
            AudioFormat::MPEG4AAC_HE          => (1633772392, None),
            AudioFormat::MPEG4AAC_LD          => (1633772396, None),
            AudioFormat::MPEG4AAC_ELD         => (1633772389, None),
            AudioFormat::MPEG4AAC_ELD_SBR     => (1633772390, None),
            AudioFormat::MPEG4AAC_ELD_V2      => (1633772391, None),
            AudioFormat::MPEG4AAC_HE_V2       => (1633772400, None),
            AudioFormat::MPEG4AAC_Spatial     => (1633772403, None),
            AudioFormat::AMR                  => (1935764850, None),
            AudioFormat::AMR_WB               => (1935767394, None),
            AudioFormat::Audible              => (1096107074, None),
            AudioFormat::iLBC                 => (1768710755, None),
            AudioFormat::DVIIntelIMA          => (1836253201, None),
            AudioFormat::MicrosoftGSM         => (1836253233, None),
            AudioFormat::AES3                 => (1634038579, None),
        }
    }

}

#[derive(Copy, Clone, Debug)]
pub enum LinearPCMFlag {
    IsFloat = 1,
    IsBigEndian = 2,
    IsSignedInteger = 4,
    IsPacked = 8,
    IsAlignedHigh = 16,
    IsNonInterleaved = 32,
    IsNonMixable = 64,
    FlagsSampleFractionShift = 7,
    FlagsSampleFractionMask = 8064,
}

impl LinearPCMFlag {
    pub fn from_u32(i: u32) -> Option<LinearPCMFlag> {
        match i {
            1           => Some(LinearPCMFlag::IsFloat),
            2           => Some(LinearPCMFlag::IsBigEndian),
            4           => Some(LinearPCMFlag::IsSignedInteger),
            8           => Some(LinearPCMFlag::IsPacked),
            16          => Some(LinearPCMFlag::IsAlignedHigh),
            32          => Some(LinearPCMFlag::IsNonInterleaved),
            64          => Some(LinearPCMFlag::IsNonMixable),
            7           => Some(LinearPCMFlag::FlagsSampleFractionShift),
            8064        => Some(LinearPCMFlag::FlagsSampleFractionMask),
            _           => None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum StandardFlag {
    IsFloat = 1,
    IsBigEndian = 2,
    IsSignedInteger = 4,
    IsPacked = 8,
    IsAlignedHigh = 16,
    IsNonInterleaved = 32,
    IsNonMixable = 64,
}

impl StandardFlag {
    pub fn from_u32(i: u32) -> Option<StandardFlag> {
        match i {
            1           => Some(StandardFlag::IsFloat),
            2           => Some(StandardFlag::IsBigEndian),
            4           => Some(StandardFlag::IsSignedInteger),
            8           => Some(StandardFlag::IsPacked),
            16          => Some(StandardFlag::IsAlignedHigh),
            32          => Some(StandardFlag::IsNonInterleaved),
            64          => Some(StandardFlag::IsNonMixable),
            _           => None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum AppleLosslessFlag {
    Bit16SourceData = 1,
    Bit20SourceData = 2,
    Bit24SourceData = 3,
    Bit32SourceData = 4,
}

impl AppleLosslessFlag {
    pub fn from_u32(i: u32) -> Option<AppleLosslessFlag> {
        match i {
            1 => Some(AppleLosslessFlag::Bit16SourceData),
            2 => Some(AppleLosslessFlag::Bit20SourceData),
            3 => Some(AppleLosslessFlag::Bit24SourceData),
            4 => Some(AppleLosslessFlag::Bit32SourceData),
            _ => None
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum Mpeg4Object {
    AAC_Main = 1,
    AAC_LC = 2,
    AAC_SSR = 3,
    AAC_LTP = 4,
    AAC_SBR = 5,
    AAC_Scalable = 6,
    TwinVQ = 7,
    CELP = 8,
    HVXC = 9,
}

impl Mpeg4Object {
    pub fn from_u32(i: u32) -> Option<Mpeg4Object> {
        match i {
            1 => Some(Mpeg4Object::AAC_Main),
            2 => Some(Mpeg4Object::AAC_LC),
            3 => Some(Mpeg4Object::AAC_SSR),
            4 => Some(Mpeg4Object::AAC_LTP),
            5 => Some(Mpeg4Object::AAC_SBR),
            6 => Some(Mpeg4Object::AAC_Scalable),
            7 => Some(Mpeg4Object::TwinVQ),
            8 => Some(Mpeg4Object::CELP),
            9 => Some(Mpeg4Object::HVXC),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum AudioTimeStampFlag {
    SampleTimeValid = 1,
    HostTimeValid = 2,
    RateScalarValid = 4,
    WordClockTimeValid = 8,
    SMPTETimeValid = 16,
}

impl AudioTimeStampFlag {
    pub fn from_u32(i: u32) -> Option<AudioTimeStampFlag> {
        match i {
            1  => Some(AudioTimeStampFlag::SampleTimeValid),
            2  => Some(AudioTimeStampFlag::HostTimeValid),
            4  => Some(AudioTimeStampFlag::RateScalarValid),
            8  => Some(AudioTimeStampFlag::WordClockTimeValid),
            16 => Some(AudioTimeStampFlag::SMPTETimeValid),
            _  => None
        }
    }
}

