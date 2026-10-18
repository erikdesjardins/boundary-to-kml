use crate::coord;
use anyhow::Error;
use geo::Point;
use std::io::{BufWriter, Write};

// The offset from each detected point to the tips of each cross drawn at that point.
// Thus, the cross will be twice this size from tip to tip.
// TODO: although EPSG:5321 is in meters (?), this actually results in a 9m x 11m cross.
const CROSS_OFFSET: f64 = 5.; // meters

pub fn emit(
    points: &[Point],
    conv: &coord::Converter,
    output: &mut BufWriter<impl Write>,
) -> Result<(), Error> {
    writeln!(output, r#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
    writeln!(output, r#"<kml xmlns="http://www.opengis.net/kml/2.2">"#)?;
    writeln!(output, "<Placemark>")?;
    writeln!(output, "<name>Test points</name>")?;
    writeln!(output, "<MultiGeometry>")?;
    for a in points {
        let a_plus = modify_in_m(*a, conv, |a| {
            Point::new(a.x() + CROSS_OFFSET, a.y() + CROSS_OFFSET)
        });
        let a_minus = modify_in_m(*a, conv, |a| {
            Point::new(a.x() - CROSS_OFFSET, a.y() - CROSS_OFFSET)
        });
        writeln!(output, "<LineString>")?;
        writeln!(
            output,
            "<coordinates>{},{} {},{}</coordinates>",
            a_minus.x(),
            a.y(),
            a_plus.x(),
            a.y()
        )?;
        writeln!(output, "</LineString>")?;
        writeln!(output, "<LineString>")?;
        writeln!(
            output,
            "<coordinates>{},{} {},{}</coordinates>",
            a.x(),
            a_minus.y(),
            a.x(),
            a_plus.y()
        )?;
        writeln!(output, "</LineString>")?;
    }
    writeln!(output, "</MultiGeometry>")?;
    writeln!(output, "</Placemark>")?;
    writeln!(output, "</kml>")?;
    Ok(())
}

fn modify_in_m(point: Point, conv: &coord::Converter, f: impl FnOnce(Point) -> Point) -> Point {
    conv.convert_ontario_to_wgs84(f(conv.convert_wgs84_to_ontario(point)))
}
