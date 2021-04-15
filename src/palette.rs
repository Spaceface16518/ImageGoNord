use image::Rgb;
use itertools::Itertools;

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
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DynamicPalette {
    colors: Vec<Rgb<u8>>,
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
pub struct StaticPalette<const N: usize> {
    colors: [Rgb<u8>; N],
}

impl<const N: usize> StaticPalette<N> {
    pub fn from_rgb(rgb: [[u8; 3]; N]) -> Self {
        let mut colors = [Rgb([0u8; 3]); N];

        let mut i = 0;
        while i < N {
            colors[i] = Rgb(rgb[i]);
            i += 1;
        }
        StaticPalette { colors }
    }

    pub fn from_rgb_hex(hex: [u32; N]) -> Self {
        let mut colors = [[0; 3]; N];

        let mut i = 0;
        while i < N {
            colors[i] = hex_to_rgb(hex[i]);
            i += 1
        }
        StaticPalette::from_rgb(colors)
    }
}

impl<const N: usize> Into<DynamicPalette> for StaticPalette<N> {
    /// Converts a static palette into a dynamic palette by cloning each color
    /// into a new colors vector for the new palette.
    fn into(self) -> DynamicPalette {
        let colors = self.colors.to_vec();
        DynamicPalette { colors }
    }
}

const fn hex_to_rgb(hex: u32) -> [u8; 3] {
    let color = hex & 0xFFFFFF;
    let r = color & 0xFF0000;
    let g = color & 0x00FF00;
    let b = color & 0x0000FF;
    [r as u8, g as u8, b as u8]
}

// TODO: impl ColorMap for {Static,Dynamic}Palette
// TODO: impl ColorMap for [{Static,DynamicPalette}]
