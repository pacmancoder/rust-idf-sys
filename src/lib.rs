//! Module contains raw IDF framework bindings. Please do not use this module directly.
//! Use idf-hal module instead.
//!
//! Bindings were generated using bindgen and then manually refactored and separated into
//! different modules.

#![no_std]

// Most identifiers were directly copied from IDF framework sources, so we will ask rustc to
// be silent about
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod ffi;
pub mod error;
pub mod network_adapter;
pub mod wifi;
pub mod system_event;
pub mod freertos;
pub mod gpio;
pub mod pwm;
pub mod watchdog;