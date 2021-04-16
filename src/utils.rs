use image::{GenericImageView, Pixel};
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

/// euclidian distance between colors
pub fn delta<T: CheckedSub + Unsigned + Clone, P: Pixel<Subpixel = T>>(_a: &P, _b: &P) -> T {
    todo!()
}
