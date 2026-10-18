use geo::Point;
use proj::Proj;

pub struct Converter {
    ontario_to_wgs84: Proj,
}

impl Converter {
    pub fn new() -> Self {
        let from = "EPSG:5321"; // NAD83(CSRS) / Teranet Ontario Lambert
        let to = "EPSG:4326"; // WGS84
        let ontario_to_wgs84 = Proj::new_known_crs(&from, &to, None).unwrap();
        Self { ontario_to_wgs84 }
    }

    pub fn convert_ontario_to_wgs84(&self, point: Point) -> Point {
        self.ontario_to_wgs84.convert(point).unwrap()
    }
}
