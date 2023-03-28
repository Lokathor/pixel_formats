use super::*;

/// sRGB encoded RGB data + linear alpha, `u8` per channel.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct r8g8b8a8_Srgb {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}
impl r8g8b8a8_Srgb {
  pub const OPAQUE_BLACK: Self = Self { r: 0, g: 0, b: 0, a: u8::MAX };
  pub const OPAQUE_WHITE: Self =
    Self { r: u8::MAX, g: u8::MAX, b: u8::MAX, a: u8::MAX };
  pub const TRANSPARENT_BLACK: Self = Self { r: 0, g: 0, b: 0, a: 0 };
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
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct r8g8b8a8_Unorm {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}
impl r8g8b8a8_Unorm {
  pub const OPAQUE_BLACK: Self = Self { r: 0, g: 0, b: 0, a: u8::MAX };
  pub const OPAQUE_WHITE: Self =
    Self { r: u8::MAX, g: u8::MAX, b: u8::MAX, a: u8::MAX };
  pub const TRANSPARENT_BLACK: Self = Self { r: 0, g: 0, b: 0, a: 0 };
}
impl From<r8g8b8_Unorm> for r8g8b8a8_Unorm {
  #[inline]
  #[must_use]
  fn from(r8g8b8_Unorm { r, g, b }: r8g8b8_Unorm) -> Self {
    r8g8b8a8_Unorm { r, g, b, a: u8::MAX }
  }
}
impl From<r32g32b32_Sfloat> for r8g8b8a8_Unorm {
  #[inline]
  #[must_use]
  fn from(x: r32g32b32_Sfloat) -> Self {
    // convert the floats to u8, then add an alpha channel
    Self::from(r8g8b8_Unorm::from(x))
  }
}

/// Linear RGBA data, `u16` per channel.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct r16g16b16a16_Unorm {
  pub r: u16,
  pub g: u16,
  pub b: u16,
  pub a: u16,
}
impl r16g16b16a16_Unorm {
  pub const OPAQUE_BLACK: Self = Self { r: 0, g: 0, b: 0, a: u16::MAX };
  pub const OPAQUE_WHITE: Self =
    Self { r: u16::MAX, g: u16::MAX, b: u16::MAX, a: u16::MAX };
  pub const TRANSPARENT_BLACK: Self = Self { r: 0, g: 0, b: 0, a: 0 };
}

/// Linear RGBA data, `f32` per channel.
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct r32g32b32a32_Sfloat {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32,
}
impl r32g32b32a32_Sfloat {
  pub const OPAQUE_BLACK: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
  pub const OPAQUE_WHITE: Self = Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
  pub const TRANSPARENT_BLACK: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
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
impl From<r32g32b32_Sfloat> for r32g32b32a32_Sfloat {
  #[inline]
  #[must_use]
  fn from(r32g32b32_Sfloat { r, g, b }: r32g32b32_Sfloat) -> Self {
    Self { r, g, b, a: 1.0 }
  }
}
