use bindings::audio_unit as au;
use error::{self, Error};
use libc;
use super::{AudioUnit, Element, Scope};

pub use self::action_flags::ActionFlags;
pub use self::data::Data;

use std::marker::PhantomData;

/// When `set_render_callback` is called, a closure of this type will be used to wrap the given
/// render callback function.
///
/// This allows the user to provide a custom, more rust-esque callback function type that takes
/// greater advantage of rust's type safety.
pub type InputProcFn<'a> = FnMut(*mut au::AudioUnitRenderActionFlags,
                             *const au::AudioTimeStamp,
                             au::UInt32,
                             au::UInt32,
                             *mut au::AudioBufferList) -> au::OSStatus + 'a;

/// This type allows us to safely wrap a boxed `RenderCallback` to use within the input proc.
pub struct InputProcFnWrapper<'a> {
    callback: Box<InputProcFn<'a>>,
    ph: PhantomData<&'a ()>
}

/// Arguments given to the render callback function.
pub struct Args<D> {
    /// A type wrapping the the buffer that matches the expected audio format.
    pub data: D,
    /// Timing information for the callback.
    pub time_stamp: au::AudioTimeStamp,
    /// TODO
    pub bus_number: u32,
    /// The number of frames in the buffer as `usize` for easier indexing.
    pub num_frames: usize,
    /// Flags for configuring audio unit rendering.
    ///
    /// This parameter lets a callback provide various hints to the audio unit.
    ///
    /// For example: if there is no audio to process, we can insert the `OUTPUT_IS_SILENCE` flag to
    /// indicate to the audio unit that the buffer does not need to be processed.
    pub flags: action_flags::Handle,
}


/// Format specific render callback data.
pub mod data {
    use bindings::audio_unit as au;
    use std::marker::PhantomData;
    use std::{iter, slice};
    use super::super::StreamFormat;
    use super::super::Sample;

    /// Audio data wrappers specific to the `AudioUnit`'s `AudioFormat`.
    pub trait Data {
        /// Check whether or not the stream format matches this type of data.
        fn does_stream_format_match(&StreamFormat) -> bool;
        /// We must be able to construct Self from arguments given to the `input_proc`.
        unsafe fn from_input_proc_args(num_frames: u32, io_data: *mut au::AudioBufferList) -> Self;
    }

    /// A raw pointer to the audio data so that the user may handle it themselves.
    pub struct Raw {
        pub data: *mut au::AudioBufferList,
    }

    impl Data for Raw {
        fn does_stream_format_match(_: &StreamFormat) -> bool {
            true
        }
        unsafe fn from_input_proc_args(_num_frames: u32, io_data: *mut au::AudioBufferList) -> Self {
            Raw { data: io_data }
        }
    }

    // TODO: When testing with the `HalOutput` audio unit it seemed not to allow interleaved data.
    // Even though the `IS_NON_INTERLEAVED` flag was not set, the audio unit continues to deliver
    // the audio as non-interleaved samples anyway. Investigate this, as it might not even be
    // possible to use this type with audio units!
    //
    // /// An interleaved linear PCM buffer with samples of type `S`.
    // pub struct Interleaved<'a, S> {
    //     pub buffer: &'a mut [S],
    //     pub channels: usize,
    // }

    // // Implementation for an interleaved linear PCM audio format.
    // impl<'a, S> Data for Interleaved<'a, S>
    //     where S: Sample,
    // {
    //     fn does_stream_format_match(format: &StreamFormat) -> bool {
    //         !format.flags.contains(linear_pcm_flags::IS_NON_INTERLEAVED) &&
    //             S::sample_format().does_match_flags(format.flags)
    //     }

    //     #[allow(non_snake_case)]
    //     unsafe fn from_input_proc_args(frames: u32, io_data: *mut au::AudioBufferList) -> Self {
    //         // We're expecting a single interleaved buffer which will be the first in the array.
    //         let au::AudioBuffer { mNumberChannels, mDataByteSize, mData } = (*io_data).mBuffers[0];

    //         // Ensure that the size of the data matches the size of the sample format
    //         // multiplied by the number of frames.
    //         //
    //         // TODO: Return an Err instead of `panic`ing.
    //         let buffer_len = frames as usize * mNumberChannels as usize;
    //         let expected_size = ::std::mem::size_of::<S>() * buffer_len;
    //         assert!(mDataByteSize as usize == expected_size);

    //         let buffer: &mut [S] = {
    //             let buffer_ptr = mData as *mut S;
    //             slice::from_raw_parts_mut(buffer_ptr, buffer_len)
    //         };

    //         Interleaved {
    //             buffer: buffer,
    //             channels: mNumberChannels as usize,
    //         }
    //     }
    // }

    /// A wrapper around the pointer to the `mBuffers` array.
    pub struct NonInterleaved<S> {
        /// A pointer to the first buffer.
        ///
        /// TODO: Work out why this works and `&'a mut [au::AudioBuffer]` does not!
        /// Perhaps use a raw pointer instead if a slice won't work.
        buffers: [au::AudioBuffer; 128],
        num_buffers: usize,
        /// The number of frames in each channel.
        frames: usize,
        sample_format: PhantomData<S>,
    }

