use super::*;

/// sRGB encoded RGB data, `u8` per channel.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct r8g8b8_Srgb {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}
impl r8g8b8_Srgb {
  pub const BLACK: Self = Self { r: 0, g: 0, b: 0 };
  pub const WHITE: Self = Self { r: u8::MAX, g: u8::MAX, b: u8::MAX };
}
impl From<r32g32b32_Sfloat> for r8g8b8_Srgb {
  #[inline]
  #[must_use]
  fn from(sf: r32g32b32_Sfloat) -> Self {
    r8g8b8_Srgb {
      r: linear_f32_to_srgb_u8(sf.r),
      g: linear_f32_to_srgb_u8(sf.g),
      b: linear_f32_to_srgb_u8(sf.b),
    }
  }
}

/// Linear RGB data, `u8` per channel.
///
/// Note that 8 bits is too little precision to encode linear colors well, so
/// this format is *inherently* a not-great option for doing color work.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct r8g8b8_Unorm {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}
impl r8g8b8_Unorm {
  pub const BLACK: Self = Self { r: 0, g: 0, b: 0 };
  pub const WHITE: Self = Self { r: u8::MAX, g: u8::MAX, b: u8::MAX };
}
impl From<r32g32b32_Sfloat> for r8g8b8_Unorm {
  #[inline]
  #[must_use]
  fn from(sf: r32g32b32_Sfloat) -> Self {
    r8g8b8_Unorm {
      r: sf.r.mul_add(255.0, 0.5) as u8,
      g: sf.g.mul_add(255.0, 0.5) as u8,
      b: sf.b.mul_add(255.0, 0.5) as u8,
    }
  }
}
#[test]
#[allow(bad_style)]
fn r8g8b8_Unorm_from_r32g32b32_Sfloat() {
  let un = r8g8b8_Unorm::from(r32g32b32_Sfloat { r: 1.0, g: 0.5, b: 0.0 });
  assert_eq!(un.r, 255);
  assert_eq!(un.g, 128);
  assert_eq!(un.b, 0);
}

/// Linear RGB data, `u16` per channel.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct r16g16b16_Unorm {
  pub r: u16,
  pub g: u16,
  pub b: u16,
}
impl r16g16b16_Unorm {
  pub const BLACK: Self = Self { r: 0, g: 0, b: 0 };
  pub const WHITE: Self = Self { r: u16::MAX, g: u16::MAX, b: u16::MAX };
}
impl From<r32g32b32_Sfloat> for r16g16b16_Unorm {
  #[inline]
  #[must_use]
  fn from(sf: r32g32b32_Sfloat) -> Self {
    r16g16b16_Unorm {
      r: (sf.r * (u16::MAX as f32)) as u16,
      g: (sf.g * (u16::MAX as f32)) as u16,
      b: (sf.b * (u16::MAX as f32)) as u16,
    }
  }
}

/// Linear RGB data, `f32` per channel.
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct r32g32b32_Sfloat {
  pub r: f32,
  pub g: f32,
  pub b: f32,
}
impl r32g32b32_Sfloat {
  pub const BLACK: Self = Self { r: 0.0, g: 0.0, b: 0.0 };
  pub const WHITE: Self = Self { r: 1.0, g: 1.0, b: 1.0 };
}
impl From<r8g8b8_Srgb> for r32g32b32_Sfloat {
  #[inline]
  #[must_use]
  fn from(r8g8b8_Srgb { r, g, b }: r8g8b8_Srgb) -> Self {
    Self {
      r: srgb_u8_to_linear_f32(r),
      g: srgb_u8_to_linear_f32(g),
      b: srgb_u8_to_linear_f32(b),
    }
  }
}
impl From<r8g8b8_Unorm> for r32g32b32_Sfloat {
  #[inline]
  #[must_use]
  fn from(r8g8b8_Unorm { r, g, b }: r8g8b8_Unorm) -> Self {
    Self {
      r: (r as f32) / (u8::MAX as f32),
      g: (g as f32) / (u8::MAX as f32),
      b: (b as f32) / (u8::MAX as f32),
    }
  }
}
