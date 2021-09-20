use image::{imageops::overlay, io::Reader, GenericImageView};
use image_go_nord::{convert, Options, NORD};
use std::{
    error::Error,
    io::{Cursor, Read},
    time::Instant,
};

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
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
    eprintln!(
        "Fetched random picture in {:.3}s",
        start.elapsed().as_secs_f32()
    );

    let (w, h) = pic.dimensions();
    let right_half = pic.view(w / 2, 0, w / 2, h).to_image();

    let start = Instant::now();
    let converted = convert(
        &right_half,
        Options {
            quantize: 5,
            blur: 0.2,
            ..Default::default()
        },
        &NORD,
    );
    eprintln!("Converted to nord in {:.3}s", start.elapsed().as_secs_f32());

    overlay(&mut pic, &converted, w / 2, 0);

    pic.save("nord.jpg")?;
    eprintln!("Saved to 'nord.jpg'");

    Ok(())
}
