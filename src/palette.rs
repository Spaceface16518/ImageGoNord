use crate::utils::delta;
use image::{imageops::ColorMap, Pixel, Rgb, Rgba};
use itertools::Itertools;
use smallvec::SmallVec;

pub(crate) type Entry = [u8; 3];

/// Palette that can be constructed at runtime. Used to load custom or filtered
/// palettes that can be changed between subsequent conversions.
/// # Palette strings
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
pub struct DynamicPalette {
    // 5 because the largest of the Nord palettes has five colors
    colors: SmallVec<[Entry; 16]>,
}

impl DynamicPalette {
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
pub struct StaticPalette<const N: usize> {
    colors: [Entry; N],
}

impl<const N: usize> StaticPalette<N> {
    pub const fn from_rgb_hex(hex: [u32; N]) -> Self {
        let mut colors = [[0; 3]; N];

        let mut i = 0;
        while i < N {
            colors[i] = hex_to_rgb(hex[i]);
            i += 1
        }
        StaticPalette { colors }
    }

    pub const fn from_colors(colors: [Entry; N]) -> Self {
        StaticPalette { colors }
    }
}

impl<const N: usize> Into<DynamicPalette> for StaticPalette<N> {
    /// Converts a static palette into a dynamic palette by cloning each color
    /// into a new colors vector for the new palette.
    fn into(self) -> DynamicPalette {
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

impl<const N: usize> ColorMap for StaticPalette<N> {
    type Color = Rgba<u8>;

    fn index_of(&self, color: &Self::Color) -> usize {
        // calculate the delta between the given color and a color in the palette
        self.colors
            .iter()
            .map(|c: &Entry| (delta(color.channels(), c.as_ref())))
            .position_min()
            .unwrap()
    }

    fn map_color(&self, color: &mut Self::Color) {
        // find closest matching color in palette
        let i = self.index_of(color);
        let c = self.colors[i];
        // don't overwrite the alpha channel
        color.channels_mut()[..3].copy_from_slice(&c)
    }

    fn has_lookup(&self) -> bool {
        true
    }

    fn lookup(&self, index: usize) -> Option<Self::Color> {
        self.colors.get(index).map(|&entry| Rgb(entry).to_rgba())
    }
}
impl ColorMap for DynamicPalette {
    type Color = Rgba<u8>;

    fn index_of(&self, color: &Self::Color) -> usize {
        // calculate the delta between the given color and a color in the palette
        self.colors
            .iter()
            .map(|c: &Entry| dbg!(delta(color.channels(), c.as_ref())))
            .position_min()
            .unwrap()
    }

    fn map_color(&self, color: &mut Self::Color) {
        // find closest matching color in palette
        let i = self.index_of(color);
        let c = self.colors[i];
        // don't overwrite the alpha channel
        color.channels_mut()[..3].copy_from_slice(&c)
    }

    fn has_lookup(&self) -> bool {
        true
    }

    fn lookup(&self, index: usize) -> Option<Self::Color> {
        self.colors.get(index).map(|&entry| Rgb(entry).to_rgba())
    }
}

pub mod palettes {
    use super::StaticPalette;
    pub static AURORA: StaticPalette<5> =
        StaticPalette::from_rgb_hex([0xBF616A, 0x08770, 0xBCB8B, 0x3BE8C, 0x48EAD]);

    pub static FROST: StaticPalette<4> =
        StaticPalette::from_rgb_hex([0x8FBCBB, 0x88C0D0, 0x81A1C1, 0x5E81AC]);
    pub static POLAR_NIGHT: StaticPalette<4> =
        StaticPalette::from_rgb_hex([0x2E3440, 0x3B4252, 0x434C5E, 0x4C566A]);
    pub static SNOW_STORM: StaticPalette<3> =
        StaticPalette::from_rgb_hex([0xD8DEE9, 0xE5E9F0, 0xECEFF4]);

    // TODO: convert this to regular static once enough const generics functionality
    // has stablized
    pub static NORD: StaticPalette<16> = StaticPalette::from_colors([
        AURORA.colors[0],
        AURORA.colors[1],
        AURORA.colors[2],
        AURORA.colors[3],
        AURORA.colors[4],
        FROST.colors[0],
        FROST.colors[1],
        FROST.colors[2],
        FROST.colors[3],
        POLAR_NIGHT.colors[0],
        POLAR_NIGHT.colors[1],
        POLAR_NIGHT.colors[2],
        POLAR_NIGHT.colors[3],
        SNOW_STORM.colors[0],
        SNOW_STORM.colors[1],
        SNOW_STORM.colors[2],
    ]);
}
