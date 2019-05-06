#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
//! # Csound
//! This crate contains safe Csound bindings for the csound's library.
//! The supported csound's version is >= 6.12
//! ## What is Csound?
//! Csound is a sound and music computing system. If you want to known more visit:
//! - [Csound webside](https://csound.com/index.html)
//! - [Documentation](http://www.csounds.com/resources/documentation/)
//! - [Community](https://csound.com/community.html)
//! - [Audio examples](https://csound.com/community.html)
//! - [Floss](http://write.flossmanuals.net/csound/preface/)
//! # Hello World
//! A simple Hello world example which reproduces a simple sine wave signal. The call to the csound's perform() method will
//! block the application until the end of the score have been reached.
//! There are another alternatives for non blocking calls to perform csound's scores or csd files. see the examples in the project's source directory
//! or go to [*csound's examples repository*](https://github.com/csound/csoundAPI_examples/tree/master/rust) for more advanced examples and use cases.
//! ```
//! extern crate csound;
//! use csound::*;
//!
//! static score: &str = "<CsoundSynthesizer>
//! <CsOptions>
//! -odac
//! </CsOptions>
//! <CsInstruments>
//!
//! sr = 44100
//! ksmps = 32
//! nchnls = 2
//! 0dbfs  = 1
//!
//! instr 1
//!
//! kamp = .6
//! kcps = 440
//! ifn  = p4
//!
//! asig oscil kamp, kcps, ifn
//!      outs asig,asig
//!
//! endin
//! </CsInstruments>
//! <CsScore>
//! f1 0 16384 10 1
//! i 1 0 2 1
//! e
//! </CsScore>
//! </CsoundSynthesizer>";
//!
//! fn main() {
//!     let mut cs = Csound::new();
//!
//!    /* a message callback*/
//!    let func = |_, message:&str| {
//!        print!("{}", message);
//!    };
//!    cs.message_string_callback(|_, msg: &str| {
//!         print!("{}", msg) );
//!    cs.compile_csd_text(csd).unwrap();
//!    cs.start().unwrap();
//!
//!    cs.perform();
//! }
//! ```

extern crate libc;
#[macro_use]
extern crate bitflags;
extern crate csound_sys;
pub use csound_sys::RTCLOCK;

mod callbacks;
mod channels;
mod csound;
mod enums;
mod rtaudio;
pub use callbacks::FileInfo;
pub use channels::{ChannelHints, ChannelInfo, PvsDataExt};
pub use csound::{BufferPtr, ControlChannelPtr, CircularBuffer, Csound, OpcodeListEntry, Table};
pub use enums::{ChannelData, ControlChannelType, FileTypes, Language, MessageType, Status};
pub use rtaudio::{CsAudioDevice, CsMidiDevice, RtAudioParams};
