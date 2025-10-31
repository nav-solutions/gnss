//! SBAS (geo service) selector helper.
use crate::prelude::Constellation;

use geo::{Contains, LineString, Point, Polygon};
use wkt::{Geometry, Wkt, WktFloat};

use std::{iter::FromIterator, str::FromStr};

fn wkt_line_string_to_geo<T>(line_string: &wkt::types::LineString<T>) -> LineString<T>
where
    T: WktFloat + Default + FromStr,
{
    LineString::from_iter(line_string.0.iter().map(|coord| (coord.x, coord.y)))
}

fn line_string<T>(name: &str) -> LineString<T>
where
    T: WktFloat + Default + FromStr,
{
    let mut res = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    res.push("data");
    res.push(name);
    let content = std::fs::read_to_string(res).unwrap();
    let wkt = Wkt::from_str(&content).unwrap();
    match wkt.item {
        Geometry::LineString(line) => wkt_line_string_to_geo(&line),
        _ => unreachable!(),
    }
}

fn load_database() -> Vec<(Constellation, geo::Polygon)> {
    let mut db: Vec<(Constellation, geo::Polygon)> = Vec::new();
    let db_path = env!("CARGO_MANIFEST_DIR").to_owned() + "/data/";
    let db_path = std::path::PathBuf::from(db_path);
    for entry in std::fs::read_dir(db_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let fullpath = &path.to_str().unwrap();
        let extension = path.extension().unwrap().to_str().unwrap();
        let name = path.file_stem().unwrap().to_str().unwrap();
        if extension.eq("wkt") {
            let poly = geo::Polygon::<f64>::new(
                line_string(fullpath), // exterior boundaries
                vec![],
            ); // dont care about interior
            if let Ok(sbas) = Constellation::from_str(&name.to_uppercase()) {
                db.push((sbas, poly))
            }
        }
    }
    db
}

/// Helps select a SBAS (geostationary augmentation service) from user coordinates.
/// ```
/// use gnss_rs::{
///     prelude::*,
///     sbas_selector,
/// };
///
/// let paris = (48.808378, 2.382682); // lat, lon [ddeg]
/// let sbas = sbas_selector(paris.0, paris.1);
/// assert_eq!(sbas, Some(Constellation::EGNOS));
///
/// let antartica = (-77.490631,  91.435181); // lat, lon [ddeg]
/// let sbas = sbas_selector(antartica.0, antartica.1);
/// assert_eq!(sbas.is_none(), true);
///```
pub fn sbas_selector(point: Point) -> Option<Constellation> {
    let db = load_database();
    for (sbas, area) in db {
        if area.contains(&point) {
            return Some(sbas.clone());
        }
    }
    None
}

#[cfg(test)]
mod test {
    use crate::{prelude::Constellation, sbas_selector};
    use geo::Point;

    #[test]
    fn test_sbas_helper() {
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