    /// An iterator produced by a `NoneInterleaved`, yielding a reference to each channel.
    pub struct Channels<'a, S: 'a> {
        buffers: iter::Take<slice::Iter<'a, au::AudioBuffer>>,
        frames: usize,
        sample_format: PhantomData<S>,
    }

    /// An iterator produced by a `NoneInterleaved`, yielding a mutable reference to each channel.
    pub struct ChannelsMut<'a, S: 'a> {
        buffers: iter::Take<slice::IterMut<'a, au::AudioBuffer>>,
        frames: usize,
        sample_format: PhantomData<S>,
    }

    unsafe impl<S> Send for NonInterleaved<S> where S: Send {}

    impl<'a, S> Iterator for Channels<'a, S> {
        type Item = &'a [S];
        #[allow(non_snake_case)]
        fn next(&mut self) -> Option<Self::Item> {
            self.buffers.next().map(|&au::AudioBuffer { mNumberChannels, mData, .. }| {
                let len = mNumberChannels as usize * self.frames;
                let ptr = mData as *mut S;
                unsafe { slice::from_raw_parts(ptr, len) }
            })
        }
    }

    impl<'a, S> Iterator for ChannelsMut<'a, S> {
        type Item = &'a mut [S];
        #[allow(non_snake_case)]
        fn next(&mut self) -> Option<Self::Item> {
            self.buffers.next().map(|&mut au::AudioBuffer { mNumberChannels, mData, .. }| {
                let len = mNumberChannels as usize * self.frames;
                let ptr = mData as *mut S;
                unsafe { slice::from_raw_parts_mut(ptr, len) }
            })
        }
    }

    impl<S> NonInterleaved<S> {

        /// An iterator yielding a reference to each channel in the array.
        pub fn channels(&self) -> Channels<S> {
            Channels {
                buffers: self.buffers.iter().take(self.num_buffers),
                frames: self.frames,
                sample_format: PhantomData,
            }
        }

        /// An iterator yielding a mutable reference to each channel in the array.
        pub fn channels_mut(&mut self) -> ChannelsMut<S> {
            ChannelsMut {
                buffers: self.buffers.iter_mut().take(self.num_buffers),
                frames: self.frames,
                sample_format: PhantomData,
            }
        }

    }

    // Implementation for a non-interleaved linear PCM audio format.
    impl<S> Data for NonInterleaved<S>
        where S: Sample,
    {
        fn does_stream_format_match(format: &StreamFormat) -> bool {
            // TODO: This is never set, even though the default ABSD on OS X is non-interleaved!
            // Should really investigate why this is.
            // format.flags.contains(linear_pcm_flags::IS_NON_INTERLEAVED) &&
                S::sample_format().does_match_flags(format.flags)
        }

        #[allow(non_snake_case)]
        unsafe fn from_input_proc_args(frames: u32, io_data: *mut au::AudioBufferList) -> Self {
            let au::AudioBufferList { mNumberBuffers, mBuffers } = *io_data;
            // TODO: This should be a raw pointer to the first elem in the array and fixed in
            // coreaudio-sys because a 128 elem FSA makes no sense!
            let buffers: [au::AudioBuffer; 128] = mBuffers;
            NonInterleaved {
                buffers: buffers,
                num_buffers: mNumberBuffers as usize,
                frames: frames as usize,
                sample_format: PhantomData,
            }
        }
    }

}

pub mod action_flags {
    use bindings::audio_unit as au;

    bitflags!{
        pub struct ActionFlags: u32 {
            /// Called on a render notification Proc, which is called either before or after the
            /// render operation of the audio unit. If this flag is set, the proc is being called
            /// before the render operation is performed.
            ///
            /// **Available** in OS X v10.0 and later.
            const PRE_RENDER = au::kAudioUnitRenderAction_PreRender;
            /// Called on a render notification Proc, which is called either before or after the
            /// render operation of the audio unit. If this flag is set, the proc is being called
            /// after the render operation is completed.
            ///
            /// **Available** in OS X v10.0 and later.
            const POST_RENDER = au::kAudioUnitRenderAction_PostRender;
            /// This flag can be set in a render input callback (or in the audio unit's render
            /// operation itself) and is used to indicate that the render buffer contains only
            /// silence. It can then be used by the caller as a hint to whether the buffer needs to
            /// be processed or not.
            ///
            /// **Available** in OS X v10.2 and later.
            const OUTPUT_IS_SILENCE = au::kAudioUnitRenderAction_OutputIsSilence;
            /// This is used with offline audio units (of type 'auol'). It is used when an offline
            /// unit is being preflighted, which is performed prior to when the actual offline
            /// rendering actions are performed. It is used for those cases where the offline
            /// process needs it (for example, with an offline unit that normalizes an audio file,
            /// it needs to see all of the audio data first before it can perform its
            /// normalization).
            ///
            /// **Available** in OS X v10.3 and later.
            const OFFLINE_PREFLIGHT = au::kAudioOfflineUnitRenderAction_Preflight;
            /// Once an offline unit has been successfully preflighted, it is then put into its
            /// render mode. This flag is set to indicate to the audio unit that it is now in that
            /// state and that it should perform processing on the input data.
            ///
            /// **Available** in OS X v10.3 and later.
            const OFFLINE_RENDER = au::kAudioOfflineUnitRenderAction_Render;
            /// This flag is set when an offline unit has completed either its preflight or
            /// performed render operation.
            ///
            /// **Available** in OS X v10.3 and later.
            const OFFLINE_COMPLETE = au::kAudioOfflineUnitRenderAction_Complete;
            /// If this flag is set on the post-render call an error was returned by the audio
            /// unit's render operation. In this case, the error can be retrieved through the
            /// `lastRenderError` property and the audio data in `ioData` handed to the post-render
            /// notification will be invalid.
            ///
            /// **Available** in OS X v10.5 and later.
            const POST_RENDER_ERROR = au::kAudioUnitRenderAction_PostRenderError;
            /// If this flag is set, then checks that are done on the arguments provided to render
            /// are not performed. This can be useful to use to save computation time in situations
            /// where you are sure you are providing the correct arguments and structures to the
            /// various render calls.
            ///
            /// **Available** in OS X v10.7 and later.
            const DO_NOT_CHECK_RENDER_ARGS = au::kAudioUnitRenderAction_DoNotCheckRenderArgs;
        }
    }

