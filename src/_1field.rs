use super::*;

/// sRGB encoded Red data, `u8`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct r8_Srgb {
  pub r: u8,
}

/// Linear Red data, `u8`.
///
/// Note that 8 bits is too little precision to encode linear colors well, so
/// this format is *inherently* a not-great option for doing color work.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct r8_Unorm {
  pub r: u8,
}

/// Linear Red data, `u16`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct r16_Unorm {
  pub r: u16,
}

/// Linear R data, `f32`.
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct r32_Sfloat {
  pub r: f32,
}
impl From<r8_Srgb> for r32_Sfloat {
  #[inline]
  #[must_use]
  fn from(r8_Srgb { r }: r8_Srgb) -> Self {
    Self { r: srgb_u8_to_linear_f32(r) }
  }
}
impl From<r8_Unorm> for r32_Sfloat {
  #[inline]
  #[must_use]
  fn from(r8_Unorm { r }: r8_Unorm) -> Self {
    Self { r: (r as f32) / (u8::MAX as f32) }
  }
}
