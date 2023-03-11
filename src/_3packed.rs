/// RGB 5-bit per channel (bitpacked) linear(?) color data.
///
/// Bits are filled low to high. The highest bit is ignored:
/// * `0bI_BBBBB_GGGGG_AAAAA`
///
/// This is the native color format of the GBA.
#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct GbaColor(u16);
impl GbaColor {
  pub const BLACK: Self = Self(0b0_00000_00000_00000);
  pub const RED: Self = Self(0b0_00000_00000_11111);
  pub const GREEN: Self = Self(0b0_00000_11111_00000);
  pub const YELLOW: Self = Self(0b0_00000_11111_11111);
  pub const BLUE: Self = Self(0b0_11111_00000_00000);
  pub const MAGENTA: Self = Self(0b0_11111_00000_11111);
  pub const CYAN: Self = Self(0b0_11111_11111_00000);
  pub const WHITE: Self = Self(0b0_11111_11111_11111);

  #[inline]
  #[must_use]
  pub const fn new() -> Self {
    Self(0)
  }

  #[inline]
  #[must_use]
  pub const fn red(self) -> u16 {
    bitfrob::u16_get_value(0, 4, self.0)
  }
  #[inline]
  #[must_use]
  pub const fn green(self) -> u16 {
    bitfrob::u16_get_value(5, 9, self.0)
  }
  #[inline]
  #[must_use]
  pub const fn blue(self) -> u16 {
    bitfrob::u16_get_value(10, 14, self.0)
  }

  #[inline]
  #[must_use]
  pub const fn with_red(self, r: u16) -> Self {
    Self(bitfrob::u16_with_value(0, 4, self.0, r))
  }
  #[inline]
  #[must_use]
  pub const fn with_green(self, g: u16) -> Self {
    Self(bitfrob::u16_with_value(5, 9, self.0, g))
  }
  #[inline]
  #[must_use]
  pub const fn with_blue(self, b: u16) -> Self {
    Self(bitfrob::u16_with_value(10, 14, self.0, b))
  }
}
impl core::fmt::Debug for GbaColor {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut x = f.debug_struct("GbaColor");
    x.field("red", &self.red());
    x.field("green", &self.green());
    x.field("blue", &self.blue());
    x.finish()
  }
}