    /// A safe handle around the `AudioUnitRenderActionFlags` pointer provided by the render
    /// callback.
    ///
    /// This type lets a callback provide various hints to the audio unit.
    ///
    /// For example: if there is no audio to process, we can insert the `OUTPUT_IS_SILENCE` flag to
    /// indicate to the audio unit that the buffer does not need to be processed.
    pub struct Handle {
        ptr: *mut au::AudioUnitRenderActionFlags,
    }

    impl Handle {

        /// Retrieve the current state of the `ActionFlags`.
        pub fn get(&self) -> ActionFlags {
            ActionFlags::from_bits_truncate(unsafe { *self.ptr })
        }

        fn set(&mut self, flags: ActionFlags) {
            unsafe { *self.ptr = flags.bits() }
        }

        /// The raw value of the flags currently stored.
        pub fn bits(&self) -> u32 {
            self.get().bits()
        }

        /// Returns `true` if no flags are currently stored.
        pub fn is_empty(&self) -> bool {
            self.get().is_empty()
        }

        /// Returns `true` if all flags are currently stored.
        pub fn is_all(&self) -> bool {
            self.get().is_all()
        }

        /// Returns `true` if there are flags common to both `self` and `other`.
        pub fn intersects(&self, other: ActionFlags) -> bool {
            self.get().intersects(other)
        }

        /// Returns `true` if all of the flags in `other` are contained within `self`.
        pub fn contains(&self, other: ActionFlags) -> bool {
            self.get().contains(other)
        }

        /// Insert the specified flags in-place.
        pub fn insert(&mut self, other: ActionFlags) {
            let mut flags = self.get();
            flags.insert(other);
            self.set(flags);
        }

        /// Remove the specified flags in-place.
        pub fn remove(&mut self, other: ActionFlags) {
            let mut flags = self.get();
            flags.remove(other);
            self.set(flags);
        }

        /// Toggles the specified flags in-place.
        pub fn toggle(&mut self, other: ActionFlags) {
            let mut flags = self.get();
            flags.toggle(other);
            self.set(flags);
        }

        /// Wrap the given pointer with a `Handle`.
        pub fn from_ptr(ptr: *mut au::AudioUnitRenderActionFlags) -> Self {
            Handle { ptr: ptr }
        }

    }

    unsafe impl Send for Handle {}

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


impl<'a> AudioUnit<'a> {

    /// Pass a render callback (aka "Input Procedure") to the **AudioUnit**.
    pub fn set_render_callback<'b:'a, F:'b, D>(&mut self, mut f: F) -> Result<(), Error>
        where F: FnMut(Args<D>) -> Result<(), ()> + 'b,
              D: Data,
    {
        // First, we'll retrieve the stream format so that we can ensure that the given callback
        // format matches the audio unit's format.
        let stream_format = try!(self.stream_format());

        // If the stream format does not match, return an error indicating this.
        if !D::does_stream_format_match(&stream_format) {
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
                let data = D::from_input_proc_args(in_number_frames, io_data);
                let flags = action_flags::Handle::from_ptr(io_action_flags);
                Args {
                    data: data,
                    time_stamp: *in_time_stamp,
                    flags: flags,
                    bus_number: in_bus_number as u32,
                    num_frames: in_number_frames as usize,
                }
            };

            match f(args) {
                Ok(()) => 0 as au::OSStatus,
                Err(()) => error::Error::Unspecified.to_os_status(),
            }
        };

        let input_proc_fn_wrapper = Box::new(InputProcFnWrapper::<'b> {
            callback: Box::new(input_proc_fn),
            ph: PhantomData
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
