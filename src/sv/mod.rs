//! Satellite vehicle definition
use thiserror::Error;

use crate::{
    constellation::{Constellation, ParsingError as ConstellationParsingError},
    prelude::{Epoch, TimeScale},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
mod python;

/// [SV] (Satellite Vehicle) definition.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "python", pyclass)]
#[cfg_attr(feature = "python", pyo3(module = "gnss"))]
pub struct SV {
    /// PRN identification number for this vehicle.
    pub prn: u8,

    /// [Constellation] to which this vehicle belongs to.
    pub constellation: Constellation,
}

// Database, built by build.rs, for detailed SBAS vehicle identification
include!(concat!(env!("OUT_DIR"), "/sbas.rs"));

/// Issues during parsing
#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("constellation parsing error: {0}")]
    ConstellationParsing(#[from] ConstellationParsingError),

    #[error("invalid PRN number")]
    InvalidPRN,
}

impl SV {
    /// Creates a new [SV] from [Constellation] and PRN number.
    pub fn new(constellation: Constellation, prn: u8) -> Self {
        Self { constellation, prn }
    }

    /// Returns the [Timescale] to which this satellite belongs to.
    /// ```
    /// extern crate gnss_rs as gnss;
    ///
    /// use gnss::sv;
    /// use std::str::FromStr;
    /// use gnss::prelude::*;
    ///
    /// assert_eq!(sv!("G01").timescale(), Some(TimeScale::GPST));
    /// assert_eq!(sv!("E13").timescale(), Some(TimeScale::GST));
    /// ```
    pub fn timescale(&self) -> Option<TimeScale> {
        self.constellation.timescale()
    }

    /// Tries to retrieve SBAS/GEO details for this satellite.
    /// We use the PRN# (+100, according to SBAS definitions)
    /// as an identifier in the database.
    pub(crate) fn sbas_definitions(prn: u8) -> Option<&'static SBASHelper<'static>> {
        let to_find = (prn as u16) + 100;
        SBAS_VEHICLES
            .iter()
            .filter(|e| e.prn == to_find)
            .reduce(|e, _| e)
    }

    /// Returns launch date and time, expressed as [Epoch], for this particular satellite. NB: this is limited to SBAS/GEO vehicles.
    pub fn launch_date(&self) -> Option<Epoch> {
        let definition = SV::sbas_definitions(self.prn)?;
        Some(Epoch::from_gregorian_utc_at_midnight(
            definition.launched_year,
            definition.launched_month,
            definition.launched_day,
        ))
    }

    /// Returns True if [Self] is a BeiDou Geostationnary satellite
    pub fn is_beidou_geo(&self) -> bool {
        self.constellation == Constellation::BeiDou && (self.prn < 6 || self.prn > 58)
    }
}

impl std::str::FromStr for SV {
    type Err = ParsingError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let constellation = Constellation::from_str(&string[0..1])?;

        let prn = string[1..]
            .trim()
            .parse::<u8>()
            .or(Err(ParsingError::InvalidPRN))?;

        let mut ret = SV { constellation, prn };

        if constellation.is_sbas() {
            // map the SXX to meaningful SBAS
            if let Some(sbas) = SV::sbas_definitions(prn) {
                // this can't fail because the SBAS database only
                // contains valid Constellations
                ret.constellation = Constellation::from_str(sbas.constellation).unwrap();
            }
        }

        Ok(ret)
    }
}

impl std::fmt::UpperHex for SV {
    /*
     * Possibly detailed identity for SBAS vehicles
     */
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.constellation.is_sbas() {
            if let Some(sbas) = SV::sbas_definitions(self.prn) {
                write!(f, "{}", sbas.id)
            } else {
                write!(f, "{:x}", self)
            }
        } else {
            write!(f, "{:x}", self)
        }
    }
}

impl std::fmt::LowerHex for SV {
    /*
     * Prints self as XYY standard format
     */
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:x}{:02}", self.constellation, self.prn)
    }
}

