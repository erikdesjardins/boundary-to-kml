use anyhow::Error;
use geo::Point;
use std::io::{BufWriter, Write};

// The offset (in degrees) from each detected point to the tips of each cross drawn at that point.
// Note that this isn't a consistent size, since the length of 1 degree of longitude differs at each latitude.
// TODO: scale this based on the coords of the point.
const CROSS_OFFSET: f64 = 0.0001; // ~11m

pub fn emit(
    points: &[Point],
    output: &mut BufWriter<impl Write>,
) -> Result<(), Error> {
    writeln!(output, r#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
    writeln!(output, r#"<kml xmlns="http://www.opengis.net/kml/2.2">"#)?;
    writeln!(output, "<Placemark>")?;
    writeln!(output, "<name>Test points</name>")?;
    writeln!(output, "<MultiGeometry>")?;
    for a in points {
        writeln!(output, "<LineString>")?;
        writeln!(output, "<coordinates>{},{} {},{}</coordinates>", a.x() - CROSS_OFFSET, a.y(), a.x() + CROSS_OFFSET, a.y())?;
        writeln!(output, "</LineString>")?;
        writeln!(output, "<LineString>")?;
        writeln!(output, "<coordinates>{},{} {},{}</coordinates>", a.x(), a.y() - CROSS_OFFSET, a.x(), a.y() + CROSS_OFFSET)?;
        writeln!(output, "</LineString>")?;
    }
    writeln!(output, "</MultiGeometry>")?;
    writeln!(output, "</Placemark>")?;
    writeln!(output, "</kml>")?;
    Ok(())
}
