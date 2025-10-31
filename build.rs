use std::{env, fs::File, io::Write, path::Path};

use serde::Deserialize;

/*
 * We use an intermediate struct
 * and "serde" to allow not to describe the launch
 * day or month for example
 */
#[derive(Deserialize)]
struct SBASDBEntry<'a> {
    /// Constellation name (must be valid)
    pub constellation: &'a str,

    /// PRN number to match
    pub prn: u16,

    /// Readable name
    pub name: &'a str,

    /// Launch Datetime
    pub launch: &'a str,
}

fn build_sbas_helper() {
    let outdir = env::var("OUT_DIR").unwrap();
    let path = Path::new(&outdir).join("sbas.rs");

    let mut fd = File::create(path).unwrap_or_else(|e| {
        panic!("Failed to initiate SBAS helpers: {}", e);
    });

    // read descriptor: parse and dump into a static array
    let db_content = std::fs::read_to_string("data/sbas.json").unwrap();

    let sbas_db: Vec<SBASDBEntry> = serde_json::from_str(&db_content).unwrap();

    let content = "use lazy_static::lazy_static;

#[derive(Debug)]
pub struct SBASHelper<'a> {
    constellation: &'a str,
    prn: u16,
    name: &'a str,
    launch: &'a str,
}

lazy_static! {
    static ref SBAS_VEHICLES: Vec<SBASHelper<'static>> = vec![
\n";

    fd.write_all(content.as_bytes()).unwrap();

    for e in sbas_db {
        fd.write_all(
            format!(
                "SBASHelper {{
                constellation: \"{}\",
                prn: {},
                name: \"{}\",
                launch: \"{}\",
            }},",
                e.constellation, e.prn, e.name, e.launch,
            )
            .as_bytes(),
        )
        .unwrap()
    }

    fd.write_all("    ];".as_bytes()).unwrap();
    fd.write_all("}\n".as_bytes()).unwrap();
}

#[cfg(feature = "sbas")]
use geojson::{Feature, GeoJson, Value};

#[cfg(feature = "sbas")]
use bincode::{config, Encode};

#[derive(Encode)]
struct SbasPolygon {
    name: String,
    coordinates: Vec<(f64, f64)>,
}

impl SbasPolygon {
    pub fn from_feature(feature: &Feature) -> Self {
        Self {
            name: {
                feature
                    .properties
                    .as_ref()
                    .unwrap_or_else(|| {
                        panic!("invalid geojson: properties not defined in {:?}", feature);
                    })
                    .get("name")
                    .unwrap_or_else(|| {
                        panic!(
                            "invalid geojson: name property not defined in {:?}",
                            feature
                        );
                    })
                    .to_string()
            },
            coordinates: {
                let mut coordinates = Vec::new();

                let geometry = feature.geometry.as_ref().unwrap_or_else(|| {
                    panic!("invalid geojson: geometry not defined in {:?}", feature);
                });

                match &geometry.value {
                    Value::Polygon(polygon) => {
                        for coords in polygon[0].iter() {
                            coordinates.push((coords[0], coords[1]));
                        }
                        coordinates
                    },
                    _ => {
                        panic!("invalid geometry definition: expecting polygons");
                    },
                }
            },
        }
    }
}

#[derive(Default, Encode)]
struct SbasMap {
    polygons: Vec<SbasPolygon>,
}

impl SbasMap {
    fn from_geojson(geo: &GeoJson) -> Self {
        match geo {
            GeoJson::FeatureCollection(collection) => {
                let mut map = Self::default();
                for feature in collection.features.iter() {
                    let polygon = SbasPolygon::from_feature(&feature);
                    map.polygons.push(polygon);
                }
                map
            },
            _ => {
                panic!("invalid geojson interpretation");
            },
        }
    }
}

/// Retrieves the SBAS geoservices maps (.geojson)
/// and stores them as a small static array of geo::Polygons
#[cfg(feature = "sbas")]
fn build_sbas_service_polygons() {
    let encoding_config = config::standard();

    let outdir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&outdir).join("sbas_polygons.bin");

    // read database
    let geojson_database = format!(
        "{}/data/coarse_sbas_coverage.geojson",
        env!("CARGO_MANIFEST_DIR")
    );

    let data = std::fs::read_to_string(geojson_database).unwrap_or_else(|e| {
        panic!("Failed to read SBAS coverage database: {}", e);
    });

    let geojson = data.parse::<GeoJson>().unwrap_or_else(|e| {
        panic!("Failed to build geojson database: {}", e);
    });

    let sbas_map = SbasMap::from_geojson(&geojson);

    let bytes = bincode::encode_to_vec(&sbas_map, encoding_config).unwrap_or_else(|e| {
        panic!("failed to serialize polygon map: {}", e);
    });

    std::fs::write(out_path, bytes).unwrap_or_else(|e| {
        panic!("failed to serialize polygon map: {}", e);
    });
}

fn main() {
    build_sbas_helper();

    #[cfg(feature = "sbas")]
    build_sbas_service_polygons();

    #[cfg(feature = "sbas")]
    println!("cargo:rerun-if-changed=data/coarse_sbas_coverage.geojson");

    println!("cargo:rerun-if-changed=data/sbas.geojson");
}
