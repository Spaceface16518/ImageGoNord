pub mod palette;
mod utils;
#[cfg(feature = "wasm_bindgen")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// These options modify the algorithm(s) used by `convert`. You can specify built-in
/// pre-processing operations like resizing and quantization, processing parameters like
/// transparency tolerance and average kernel, and post-processing operations like Gaussian blur.
#[cfg_attr(feature = "ffi", repr(C))]
#[cfg_attr(feature = "wasm_bindgen", wasm_bindgen)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Options {
    /// Resize image before performing other operations. Dimensions must be non-zero; passing a
    /// one or more zeros will cancel resampling. In other words, pass `[0, 0]` to disable
    /// resampling. This is the default behavior.
    pub resample: [usize; 2],
    /// Use the average value of a kernel surrounding each pixel rather than the pixel value
    /// itself.
    pub avg: [usize; 2],
    /// Quantize the image by the given balance. The value must be between 1 and 30. A value closer
    /// to 1 will be slower, but provide better quantization. The default value of 10 is a good
    /// balance between speed and quality.
    pub quantize: u8,
    /// Filter out pixels below a certain alpha.
    pub transparency_tolerance: u8,
    /// Perform a Gaussian blur on the output image. This can help smooth gradients and remove
    /// unwanted artifacts.
    pub blur: f32,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            blur: 0.,
            avg: [0, 0],
            resample: [0, 0],
            quantize: 10,
            transparency_tolerance: 190,
        }
    }
}

pub fn convert<I>(img: I, opt: Options, palette: ()) -> () {
    todo!()
}
