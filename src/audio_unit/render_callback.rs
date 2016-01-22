use bindings::audio_unit as au;
use error::{self, Error};
use libc;
use super::{AudioUnit, Element, Scope, StreamFormat};

pub use self::action_flags::ActionFlags;
pub use self::buffer::Buffer;


/// When `set_render_callback` is called, a closure of this type will be used to wrap the given
/// render callback function.
///
/// This allows the user to provide a custom, more rust-esque callback function type that takes
/// greater advantage of rust's type safety.
pub type InputProcFn = FnMut(*mut au::AudioUnitRenderActionFlags,
                             *const au::AudioTimeStamp,
                             au::UInt32,
                             au::UInt32,
                             *mut au::AudioBufferList) -> au::OSStatus;

/// This type allows us to safely wrap a boxed `RenderCallback` to use within the input proc.
pub struct InputProcFnWrapper {
    callback: Box<InputProcFn>,
}

/// Arguments given to the render callback function.
pub struct Args<B> {
    /// A type wrapping the the buffer that matches the expected audio format.
    pub buffer: B,
    /// Timing information for the callback.
    pub time_stamp: au::AudioTimeStamp,
    /// Flags for configuring audio unit rendering.
    ///
    /// TODO: I can't find any solid documentation on this, but it looks like we should be allowing
    /// the user to also *set* these flags, as `rust-bindgen` generated a `*mut` to them. If that's
    /// the case, then perhaps we should change the return type to `Result<ActionFlags, ()>`?
    pub flags: ActionFlags,
    /// TODO
    pub bus_number: u32,
    /// The number of frames in the buffer.
    pub num_frames: u32,
}

/// Format specific render callback buffers.
pub mod buffer {
    use bindings::audio_unit as au;
    use super::{audio_format, AudioFormat};

    /// Audio data buffer wrappers specific to the `AudioUnit`'s `AudioFormat`.
    pub trait Buffer {
        /// Check whether or not the stream format matches this type of buffer.
        fn does_stream_format_match(&StreamFormat) -> bool;
        /// We must be able to construct Self from arguments given to the `input_proc`.
        fn from_input_proc_args(num_frames: u32, io_data: *mut au::AudioBufferList) -> Self;
    }

    /// A raw pointer to the audio data so that the user may handle it themselves.
    pub struct Custom {
        pub data: *mut au::AudioBufferList,
    }

    /// Arguments that are specific to the `LinearPCM` `AudioFormat` variant.
    pub struct LinearPcm<'a, B> {
        pub data: &'a mut B,
    }

    impl Buffer for Custom {
        fn does_stream_format_match(_: &StreamFormat) -> bool {
            true
        }
        fn from_input_proc_args(_num_frames: u32, io_data: *mut au::AudioBufferList) -> Self {
            Custom { data: io_data }
        }
    }

    /// Dynamic representation of audio data sample format.
    pub enum SampleFormat {
        F32,
        I32,
        I16,
        I8,
    }

    impl SampleFormat {
        fn does_match_linear_pcm_flags(&self, flags: &audio_format::LinearPCMFlags) -> bool {
            let is_float = flags.contains(linear_pcm_flags::IS_FLOAT);
            let is_signed_integer = flags.contains(linear_pcm_flags::IS_SIGNED_INTEGER);
            match *self {
                SampleFormat::F32 => is_float && !is_signed_integer,
                SampleFormat::I32 |
                SampleFormat::I16 |
                SampleFormat::I8 => is_signed_integer && !is_float,
                _ => !is_float,
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
        ($($T:ident $format:expr,)*) => {
            $(
                impl Sample for $T {
                    fn sample_format() -> SampleFormat {
                        SampleFormat::$format
                    }
                }
            )*
        }
    }

    impl_sample!(f32 F32, i32 I32);

    // Implementation for an interleaved linear PCM audio format.
    impl<'a, S> Buffer for LinearPcm<'a, [S]> {

        fn does_stream_format_match(format: &StreamFormat) -> bool {
            use super::audio_format::linear_pcm_flags;
            if let AudioFormat::LinearPCM(flags) = format.audio_format {
                !flags.contains(linear_pcm_flags::IS_NON_INTERLEAVED)
                    && S::sample_format().does_match_linear_pcm_flags(flags);
            } else {
                false
            }
        }

        fn from_input_proc_args(frames: u32, io_data: *mut au::AudioBufferList) -> Self {
            unsafe {
                let au::AudioBuffer { mNumberChannels, mDataByteSize, mData } = io_data.mBuffers[0];

                // Make this an `Err` instead.
                assert!(::std::mem::size_of(S) as u32 == mDataByteSize);

                let data: &mut [S] = {
                    let buffer_len = mNumberChannels as usize * frames as usize;
                    let buffer_ptr = mData as *mut S;
                    slice::from_raw_parts_mut(buffer_ptr, buffer_len)
                };

                LinearPcm { data: data }
            }
        }
    }

    // Implementation for a non-interleaved buffer.
    impl<'a, S> Buffer for LinearPcm<'a, [&'a mut [S]]> {

        fn does_stream_format_match(format: &StreamFormat) -> bool {
            use super::audio_format::linear_pcm_flags;
            if let AudioFormat::LinearPCM(flags) = format.audio_format {
                flags.contains(linear_pcm_flags::IS_NON_INTERLEAVED)
                    && S::sample_format().does_match_linear_pcm_flags(flags);
            } else {
                false
            }
        }

        fn from_input_proc_args(frames: u32, io_data: *mut au::AudioBufferList) -> Self {
            unsafe {
                let au::AudioBufferList { mNumberBuffers, mBuffers } = *io_data;
                for buffer in mBuffers.iter().take(mNumberBuffers as usize) {
                    assert!(buffer.mNumberChannels == 1);
                }
                let data: &'a mut [&'a mut [S]] = {
                    let buffer_slice_ptr = mBuffers.as_mut_ptr();
                    slice::from_raw_parts_mut(buffer_slice_ptr, mNumberBuffers as usize)
                };

                LinearPcm { data: data }
            }
        }
    }

}

