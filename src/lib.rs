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
    /// Resize image to a new width and height, `[w, h]`, before performing
    /// other operations. Dimensions must be non-zero; passing a one or more
    /// zeros will cancel resampling. In other words, pass `[0, 0]` to
    /// disable resampling. This is the default behavior.
    ///
    /// This is an expensive operation, so if it needs to be done repeatedly to
    /// the same image, it is usually more efficient to resize the image in
    /// advance and skip resizing during the conversion. A good
    /// example situation in which this would be beneficial is a live conversion
    /// preview.
    pub resize: u32,
    /// Quantize the image by the given balance. The value must be between 1 and
    /// 30. A value closer to 1 will be slower, but provide better
    /// quantization. The default value of 10 is a good balance between
    /// speed and quality.
    ///
    /// While it can technically combined with the average algorithm, it is
    /// generally an *alternative*, so it should be passed in combination
    /// with `avg: [0, 0]`. However, this is merely a suggestion—it is
    /// completely possible to do both, but the results may not be
    /// aesthetically desirable. If it is passed with a non-zero `avg`,
    /// quantization will happen before taking the average (obviously).
    ///
    /// Passing any invalid value (like 0) disables quantization, which is the
    /// default behavior
    pub quantize: i32,
    /// Use the average value of a kernel surrounding each pixel rather than the
    /// pixel value itself.
    pub avg: [u32; 2],
    /// Filter out pixels below a certain alpha.
    pub transparency_tolerance: u8,
    /// Perform a Gaussian blur on the output image. This can help smooth
    /// gradients and remove unwanted artifacts.
    ///
    /// `0.0` means don't blur.
    pub blur: f32,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            blur: 0.,
            avg: [0, 0],
            resize: 0,
            quantize: 0,
            transparency_tolerance: 190,
        }
    }
}

// TODO: make generic over different image types
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

// TODO: need to change the signatures to take the image as a parameter rather
// than capturing it

//fn map_color(pix: &mut Rgba<u8>, palette: &impl ColorMap, quant: &impl
// ColorMap) {
////palette.map_color(pix);
//unimplemented!()
//}
//
//fn get_color(img: &RgbaImage, [x, y]: [u32; 2], avg: [u32; 2]) -> Rgba<u8> {
//todo!("get color from coordinate and kernel")
//}
//
//fn map_pixel(
//img: &RgbaImage,
//coord: [u32; 2],
//avg: [u32; 2],
//transparency_tolerance: u8,
//palette: &impl ColorMap,
//quant: &impl ColorMap,
//) -> Rgba<u8> {
//let pix = get_color(img, coord, avg);
//if pix[3] > transparency_tolerance {
//map_color(&mut pix, palette, quant);
//}
//pix
//}
//
//#[cfg(not(feature = "rayon"))]
//fn map_pixels(
//img: RgbaImage,
//avg: [u32; 2],
//transparency_tolerance: u8,
//palette: &impl ColorMap,
//quant: &impl ColorMap
//) -> RgbaImage {
//RgbaImage::from_fn(img.width(), img.height(), |x, y| {
//map_pixel(&img, [x, y], avg, transparency_tolerance, palette, quant)
//})
//}
//
//
//