impl std::fmt::Display for SV {
    /*
     * Prints self as XYY standard format
     */
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:x}{:02}", self.constellation, self.prn)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;
    #[test]
    fn from_str() {
        for (descriptor, expected) in vec![
            ("G01", SV::new(Constellation::GPS, 1)),
            ("G 1", SV::new(Constellation::GPS, 1)),
            ("G33", SV::new(Constellation::GPS, 33)),
            ("C01", SV::new(Constellation::BeiDou, 1)),
            ("C 3", SV::new(Constellation::BeiDou, 3)),
            ("R01", SV::new(Constellation::Glonass, 1)),
            ("R 1", SV::new(Constellation::Glonass, 1)),
            ("C254", SV::new(Constellation::BeiDou, 254)),
            ("E4 ", SV::new(Constellation::Galileo, 4)),
            ("R 9", SV::new(Constellation::Glonass, 9)),
            ("I 3", SV::new(Constellation::IRNSS, 3)),
            ("I09", SV::new(Constellation::IRNSS, 9)),
            ("I16", SV::new(Constellation::IRNSS, 16)),
        ] {
            let sv = SV::from_str(descriptor);
            assert!(
                sv.is_ok(),
                "failed to parse sv from \"{}\" - {:?}",
                descriptor,
                sv.err().unwrap()
            );
            let sv = sv.unwrap();
            assert_eq!(
                sv, expected,
                "badly identified {} from \"{}\"",
                sv, descriptor
            );
        }
    }
    #[test]
    fn sbas_from_str() {
        for (desc, parsed, lowerhex, upperhex) in vec![
            ("S 3", SV::new(Constellation::SBAS, 3), "S03", "S03"),
            (
                "S22",
                SV::new(Constellation::AusNZ, 22),
                "S22",
                "INMARSAT-4F1",
            ),
            ("S23", SV::new(Constellation::EGNOS, 23), "S23", "ASTRA-5B"),
            ("S25", SV::new(Constellation::SDCM, 25), "S25", "Luch-5A"),
            ("S 5", SV::new(Constellation::SBAS, 5), "S05", "S05"),
            ("S48", SV::new(Constellation::ASAL, 48), "S48", "ALCOMSAT-1"),
        ] {
            let sv = SV::from_str(desc).unwrap();
            assert_eq!(sv, parsed, "failed to parse correct sv from \"{}\"", desc);
            assert_eq!(format!("{:x}", sv), lowerhex);
            assert_eq!(format!("{:X}", sv), upperhex);
            assert!(sv.constellation.is_sbas(), "should be sbas");
        }
    }
    #[test]
    fn sbas_db_sanity() {
        for sbas in SBAS_VEHICLES.iter() {
            /* verify PRN */
            assert!(sbas.prn > 100);

            /* verify constellation */
            let constellation = Constellation::from_str(sbas.constellation);
            assert!(
                constellation.is_ok(),
                "sbas database should only contain valid constellations: \"{}\"",
                sbas.constellation,
            );

            let constellation = constellation.unwrap();
            assert_eq!(constellation.timescale(), Some(TimeScale::GPST));

            /* verify launch date */
            let _ = Epoch::from_gregorian_utc_at_midnight(
                sbas.launched_year,
                sbas.launched_month,
                sbas.launched_day,
            );
        }
    }
    #[test]
    fn test_beidou_geo() {
        assert_eq!(SV::from_str("G01").unwrap().is_beidou_geo(), false);
        assert_eq!(SV::from_str("E01").unwrap().is_beidou_geo(), false);
        assert_eq!(SV::from_str("C01").unwrap().is_beidou_geo(), true);
        assert_eq!(SV::from_str("C02").unwrap().is_beidou_geo(), true);
        assert_eq!(SV::from_str("C06").unwrap().is_beidou_geo(), false);
        assert_eq!(SV::from_str("C48").unwrap().is_beidou_geo(), false);
        assert_eq!(SV::from_str("C59").unwrap().is_beidou_geo(), true);
        assert_eq!(SV::from_str("C60").unwrap().is_beidou_geo(), true);
    }
}
