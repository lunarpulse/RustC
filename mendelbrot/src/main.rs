extern crate num;
extern crate image;
extern crate crossbeam;

use std::str::FromStr;
use num::Complex;
use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;
use std::io::{Result,Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 7{
        writeln!(std::io::stderr(),
                "Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT LIMIT_RADIUS", args[0]).unwrap();
        writeln!(std::io::stderr(),
                "Example: {} mandel.png 1920x1080 -1.20,0.35 -1,0.20 255 4.0", args[0]).unwrap();
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_pair(&args[3], ',').expect("Error parsing upperleft corner point");
    let lower_right = parse_pair(&args[4], ',').expect("error parsing lower right corner point");
    let limit: &u32 = &args[5].parse::<u32>().expect("error parsing limit "); //parsing the args to u32 or f64
    let radius: &f64 = &args[6].parse::<f64>().expect("error parsing radius");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    /*
    render(&mut pixels[..], bounds, upper_left, lower_right,limit, radius);
    */

    let threads = 8;
    let band_rows = bounds.1/ threads+1;
    {
        let bands: Vec<_> = pixels.chunks_mut(band_rows*bounds.0).collect();
        crossbeam::scope(|scope|{
            for(i, band) in bands.into_iter().enumerate(){
                let top = band_rows * i;
                let height = band.len()/bounds.0;
                let band_bounds = (bounds.0,height);
                let band_upper_left = pixel_to_point(bounds, (0,top), upper_left, lower_right);
                let band_lower_right = pixel_to_point(bounds, (bounds.0, top+height), upper_left, lower_right);
                scope.spawn(move||{
                    render(band, band_bounds,band_upper_left,band_lower_right, *limit, *radius);
                });
            }
        });
    }

    write_bitmap(&args[1], &pixels[..], bounds).expect("error writing PNG file");

}

/// Parse the string `s` as a coordinate pair, like `"400x600"` or `"1.0,0.5"`.
///
/// Specifically, `s` should have the form <left><sep><right>, where <sep> is
/// the character given by the `separator` argument, and <left> and <right> are both
/// strings that can be parsed by `T::from_str`.
///
/// If `s` has the proper form, return `Some<(x, y)>`. If it doesn't parse
/// correctly, return `None`.
fn parse_pair<T:FromStr>(s: &str, separator: char) -> Option<(T,T)> { //only one argument
    match s.find(separator){
        None => None,
        Some(index)=> {
            match (T::from_str(&s[..index]), T::from_str(&s[index+1..])) {
                (Ok(l), Ok(r))=> Some((l,r)),
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("",','),None);
    assert_eq!(parse_pair::<i32>("10",','),None);
    assert_eq!(parse_pair::<i32>("10,20",','),Some((10,20)));
    assert_eq!(parse_pair::<i32>("10,20xy",','),None);
    assert_eq!(parse_pair::<f64>("0.5x",'x'),None);
    assert_eq!(parse_pair::<f64>("0.5x1.5",'x'),Some((0.5,1.5)));
}

/// Return the point on the complex plane corresponding to a given pixel in the
/// bitmap.
///
/// `bounds` is a pair giving the width and height of the bitmap. `pixel` is a
/// pair indicating a particular pixel in that bitmap. The `upper_left` and
/// `lower_right` parameters are points on the complex plane designating the
/// area our bitmap covers.
fn pixel_to_point(bounds: (usize,usize), pixel: (usize,usize), upper_left:(f64, f64), lower_right:(f64,f64)) -> (f64, f64) {
    // It might be nicer to find the position of the *middle* of the pixel,
    // instead of its upper left corner, but this is easier to write tests for.
    let (width, height) = (lower_right.0 - upper_left.0, upper_left.1- lower_right.1);
    (upper_left.0 + pixel.0 as f64 *width /bounds.0 as f64,
    upper_left.1 - pixel.1 as f64 * height/ bounds.1 as f64)
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100,100), (25,75), (-1.0,1.0),(1.0, -1.0)),
    (-0.5, -0.5));
    assert_eq!(pixel_to_point((1000,1000), (250,750), (-1.0,1.0),(1.0, -1.0)),
    (-0.5, -0.5));
}

/// Try to determine whether the complex number `c` is in the Mandelbrot set.
///
/// A number `c` is in the set if, starting with zero, repeatedly squaring and
/// adding `c` never causes the number to leave the circle of radius 2 centered
/// on the origin; the number instead orbits near the origin forever. (If the
/// number does leave the circle, it eventually flies away to infinity.)
///
/// If after `limit` iterations our number has still not left the circle, return
/// `None`; this is as close as we come to knowing that `c` is in the set.
///
/// If the number does leave the circle before we give up, return `Some(i)`, where
/// `i` is the number of iterations it took.
fn escapes(c: Complex<f64>, limit: u32, radius: f64) -> Option<u32> { //giving the limit and radius arguments
    let mut z = Complex{re: 0.0, im: 0.0};
    for i in 0.. limit{ //using limit casting to u32 as f64 is not suitable for iteration
        z = z*z +c;
        if z.norm_sqr() > radius{ //radius used here
            return Some(i);
        }
    }
    return None;
}

/// Render a rectangle of the Mandelbrot set into a buffer of pixels.
///
/// The `bounds` argument gives the width and height of the buffer `pixels`,
/// which holds one grayscale pixel per byte. The `upper_left` and `lower_right`
/// arguments specify points on the complex plane corresponding to the upper
/// left and lower right corners of the pixel buffer.
fn render(pixels: &mut [u8], bounds: (usize, usize), upper_left: (f64, f64), lower_right: (f64, f64), limit: u32, radius:f64) { //adding limit and radius arguments
    assert!(pixels.len() == bounds.0 * bounds.1);

    for r in 0.. bounds.1{
        for c in 0.. bounds.0{
            let point = pixel_to_point(bounds, (c,r), upper_left, lower_right);
            pixels[r*bounds.0 + c] =
                match escapes(Complex{re: point.0, im: point.1}, limit, radius){ //using limit and radius in this
                    None=> 0,
                    Some(count) => limit as u8 - count as u8 //casting limit u32 to u8 cutting of the range expected
                };
        }
    }
}

/// Write the buffer `pixels`, whose dimensions are given by `bounds`, to the
/// file named `filename`.
fn write_bitmap(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<()> {
    let output = try!(File::create(filename));

    let encoder = PNGEncoder::new(output);
    try!(encoder.encode(&pixels[..],
                        bounds.0 as u32, bounds.1 as u32,
                        ColorType::Gray(8)));
    Ok(())
}
