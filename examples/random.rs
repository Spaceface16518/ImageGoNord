use image::{imageops::overlay, io::Reader, GenericImageView, RgbaImage};
use image_go_nord::{convert, Options, AURORA};
use std::{
    error::Error,
    io::{Cursor, Read},
};
fn main() -> Result<(), Box<dyn Error>> {
    let mut pic = {
        let req = ureq::get("https://source.unsplash.com/random").call()?;
        let mut bytes = if let Some(len) = req.header("Content-Length").and_then(|s| s.parse().ok())
        {
            Vec::with_capacity(len)
        } else {
            Vec::new()
        };
        req.into_reader().take(20_000_000).read_to_end(&mut bytes)?;
        Reader::new(Cursor::new(bytes))
            .with_guessed_format()?
            .decode()?
            .to_rgba8()
    };

    let (w, h) = pic.dimensions();
    let right_half = pic.view(w / 2, 0, w / 2, h).to_image();

    let converted = convert(right_half, Default::default(), &AURORA);

    overlay(&mut pic, &converted, w / 2, 0);

    pic.save("nord.jpg")?;

    Ok(())
}
