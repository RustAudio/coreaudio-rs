#![feature(box_syntax)]

pub extern crate coreaudio_sys as bindings;
extern crate libc;

pub mod audio_unit;
pub mod error;

