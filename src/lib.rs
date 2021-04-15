pub mod palette;
mod utils;
#[cfg(feature = "wasm_bindgen")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg_attr(feature = "ffi", repr(C))]
#[cfg_attr(feature = "wasm_bindgen", wasm_bindgen)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Options {
    pub blur: f32,
    pub avg: [usize; 2],
    pub transparency_tolerance: u8,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            blur: 0.,
            avg: [0, 0],
            transparency_tolerance: 190,
        }
    }
}

pub fn convert<I>(img: I, opt: Options, palette: ()) -> () {
    todo!()
}
