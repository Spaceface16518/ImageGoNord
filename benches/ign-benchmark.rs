use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use image_go_nord::{convert, Options, AURORA, FROST, NORD, POLAR_NIGHT, SNOW_STORM};
use std::path::Path;

const IMAGES: &[&str] = &["tinycross.png", "test-profile.png", "cat.png"];
const DEFAULT_IMAGE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/images/test-profile.png");

const BLUR_SIGMAS: &[f32] = &[0.0, 0.2, 0.33, 0.66];
const QUANT_LEVELS: &[i32] = &[0, 30, 10, 1];

/// unrolls a loop to simulate looping over an array of static palettes
macro_rules! unroll {
    (for $palette_var:ident in [$($palette_name:ident),*] { $body:stmt; }) => {
        $(
            {
                const PALETTE_NAME: &str = stringify!($palette_name);
                let ref $palette_var = $palette_name; $body
            };
        )*
    };
}

macro_rules! def_benches {
    ($(fn $fn_name:ident($img_var_name:ident : &RgbaImage) { $bench_body:expr } )* ) => {
        $(
        pub fn $fn_name(c: &mut Criterion) {
            for image in IMAGES {
                let ref $img_var_name = image::open(Path::new(env!("CARGO_MANIFEST_DIR")).join("images").join(image)).unwrap();
                unroll! {
                    for p in [NORD, AURORA, FROST, POLAR_NIGHT, SNOW_STORM] {
                        c.bench_function(&format!("{} {} ", stringify!($fn_name), PALETTE_NAME), |b| {
                            b.iter(|| {$bench_body})
                        });
                    }
                }
            }
        }
        )*
    };
}

pub fn bench_palette(c: &mut Criterion) {
    let mut group = c.benchmark_group("palette");

    let image = image::open(DEFAULT_IMAGE).unwrap().to_rgba8();
    unroll! {
        for p in [NORD, AURORA, FROST, POLAR_NIGHT, SNOW_STORM] {
            group.bench_function(BenchmarkId::new("palette", PALETTE_NAME), |b| b.iter(|| convert(&image, Default::default(), p)));
        }
    }
}

pub fn bench_quant(c: &mut Criterion) {
    let mut group = c.benchmark_group("quantization");

    let image = image::open(DEFAULT_IMAGE).unwrap().to_rgba8();
    for q in 1..=30 {
        let options = Options {
            quantize: q,
            ..Default::default()
        };
        group.bench_function(BenchmarkId::new("quantize", q), |b| {
            b.iter(|| convert(&image, options.clone(), &NORD))
        });
    }
}

pub fn bench_resize(c: &mut Criterion) {
    let mut group = c.benchmark_group("resize");

    let image = image::open(DEFAULT_IMAGE).unwrap().to_rgba8();
    for r in [2, 3, 4, 5, 6, 7, 8, 9, 10, 15, 20, 30, 40, 50, 75, 100, 200] {
        let options = Options {
            resize: r,
            ..Default::default()
        };
        group.bench_function(BenchmarkId::new("resize", r), |b| {
            b.iter(|| convert(&image, options.clone(), &NORD))
        });
    }
}

pub fn bench_blur(c: &mut Criterion) {
    let mut group = c.benchmark_group("blur");

    let image = image::open(DEFAULT_IMAGE).unwrap().to_rgba8();
    for b in [0.1, 0.15, 0.2, 0.33, 0.4, 0.5, 0.66, 0.8, 0.9] {
        let options = Options {
            blur: b,
            ..Default::default()
        };
        group.bench_function(BenchmarkId::new("blur", b), |b| {
            b.iter(|| convert(&image, options.clone(), &NORD))
        });
    }
}

pub fn bench_default(c: &mut Criterion) {
    let image = image::open(DEFAULT_IMAGE).unwrap().to_rgba8();
    let options = Options::default();
    c.bench_function("default", |b| {
        b.iter(|| convert(&image, options.clone(), &NORD))
    });
}

criterion_group!(
    options,
    bench_default,
    bench_quant,
    bench_resize,
    bench_blur
);
criterion_group!(inputs, bench_palette);
criterion_main!(options, options, inputs);
