pub mod palette;
mod utils;
#[cfg(feature = "wasm_bindgen")]
use wasm_bindgen::prelude::*;

use image::{
    imageops::{blur, resize, ColorMap},
    GenericImageView, Rgba, RgbaImage,
};
pub use palette::palettes::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// These options modify the algorithm(s) used by `convert`. You can specify
/// built-in pre-processing operations like resizing and quantization,
/// processing parameters like transparency tolerance and average kernel, and
/// post-processing operations like Gaussian blur.
#[cfg_attr(feature = "ffi", repr(C))]
#[cfg_attr(feature = "wasm_bindgen", wasm_bindgen)]
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
    pub resize: [u32; 2],
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
    pub quantize: u8,
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
            resize: [0, 0],
            quantize: 10,
            transparency_tolerance: 190,
        }
    }
}

// TODO: make generic over different image types
pub fn convert<P: ColorMap + Sync>(img: RgbaImage, opt: Options, palette: &P) -> RgbaImage {
    let img = if opt.resize != [0, 0] {
        // resize with nearest neigbor filtering
        resize_img(&img, opt.resize)
    } else {
        img // NOTE: will need to cast this to rgba when img is generic
    };

    // 1 <= q <= 10
    let img = if opt.quantize >= 1 && opt.quantize <= 10 {
        todo!("neuquant quantization pre-process")
    } else {
        img
    };

    let get_pixel_fn = if let [0, 0] = opt.avg {
        // just get the pixel at the coordinates, nothing fancy
        |x, y| img.get_pixel(x, y).clone()
    } else {
        todo!("get avg around pixel fn")
    };

    let img = replace_pixels(get_pixel_fn, opt.transparency_tolerance, palette);

    // gaussian blur
    let img = if opt.blur > 0. {
        blur(&img, opt.blur)
    } else {
        img
    };

    img
}

fn resize_img(img: &RgbaImage, [w, h]: [u32; 2]) -> RgbaImage {
    resize(img, w, h, image::imageops::FilterType::Nearest)
}

// TODO: need to change the signatures to take the image as a parameter rather
// than capturing it

#[cfg(not(feature = "rayon"))]
fn replace_pixels<P: ColorMap>(
    get_pixel_fn: impl Fn(u32, u32) -> Rgba<u8>,
    transparency_tolerance: u8,
    palette: &P,
) -> RgbaImage {
    todo!()
}

#[cfg(feature = "rayon")]
fn replace_pixels<P: ColorMap + Sync>(
    get_pixel_fn: impl Fn(u32, u32) -> Rgba<u8>,
    transparency_tolerance: u8,
    palette: &P,
) -> RgbaImage {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::{resize_img};
    use image::RgbaImage;

    #[test]
    fn test_resize_img() {
        let img = RgbaImage::from_pixel(64, 64, [255; 4].into());
        let img = resize_img(&img, [32, 32]);

        assert_eq!(img.dimensions(), (32, 32))
    }
}
