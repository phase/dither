use super::dither::Dither;
use super::*;
use rand::prelude::*;
use std::path::PathBuf;
#[test]
fn test_save_and_load() {
    let img = test_image();
    let mut output = std::env::current_dir().unwrap();

    output.push("save_load_test.png");
    img.clone().save_png(&output).unwrap();

    assert_eq!(img, Img::read_png(&output).unwrap());
    std::fs::remove_file(output).unwrap();
}

fn test_image_path() -> PathBuf {
    let mut input = std::env::current_dir().unwrap();
    input.push("bliss.png");
    input
}
fn test_image() -> Img<RGB<u8>> {
    Img::read_png(test_image_path()).unwrap()
}
/// No Op ditherer; doesn't do anything;
struct NoOpDither;

impl Dither for NoOpDither {
    const DIV: f64 = 1.;
    const OFFSETS: &'static [(isize, isize, f64)] = &[];
}
#[test]
fn test_no_op() {
    fn no_op(p: RGB<f64>) -> (RGB<f64>, RGB<f64>) {
        (p, RGB::default())
    }
    let mut test_img_buf: Vec<RGB<f64>> = Vec::with_capacity(9);
    for y in 0..3 {
        for x in 0..3 {
            test_img_buf.push(RGB(0., f64::from(y), f64::from(x)));
        }
    }
    let img = Img {
        buf: test_img_buf,
        width: 3,
    };
    assert_eq!(img, NoOpDither::dither(img.clone(), no_op));
}

/*#[test]
fn test_main() {
    let output = "/Users/efron/rust/dither/grayscale.png";
    let opts = Opt {
        color: false,
        input: test_image_path(),
        output: PathBuf::from(output.to_owned()),
        ditherer: Ditherer::Stucki,
    };
    main(opts);
}
*/

#[test]
fn test_quantize() {
    let uniform = rand::distributions::Uniform::from((0.)..(255.));
    let mut rng = rand::thread_rng();
    let mut q = create_quantize_n_bits_func(1).unwrap();
    for _ in 0..20 {
        let n = uniform.sample(&mut rng);
        let (want_q, want_r) = quantize_1_bit(n);
        let (got_q, got_r) = q(n);
        if (got_q - want_q).abs() > std::f64::EPSILON
            || (got_r - want_r).abs() > 1. + std::f64::EPSILON
        {
            dbg!((n, want_q, want_r, got_q, got_r));
            panic!();
        }
    }
}

fn quantize_1_bit(x: f64) -> (f64, f64) {
    if x < 128. {
        (0., x)
    } else {
        (255., x - 255.)
    }
}
