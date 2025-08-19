/*
 * gnss.rs
 * Define new planetary datasets to enhance ANISE datasets
 *
 * rtk-rs @github.com
 */
use env_logger::{Builder, Target};
use std::path::PathBuf;

use anise::{
    //constants::celestial_objects::EARTH,
    constants::frames::{EARTH_ITRF93, EARTH_J2000},
    prelude::MetaAlmanac,
    structure::planetocentric::{ellipsoid::Ellipsoid, PlanetaryData},
};

fn ellipsoid(flatenning: f64, semi_major_km: f64, semi_minor_km: f64) -> Ellipsoid {
    let r = (semi_major_km + semi_minor_km) / 2.0;
    Ellipsoid {
        polar_radius_km: r - flatenning * r,
        semi_major_equatorial_radius_km: semi_major_km,
        semi_minor_equatorial_radius_km: semi_minor_km,
    }
}

fn main() {
    let mut builder = Builder::from_default_env();
    builder
        .target(Target::Stdout)
        .format_timestamp_secs()
        .format_module_path(true)
        .init();

    let gps_wgs84 = ellipsoid(298.257223563_f64, 6378.137_f64, 6356.7523142_f64);

    let mut almanac =
        MetaAlmanac::latest().unwrap_or_else(|e| panic!("MetaAlmanac::latest() error: {}", e));

    // define new frames
    for (frame_id, frame_name, shape) in [(3990001, "GPS WGS84", gps_wgs84)] {
        //let earth_itrf93 = EARTH_ITRF93;
        //let planetary_data = almanac
        //    .planetary_data
        //    .get_by_name("GPS WGS84")
        //    .unwrap_or_else(|e| panic!("get_by_name({}) failure: {}", "GPS WGS84", e));

        let mut earth_inertial = almanac
            .frame_from_uid(EARTH_ITRF93)
            .unwrap_or_else(|e| panic!("Failed to retrieve Earth from_uid: {}", e));

        earth_inertial.shape = Some(shape);

        let planetary_data = PlanetaryData {
            parent_id: 0,
            shape: Some(shape),
            object_id: frame_id, //earth_inertial.ephemeris_id, //frame_id,
            mu_km3_s2: earth_inertial.mu_km3_s2.unwrap_or_default(),
            long_axis: None,
            prime_meridian: None,
            pole_declination: None,
            pole_right_ascension: None,
            num_nut_prec_angles: 0,
            nut_prec_angles: [Default::default(); 32],
        };

        almanac
            .planetary_data
            .push(planetary_data, Some(frame_id), Some(frame_name))
            .unwrap_or_else(|e| {
                panic!("Failed to extend planetary data with {}: {}", frame_name, e)
            });
    }
    // finally: save the new dataset so we can use it later on
    let path = PathBuf::new().join("pck_gnss.pca");

    almanac.planetary_data.set_crc32();

    almanac
        .planetary_data
        .save_as(&path, true)
        .unwrap_or_else(|e| {
            panic!(
                "Failed to save new GNSS Planetary Constant ANISE (PCA): {}",
                e
            )
        });
}

#[cfg(test)]
mod test {
    use anise::constants::celestial_objects::EARTH;
    use anise::constants::orientations::ITRF93;
    use anise::prelude::{Almanac, Frame, FrameUid};
    use anise::structure::PlanetaryDataSet;
    use std::fs::read_to_string;
    use std::fs::File;
    use std::io::Read;
    // This test requires main to run once before... :)
    #[test]
    fn test_gnss_pck() {
        //let mut bytes = Vec::<u8>::new();
        //// load gnss_pck.pca
        //let almanac = Almanac::default();
        //let mut fd = File::open("pck_gnss.pca")
        //    .unwrap_or_else(|e| panic!("Failed to read GNSS PCA: does file exist? {}", e));
        //fd.read_to_end(&mut bytes)
        //    .unwrap_or_else(|e| panic!("Failed to read GNSS PCA: does file exist? {}", e));

        //let pck = PlanetaryDataSet::from_bytes(bytes);
        //almanac.with_planetary_data(pck);

        // much simpler, and seems to work
        let mut almanac = Almanac::default();
        almanac
            .load("pck_gnss.pca")
            .unwrap_or_else(|e| panic!("Failed to read GNSS PCA: does file exist? {}", e));

        // load new definitions
        for key in ["GPS WGS84"] {
            let earth_itrf93 = Frame::new(EARTH, ITRF93);

            let planetary_data = almanac
                .planetary_data
                .get_by_name(key)
                .unwrap_or_else(|e| panic!("Failed to fetch {}: {}", key, e));

            let frame_uid = FrameUid {
                ephemeris_id: earth_itrf93.ephemeris_id,
                orientation_id: earth_itrf93.orientation_id,
            };

            let earth_itrf93_wgs84 = planetary_data.to_frame(frame_uid);

            // rotation attempt
        }
    }
}
