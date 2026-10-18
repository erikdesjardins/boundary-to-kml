use anyhow::Error;
use geo::{Point, Rect};
use image::{ImageFormat, ImageReader, Rgba};
use std::io::{BufRead, Seek};
use std::ops::Range;

const LINE_COLOR: Rgba<u8> = Rgba([104, 104, 104, 128]);

pub fn run<R: BufRead + Seek>(bounds: Rect, input: &mut R) -> Result<Vec<Point>, Error> {
    let image = ImageReader::with_format(input, ImageFormat::Png).decode()?;

    let image = match image {
        image::DynamicImage::ImageRgba8(image) => image,
        _ => {
            log::warn!("Unexpected image format--converting to RGBA8");
            image.into_rgba8()
        }
    };

    // Compute the center of mass of line colored pixels within the given bounding box.
    // Returns (x, y), each 0-1, representing the position in image space with (0, 0) at the top left, or none if no line segments were present
    let centre_of_mass_in_region =
        |x_range: Range<u32>, y_range: Range<u32>| -> Option<(f64, f64)> {
            assert!(x_range.start < x_range.end);
            assert!(y_range.start < y_range.end);

            let mut x_mass = 0;
            let mut y_mass = 0;
            let mut count = 0;

            for x in x_range.clone() {
                for y in y_range.clone() {
                    if *image.get_pixel(x, y) == LINE_COLOR {
                        x_mass += x;
                        y_mass += y;
                        count += 1u32;
                    }
                }
            }

            if count > 0 {
                Some((
                    f64::from(x_mass) / f64::from(count) / f64::from(image.width()),
                    f64::from(y_mass) / f64::from(count) / f64::from(image.height()),
                ))
            } else {
                None
            }
        };

    let lerp_bounds = |x_frac, y_frac| {
        assert!(x_frac >= 0.);
        assert!(x_frac <= 1.);
        assert!(y_frac >= 0.);
        assert!(y_frac <= 1.);

        Point::new(
            bounds.min().x + bounds.width() * x_frac,
            bounds.min().y + bounds.height() * y_frac,
        )
    };

    let mut points = Vec::new();

    // Divide into square zones, ~100 horizontally, and output a point in the center of mass within each zone.
    let step = image.width() / 200;
    let mut y = 0;
    while y < image.height() - 1 {
        let y_range = y..(y + step).clamp(0, image.height());
        y = y_range.end;

        let mut x = 0;
        while x < image.width() - 1 {
            let x_range = x..(x + step).clamp(0, image.width());
            x = x_range.end;

            if let Some((x_frac, y_frac)) = centre_of_mass_in_region(x_range, y_range.clone()) {
                // Need to invert y, since image is positive down and coords are positive up
                points.push(lerp_bounds(x_frac, 1. - y_frac));
            }
        }
    }

    Ok(points)
}
