use super::Dither;
use super::*;
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
    const DIV: i16 = 1;
    const OFFSETS: &'static [(isize, isize, i16)] = &[];
}
#[test]
fn test_no_op() {
    fn no_op(p: RGB<i16>) -> (RGB<i16>, RGB<i16>) {
        (p, RGB::default())
    }
    let mut test_img_buf: Vec<RGB<i16>> = Vec::with_capacity(9);
    for y in 0..3 {
        for x in 0..3 {
            test_img_buf.push(RGB(0, y, x));
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
