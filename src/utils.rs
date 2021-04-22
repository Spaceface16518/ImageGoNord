use image::{GenericImageView, Pixel, Rgba};
use num_traits::{CheckedSub, Unsigned, Zero};

pub fn avg_color<I: GenericImageView>(_pixels: I) -> I::Pixel
where
    <I::Pixel as Pixel>::Subpixel: Zero + Clone,
{
    //pixels.pixels().map(|(_, _, pix)|
    // pix).fold(I::Pixel::from_slice([Zero::zero();
    // I::Pixel::CHANNEL_COUNT].as_ref()), |sum)
    todo!()
}

/// euclidian distance between colors, regulated by alpha component
///
/// NOTE: this is currently the **distance squared**!!!
///
/// TODO: regulate by alpha component for accuracy
pub fn delta<P: Pixel<Subpixel = u8>>(a: &P, b: &P) -> u8 {
    a.channels()[..3]
        .iter()
        .zip(b.channels()[..3].iter())
        .map(|(a_c, b_c)| a_c.saturating_sub(*b_c).saturating_pow(2))
        .fold(0, u8::saturating_add)
}
