use crate::utils::delta;
use image::{imageops::ColorMap, Pixel, Rgb};
use itertools::Itertools;
use smallvec::SmallVec;

/// Palette that can be constructed at runtime. Used to load custom or filtered
/// palettes that can be changed between subsequent conversions.
///
/// # Palette strings
///
/// A dynamic palette can be created by parsing a *palette string*. A palette
/// string is just string of hexadecimal numbers separated by newlines.
/// Representations can be uppercase or lowercase. Each line is parsed
/// indivdually, so representations may be mixed.  Lines not starting with a '#'
/// will be skipped. After a '#', only the subsequent word will be parsed, so
/// you can put comments after the color (including more numbers) provided there
/// is a space after the color.
///
/// ```
/// # use image_go_nord::palette::{StaticPalette, DynamicPalette};
/// // a valid (but messy) palette string
/// let palette_string = r#"
///     ========== Nord Frost Palette ==========
///     #8FBCBB is called nord7
///     #88c0d0 (nord8)
///     #81A1C1
///     skip this line
///     #5e81ac -- an informative comment about nord9?
/// "#;
/// let palette = DynamicPalette::from_palette_str(palette_string).unwrap();
///
/// let frost = StaticPalette::from_rgb_hex([
///     0x8FBCBB,
///     0x88C0D0,
///     0x81A1C1,
///     0x5E81AC
/// ]);
/// assert_eq!(palette, frost.into())
/// ```
///
/// Only works on byte-size subpixel images
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DynamicPalette<P: Pixel<Subpixel = u8>> {
    // 5 because the largest of the Nord palettes has five colors
    colors: SmallVec<[P; 5]>,
}

impl DynamicPalette<Rgb<u8>> {
    pub fn from_palette_str(s: &str) -> Result<Self, std::num::ParseIntError> {
        let colors = s
            .lines()
            // filter out comments and comment lines
            .filter_map(|s| {
                s.trim_start()
                    .strip_prefix('#')
                    .and_then(|s| s.split_whitespace().nth(0))
            })
            // try to parse hex string
            .map(|s| u32::from_str_radix(s.trim(), 16))
            // convert hex code to rgb triple
            .map_ok(hex_to_rgb)
            .map_ok(|rgb| Rgb(rgb))
            // fail on the first parse error
            .collect::<Result<_, _>>()?;
        Ok(DynamicPalette { colors })
    }
}

/// Palette that can be constructed at compile time. Used to embed built-in
/// palettes in the library.
///
/// Use either of the `from_` funcitons to construct this
/// palette. For example, using `from_rgb_hex`.
///
/// ```
/// # use image_go_nord::palette::StaticPalette;
/// let frost = StaticPalette::from_rgb_hex([
///     0x8FBCBB,
///     0x88C0D0,
///     0x81A1C1,
///     0x5E81AC
/// ]);
/// ```
///
/// Only supports Rgb<u8> pixels at the moment.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct StaticPalette<P: Pixel<Subpixel = u8>, const N: usize> {
    colors: [P; N],
}

impl<const N: usize> StaticPalette<Rgb<u8>, N> {
    pub const fn from_rgb_hex(hex: [u32; N]) -> Self {
        let mut colors = [Rgb([0; 3]); N];

        let mut i = 0;
        while i < N {
            colors[i] = Rgb(hex_to_rgb(hex[i]));
            i += 1
        }
        StaticPalette { colors }
    }
}

impl<P: Pixel<Subpixel = u8>, const N: usize> Into<DynamicPalette<P>> for StaticPalette<P, N> {
    /// Converts a static palette into a dynamic palette by cloning each color
    /// into a new colors vector for the new palette.
    fn into(self) -> DynamicPalette<P> {
        let colors = self.colors.as_ref().into();
        DynamicPalette { colors }
    }
}

const fn hex_to_rgb(hex: u32) -> [u8; 3] {
    let r = (hex >> 16) & 0xFF;
    let g = (hex >> 8) & 0xFF;
    let b = (hex >> 0) & 0xFF;
    [r as u8, g as u8, b as u8]
}

impl<P: Pixel<Subpixel = u8>, const N: usize> ColorMap for StaticPalette<P, N> {
    type Color = P;

    fn index_of(&self, color: &Self::Color) -> usize {
        let d = |b| move |a| delta(a, b);
        self.colors.iter().map(d(color)).position_min().unwrap()
    }

    fn map_color(&self, color: &mut Self::Color) {
        let i = self.index_of(color);
        *color = self.colors[i];
    }

    fn has_lookup(&self) -> bool {
        true
    }

    fn lookup(&self, index: usize) -> Option<Self::Color> {
        self.colors.get(index).cloned()
    }
}
impl<P: Pixel<Subpixel = u8>> ColorMap for DynamicPalette<P> {
    type Color = P;

    fn index_of(&self, color: &Self::Color) -> usize {
        let d = |b| move |a| delta(a, b);
        self.colors.iter().map(d(color)).position_min().unwrap()
    }

    fn map_color(&self, color: &mut Self::Color) {
        let i = self.index_of(color);
        *color = self.colors[i];
    }

    fn has_lookup(&self) -> bool {
        true
    }

    fn lookup(&self, index: usize) -> Option<Self::Color> {
        self.colors.get(index).cloned()
    }
}

pub mod palettes {
    use super::StaticPalette;
    use image::Rgb;
    #[cfg_attr(feature = "ffi", no_mangle)]
    pub static AURORA: StaticPalette<Rgb<u8>, 5> =
        StaticPalette::from_rgb_hex([0xBF616A, 0x08770, 0xBCB8B, 0x3BE8C, 0x48EAD]);

    #[cfg_attr(feature = "ffi", no_mangle)]
    pub static FROST: StaticPalette<Rgb<u8>, 4> =
        StaticPalette::from_rgb_hex([0x8FBCBB, 0x88C0D0, 0x81A1C1, 0x5E81AC]);
    #[cfg_attr(feature = "ffi", no_mangle)]
    pub static POLAR_NIGHT: StaticPalette<Rgb<u8>, 4> =
        StaticPalette::from_rgb_hex([0x2E3440, 0x3B4252, 0x434C5E, 0x4C566A]);
    #[cfg_attr(feature = "ffi", no_mangle)]
    pub static SNOW_STORM: StaticPalette<Rgb<u8>, 3> =
        StaticPalette::from_rgb_hex([0xD8DEE9, 0xE5E9F0, 0xECEFF4]);
}
