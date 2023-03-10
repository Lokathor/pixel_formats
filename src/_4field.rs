use super::*;

/// sRGB encoded RGB data + linear alpha, `u8` per channel.
///
/// * **GL:** `internalFormat=GL_SRGB8_ALPHA8`, `format=GL_RGBA`,
///   `type=GL_UNSIGNED_BYTE`
/// * **VK:** `VK_FORMAT_R8G8B8A8_SRGB`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct r8g8b8a8_Srgb {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}
impl From<r8g8b8_Srgb> for r8g8b8a8_Srgb {
  /// Adds an alpha channel value of "fully opaque" (`u8::MAX`).
  #[inline]
  #[must_use]
  fn from(r8g8b8_Srgb { r, g, b }: r8g8b8_Srgb) -> Self {
    Self { r, g, b, a: u8::MAX }
  }
}
impl From<r32g32b32a32_Sfloat> for r8g8b8a8_Srgb {
  #[inline]
  #[must_use]
  fn from(r32g32b32a32_Sfloat { r, g, b, a }: r32g32b32a32_Sfloat) -> Self {
    Self {
      r: linear_f32_to_srgb_u8(r),
      g: linear_f32_to_srgb_u8(g),
      b: linear_f32_to_srgb_u8(b),
      a: a.mul_add(255.0, 0.5) as u8,
    }
  }
}

/// Linear RGBA data, `u8` per channel.
///
/// Note that 8 bits is too little precision to encode linear colors well, so
/// this format is *inherently* a not-great option for doing color work.
///
/// * **GL:** `internalFormat=GL_RGBA8`, `format=GL_RGBA`,
///   `type=GL_UNSIGNED_BYTE`
/// * **VK:** `VK_FORMAT_B8G8R8A8_UNORM`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct r8g8b8a8_Unorm {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

/// Linear RGBA data, `f32` per channel.
///
/// * **GL:** `internalFormat=GL_RGBA32F`, `format=GL_RGBA`, `type=GL_FLOAT`
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
impl From<r8g8b8a8_Srgb> for r32g32b32a32_Sfloat {
  #[inline]
  #[must_use]
  fn from(r8g8b8a8_Srgb { r, g, b, a }: r8g8b8a8_Srgb) -> Self {
    Self {
      r: srgb_u8_to_linear_f32(r),
      g: srgb_u8_to_linear_f32(g),
      b: srgb_u8_to_linear_f32(b),
      a: (a as f32) / (u8::MAX as f32),
    }
  }
}
impl From<r8g8b8a8_Unorm> for r32g32b32a32_Sfloat {
  #[inline]
  #[must_use]
  fn from(r8g8b8a8_Unorm { r, g, b, a }: r8g8b8a8_Unorm) -> Self {
    Self {
      r: (r as f32) / (u8::MAX as f32),
      g: (g as f32) / (u8::MAX as f32),
      b: (b as f32) / (u8::MAX as f32),
      a: (a as f32) / (u8::MAX as f32),
    }
  }
}