pub mod action_flags {
    use bindings::audio_unit as au;

    bitflags!{
        flags ActionFlags: u32 {
            /// Called on a render notification Proc, which is called either before or after the
            /// render operation of the audio unit. If this flag is set, the proc is being called
            /// before the render operation is performed.
            ///
            /// **Available** in OS X v10.0 and later.
            const PRE_RENDER = au::kAudioUnitRenderAction_PreRender,
            /// Called on a render notification Proc, which is called either before or after the
            /// render operation of the audio unit. If this flag is set, the proc is being called
            /// after the render operation is completed.
            ///
            /// **Available** in OS X v10.0 and later.
            const POST_RENDER = au::kAudioUnitRenderAction_PostRender,
            /// This flag can be set in a render input callback (or in the audio unit's render
            /// operation itself) and is used to indicate that the render buffer contains only
            /// silence. It can then be used by the caller as a hint to whether the buffer needs to
            /// be processed or not.
            ///
            /// **Available** in OS X v10.2 and later.
            const OUTPUT_IS_SILENCE = au::kAudioUnitRenderAction_OutputIsSilence,
            /// This is used with offline audio units (of type 'auol'). It is used when an offline
            /// unit is being preflighted, which is performed prior to when the actual offline
            /// rendering actions are performed. It is used for those cases where the offline
            /// process needs it (for example, with an offline unit that normalizes an audio file,
            /// it needs to see all of the audio data first before it can perform its
            /// normalization).
            ///
            /// **Available** in OS X v10.3 and later.
            const OFFLINE_PREFLIGHT = au::kAudioOfflineUnitRenderAction_Preflight,
            /// Once an offline unit has been successfully preflighted, it is then put into its
            /// render mode. This flag is set to indicate to the audio unit that it is now in that
            /// state and that it should perform processing on the input data.
            ///
            /// **Available** in OS X v10.3 and later.
            const OFFLINE_RENDER = au::kAudioOfflineUnitRenderAction_Render,
            /// This flag is set when an offline unit has completed either its preflight or
            /// performed render operation.
            ///
            /// **Available** in OS X v10.3 and later.
            const OFFLINE_COMPLETE = au::kAudioOfflineUnitRenderAction_Complete,
            /// If this flag is set on the post-render call an error was returned by the audio
            /// unit's render operation. In this case, the error can be retrieved through the
            /// `lastRenderError` property and the aduio data in `ioData` handed to the post-render
            /// notification will be invalid.
            ///
            /// **Available** in OS X v10.5 and later.
            const POST_RENDER_ERROR = au::kAudioUnitRenderAction_PostRenderError,
            /// If this flag is set, then checks that are done on the arguments provided to render
            /// are not performed. This can be useful to use to save computation time in situations
            /// where you are sure you are providing the correct arguments and structures to the
            /// various render calls.
            ///
            /// **Available** in OS X v10.7 and later.
            const DO_NOT_CHECK_RENDER_ARGS = au::kAudioUnitRenderAction_DoNotCheckRenderArgs,
        }
    }

