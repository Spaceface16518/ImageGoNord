pub mod palette;
pub mod utils;

use color_quant::NeuQuant;
use image::{
    imageops::{blur, dither, resize, ColorMap, FilterType::Triangle},
    Rgba, RgbaImage,
};

pub use palette::palettes::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// These options modify the algorithm(s) used by `convert`. You can specify
/// built-in pre-processing operations like resizing and quantization,
/// processing parameters like transparency tolerance and average kernel, and
/// post-processing operations like Gaussian blur.
///
/// `Options` implements [`std::default::Default`], so you can use struct
/// builder syntax to easily make an `Options` struct that "overrides" the
/// default struct.
///
/// ```ignore
/// let options = Options {
///     resize: [1920, 1080],
///     blur: 0.4,
///     ..Default::default()
/// };
/// assert_eq!(options, Options {
///     resize: [1920, 1080],
///     quantize: 0,
///     avg: [0, 0],
///     transparency_tolerance: 0,
///     blur: 0.4,
/// })
/// ```
#[cfg_attr(feature = "ffi", repr(C))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Options {
    /// Resize image by a certain factor before performing other processing. The
    /// image will be resized using linear filtering back to the original
    /// dimensions before it is output. This will cause the algorithm to
    /// consider bigger "pixels" in the image, which will result in a
    /// lower quality conversion.
    #[deprecated]
    pub resize: u32,
    /// Quantize the image by the given balance. The value must be between 1 and
    /// 30. A value closer to 1 will be slower, but provide better
    /// quantization. The default value of 10 is a good balance between
    /// speed and quality.
    ///
    /// Passing any invalid value (like 0) disables quantization, which is the
    /// default behavior
    pub quantize: i32,
    /// Perform a Gaussian blur on the output image. This can help smooth
    /// gradients and remove unwanted artifacts.
    ///
    /// `0.0` means don't blur.
    pub blur: f32,
}

impl Default for Options {
    #[allow(deprecated)]
    fn default() -> Self {
        Options {
            blur: 0.,
            resize: 0,
            quantize: 0,
        }
    }
}

// TODO: make generic over different image types
#[allow(deprecated)]
pub fn convert(
    img: &RgbaImage,
    opt: Options,
    palette: &impl ColorMap<Color = Rgba<u8>>,
) -> RgbaImage {
    // resize the image to simulate averaging of pixels
    let (w, h) = img.dimensions(); // save width and height for later
    let mut img = if opt.resize > 1 {
        // sample using linear filtering
        resize(img, w - w / opt.resize, h - h / opt.resize, Triangle)
    } else {
        img.clone()
    };

    // dither image using neu-quant quantization
    if (1..=30).contains(&opt.quantize) {
        let ref q = NeuQuant::new(opt.quantize, 256, img.as_raw()); // train neural network
        dither(&mut img, q);
    }

    // re-color the image using the provided palette
    dither(&mut img, palette);

    // blur image
    let img = if opt.blur > 0. {
        blur(&img, opt.blur)
    } else {
        img
    };

    // if we resized earlier, restore original size
    if opt.resize > 1 {
        resize(&img, w, h, Triangle)
    } else {
        img
    }
}
