#![cfg(feature = "clap")]

use color_eyre::eyre::{bail, Result};
#[cfg(feature = "rayon")]
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};

use clap::{crate_authors, crate_version, AppSettings, Clap};
use image::RgbaImage;
use image_go_nord::{convert, Options, NORD};

#[derive(Clap, Debug)]
#[clap(name = "Image Go Nord", version = crate_version!(), author = crate_authors!(","))]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opt {
    /// Input file or directory path.
    input: PathBuf,

    /// Blur by a given 𝛔 value between 0 and 1.
    #[clap(short, long, value_name = "SIGMA", default_value = "0.0")]
    blur: f32,
    /// Quantize the image with a given factor of samples, between 1 and 30. Any
    /// other value will disable quantization.
    #[clap(short, long, value_name = "SAMPLEFAC", default_value = "0")]
    quantize: i32,

    /// Output file or directory path.
    output: PathBuf,
}

fn main() -> Result<()> {
    let opts = Opt::parse();

    let options = Options {
        blur: opts.blur,
        quantize: opts.quantize,
        ..Default::default()
    };

    if opts.input.is_file() {
        convert_image(&opts.input, options)?.save(if opts.output.is_dir() {
            opts.output.join(opts.input.file_name().unwrap())
        } else {
            opts.output
        })?
    } else if opts.input.is_dir() && opts.output.is_dir() {
        // FIXME: this fails on the first error, which won't work if the user calls it
        // on a directory with anything besides just images.
        convert_images(&opts.input, options)?.try_for_each(|img| {
            img.and_then(|ConvertedImage { name, img }| Ok(img.save(opts.output.join(name))?))
        })?
    } else {
        bail!("INPUT and OUTPUT must both be either a file or directory")
    }
    Ok(())
}

fn convert_image(path: &Path, options: Options) -> Result<RgbaImage> {
    let img = image::open(path)?.to_rgba8();
    Ok(convert(&img, options, &NORD))
}

/// Holds name as well as image data so it can be saved with the same name
struct ConvertedImage {
    name: OsString,
    img: RgbaImage,
}

#[cfg(not(feature = "rayon"))]
fn convert_images(
    input_dir: &Path,
    options: Options,
) -> Result<impl Iterator<Item = Result<ConvertedImage>>> {
    Ok(input_dir.read_dir()?.map(move |e| {
        let e = e?;
        let name = e.file_name();
        let img = convert_image(&e.path(), options.clone())?;
        Ok(ConvertedImage { img, name })
    }))
}

#[cfg(feature = "rayon")]
fn convert_images(
    input_dir: &Path,
    options: Options,
) -> Result<impl ParallelIterator<Item = Result<ConvertedImage>>> {
    Ok(input_dir.read_dir()?.par_bridge().map(move |e| {
        let e = e?;
        let name = e.file_name();
        let img = convert_image(&e.path(), options.clone())?;
        Ok(ConvertedImage { img, name })
    }))
}