    impl ::std::fmt::Display for ActionFlags {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(f, "{:?}", match self.bits() {
                au::kAudioUnitRenderAction_PreRender => "PRE_RENDER",
                au::kAudioUnitRenderAction_PostRender => "POST_RENDER",
                au::kAudioUnitRenderAction_OutputIsSilence => "OUTPUT_IS_SILENCE",
                au::kAudioOfflineUnitRenderAction_Preflight => "OFFLINE_PREFLIGHT",
                au::kAudioOfflineUnitRenderAction_Render => "OFFLINE_RENDER",
                au::kAudioOfflineUnitRenderAction_Complete => "OFFLINE_COMPLETE",
                au::kAudioUnitRenderAction_PostRenderError => "POST_RENDER_ERROR",
                au::kAudioUnitRenderAction_DoNotCheckRenderArgs => "DO_NOT_CHECK_RENDER_ARGS",
                _ => "<Unknown ActionFlags>",
            })
        }
    }
}


impl AudioUnit {

    /// Pass a render callback (aka "Input Procedure") to the **AudioUnit**.
    pub fn set_render_callback<F, A>(&mut self, mut f: F) -> Result<(), Error>
        where F: FnMut(Args<A>) -> Result<(), ()> + 'static,
              A: AudioFormatArgs,
    {
        // First, we'll retrieve the stream format so that we can ensure that the given callback
        // format matches the audio unit's format.
        let stream_format = self.stream_format();

        // If the stream format does not match, return an error indicating this.
        if !A::does_stream_format_match(&stream_format) {
            return Err(Error::RenderCallbackBufferFormatDoesNotMatchAudioUnitStreamFormat);
        }

        // Here, we call the given render callback function within a closure that matches the
        // arguments of the required coreaudio "input_proc".
        //
        // This allows us to take advantage of rust's type system and provide format-specific
        // `Args` types which can be checked at compile time.
        let input_proc_fn = move |io_action_flags: *mut au::AudioUnitRenderActionFlags,
                                  in_time_stamp: *const au::AudioTimeStamp,
                                  in_bus_number: au::UInt32,
                                  in_number_frames: au::UInt32,
                                  io_data: *mut au::AudioBufferList| -> au::OSStatus
        {
            let args = unsafe {
                let audio_format_args = AudioFormatArgs::from_input_proc_args(in_number_frames,
                                                                              io_data);
                let flags = ActionFlags::from_bits(*io_action_flags)
                    .unwrap_or_else(|| ActionFlags::empty());
                Args {
                    format: audio_format_args,
                    time_stamp: *in_time_stamp,
                    flags: flags,
                    bus_number: in_bus_number as u32,
                    num_frames: in_number_frames as u32,
                }
            };

            match f(args) {
                Ok(()) => 0 as au::OSStatus,
                Err(()) => error::Error::Unspecified.to_os_status(),
            }
        };

        let input_proc_fn_wrapper = Box::new(InputProcFnWrapper {
            callback: Box::new(input_proc_fn),
        });

        // Setup render callback. Notice that we relinquish ownership of the Callback
        // here so that it can be used as the C render callback via a void pointer.
        // We do however store the *mut so that we can convert back to a Box<InputProcFnWrapper>
        // within our AudioUnit's Drop implementation (otherwise it would leak).
        let input_proc_fn_wrapper_ptr = Box::into_raw(input_proc_fn_wrapper) as *mut libc::c_void;

        let render_callback = au::AURenderCallbackStruct {
            inputProc: Some(input_proc),
            inputProcRefCon: input_proc_fn_wrapper_ptr,
        };

        try!(self.set_property(au::kAudioUnitProperty_SetRenderCallback,
                               Scope::Input,
                               Element::Output,
                               Some(&render_callback)));

        self.free_render_callback();
        self.maybe_callback = Some(input_proc_fn_wrapper_ptr as *mut InputProcFnWrapper);
        Ok(())
    }

    /// Retrieves ownership over the render callback and drops it.
    pub fn free_render_callback(&mut self) {
        if let Some(callback) = self.maybe_callback.take() {
            // Here, we transfer ownership of the callback back to the current scope so that it
            // is dropped and cleaned up. Without this line, we would leak the Boxed callback.
            let _: Box<InputProcFnWrapper> = unsafe {
                Box::from_raw(callback as *mut InputProcFnWrapper)
            };
        }
    }

}


/// Callback procedure that will be called each time our audio_unit requests audio.
extern "C" fn input_proc(in_ref_con: *mut libc::c_void,
                         io_action_flags: *mut au::AudioUnitRenderActionFlags,
                         in_time_stamp: *const au::AudioTimeStamp,
                         in_bus_number: au::UInt32,
                         in_number_frames: au::UInt32,
                         io_data: *mut au::AudioBufferList) -> au::OSStatus
{
    let wrapper = in_ref_con as *mut InputProcFnWrapper;
    unsafe {
        (*(*wrapper).callback)(io_action_flags,
                               in_time_stamp,
                               in_bus_number,
                               in_number_frames,
                               io_data)
    }
}
