//#![no_std] // needed for f32 functions. TODO: pull in libm or something?
#![allow(nonstandard_style)]

/// sRGB encoded RGB data, `u8` per channel.
///
/// * **GL:** `internalFormat=GL_SRGB8`, `format=GL_RGB`,
///   `type=GL_UNSIGNED_BYTE`
/// * **VK:** `VK_FORMAT_R8G8B8_SRGB`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct r8g8b8_Srgb {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

/// Linear RGB data, `u8` per channel.
///
/// Note that 8 bits is too little precision to encode linear colors well, so
/// this format is *inherently* a not-great option for doing color work.
///
/// * **GL:** `internalFormat=GL_RGB8`, `format=GL_RGB`, `type=GL_UNSIGNED_BYTE`
/// * **VK:** `VK_FORMAT_R8G8B8_UNORM`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct r8g8b8_Unorm {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

/// Linear RGB data, `f32` per channel.
///
/// * **VK:** `VK_FORMAT_R32G32B32_SFLOAT`
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct r32g32b32_Sfloat {
  pub r: f32,
  pub g: f32,
  pub b: f32,
}
impl From<r8g8b8_Srgb> for r32g32b32_Sfloat {
  #[inline]
  #[must_use]
  fn from(r8g8b8_Srgb{r,g,b}: r8g8b8_Srgb) -> Self{
    r32g32b32_Sfloat {
      r: srgb_u8_to_linear_f32(r),
      g: srgb_u8_to_linear_f32(g),
      b: srgb_u8_to_linear_f32(b),
    }
  }
}

/// Linear RGBA data, `f32` per channel.
///
/// * **VK:** `VK_FORMAT_R32G32B32A32_SFLOAT`
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct r32g32b32a32_Sfloat {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32,
}

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
