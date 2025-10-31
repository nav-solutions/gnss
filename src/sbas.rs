//! SBAS (geo service) selector helper.
use bincode::{config, Decode};

use crate::prelude::Constellation;
use once_cell::sync::Lazy;
use std::str::FromStr;

use geo::{Contains, LineString, Point, Polygon};

#[derive(Decode)]
struct SbasPolygon {
    name: String,
    coordinates: Vec<(f64, f64)>,
}

#[derive(Default, Decode)]
struct SbasMap {
    polygons: Vec<SbasPolygon>,
}

static SBAS_POLYGONS: Lazy<SbasMap> = Lazy::new(|| {
    let config = config::standard();

    let bytes = include_bytes!(concat!(env!("OUT_DIR"), "/sbas_polygons.bin"));

    bincode::decode_from_slice(bytes, config)
        .unwrap_or_else(|e| {
            panic!("corrupt SBAS polygons database: {}", e);
        })
        .0
});

/// Helps select a SBAS (geostationary augmentation service) from user coordinates.
/// ```
/// use geo::Point;
/// use gnss_rs::{
///     prelude::*,
///     sbas_selector,
/// };
///
/// let paris = Point::new(48.808378, 2.382682);
/// assert_eq!(sbas_selector(paris), Some(Constellation::EGNOS));
///
/// let antarctica = Point::new(-77.490631,  91.435181);
/// assert_eq!(sbas_selector(antarctica), None);
///```
pub fn sbas_selector(point: Point) -> Option<Constellation> {
    for entry in SBAS_POLYGONS.polygons.iter() {
        let polygon = Polygon::new(LineString::from(entry.coordinates.clone()), vec![]);
        if polygon.contains(&point) {
            if let Ok(constellation) = Constellation::from_str(&entry.name) {
                // errors will not happen here,
                // because every single entrie is validated in CI
                return Some(constellation);
            }
        }
    }
    None
}

#[cfg(test)]
mod test {
    use crate::{prelude::Constellation, sbas_selector};
    use geo::Point;

    #[test]
    fn test_sbas_selector() {
        for (lat_ddeg, long_ddeg, expected) in [
            (48.808378, 2.38268, Some(Constellation::EGNOS)),
            (33.981431, -118.193601, Some(Constellation::WAAS)),
            (10.714217, 17.087263, Some(Constellation::ASBAS)),
            (19.314290, 76.798953, Some(Constellation::GAGAN)),
            (-27.579847, 131.334992, Some(Constellation::SPAN)),
            (-45.113525, 169.864842, Some(Constellation::SPAN)),
            (34.462967, 98.172480, Some(Constellation::BDSBAS)),
            (37.067846, 128.34, Some(Constellation::KASS)),
            (36.081095, 138.274859, Some(Constellation::MSAS)),
            (60.004390, 89.090326, Some(Constellation::SDCM)),
            (-32.473320, 21.112770, None),  // south-africa
            (-23.216639, -63.170983, None), // argentina
            (-77.490631, 91.435181, None),  // antarctica
            (-29.349172, 72.773447, None),  // south indian ocean
        ] {
            assert_eq!(
                sbas_selector(Point::new(lat_ddeg, long_ddeg)),
                expected,
                "invalid results for coordinates lat={}° long={}°",
                lat_ddeg,
                long_ddeg
            );
        }
    }
}
