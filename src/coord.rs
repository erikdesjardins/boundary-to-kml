use geo::Point;
use proj::Proj;

pub struct Converter {
    ontario_to_wgs84: Proj,
    wgs84_to_ontario: Proj,
}

impl Converter {
    pub fn new() -> Self {
        let ontario = "EPSG:5321"; // NAD83(CSRS) / Teranet Ontario Lambert
        let wgs84 = "EPSG:4326"; // WGS84
        let ontario_to_wgs84 = Proj::new_known_crs(ontario, wgs84, None).unwrap();
        let wgs84_to_ontario = Proj::new_known_crs(wgs84, ontario, None).unwrap();
        Self {
            ontario_to_wgs84,
            wgs84_to_ontario,
        }
    }

    pub fn convert_ontario_to_wgs84(&self, point: Point) -> Point {
        self.ontario_to_wgs84.convert(point).unwrap()
    }

    pub fn convert_wgs84_to_ontario(&self, point: Point) -> Point {
        self.wgs84_to_ontario.convert(point).unwrap()
    }
}
