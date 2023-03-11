//#![no_std] // needed for f32 functions. TODO: pull in libm or something?
#![allow(nonstandard_style)]

//! Structs for various pixels formats.
//!
//! Format types expose public fields where possible, though formats with
//! bitpacked data must use accessor methods.
//!
//! The docs for each type have notes on the appropriate constants to let that
//! type be used with GL or VK.

mod _1field;
mod _3field;
mod _4field;

pub use _1field::*;
pub use _3field::*;
pub use _4field::*;

#[cfg(feature = "bitfrob")]
mod _3packed;
#[cfg(feature = "bitfrob")]
pub use _3packed::*;

/// Converts a `u8` sRGB encoded value into linear `f32` form.
#[inline]
#[must_use]
pub fn srgb_u8_to_linear_f32(u: u8) -> f32 {
  let f = (u as f32) / (u8::MAX as f32);
  if f < 0.04045 {
    f / 12.92
  } else {
    ((f + 0.055) / 1.055).powf(2.4)
  }
}
