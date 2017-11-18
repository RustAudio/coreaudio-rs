# coreaudio-rs [![Build Status](https://travis-ci.org/RustAudio/coreaudio-rs.svg?branch=master)](https://travis-ci.org/RustAudio/coreaudio-rs) [![Crates.io](https://img.shields.io/crates/v/coreaudio-rs.svg)](https://crates.io/crates/coreaudio-rs) [![Crates.io](https://img.shields.io/crates/l/coreaudio-rs.svg)](https://github.com/RustAudio/coreaudio-rs/blob/master/LICENSE-MIT)

ADDED: This is a fork of the original coreaudio-rs whose only difference with the original is that audio streams are not forced to have the static lifetime.  Will be removed from this account if and when our PR is implemented.

A friendly rust interface for [Apple's Core Audio API](https://developer.apple.com/library/ios/documentation/MusicAudio/Conceptual/CoreAudioOverview/CoreAudioEssentials/CoreAudioEssentials.html).

This crate aims to expose and wrap the functionality of the original C API in a zero-cost, safe, Rust-esque manner.

If you just want direct access to the unsafe bindings, use [coreaudio-sys](https://crates.io/crates/coreaudio-sys).

[Documentation](http://rustaudio.github.io/coreaudio-rs/coreaudio)
