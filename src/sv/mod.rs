//! Space vehicle definition
use hifitime::TimeScale;
use thiserror::Error;

use crate::constellation::{Constellation, ParsingError as ConstellationParsingError};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "std")]
use hifitime::{Duration, Epoch};

#[cfg(feature = "std")]
use std::str::FromStr;

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
mod python;

// #[cfg(feature = "cospar")]
// use crate::prelude::COSPAR;

/// ̀SV describes a Satellite Vehicle
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "python", pyclass)]
#[cfg_attr(feature = "python", pyo3(module = "gnss"))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SV {
    /// PRN identification number for this vehicle
    pub prn: u8,

    /// [Constellation] to which this satellite belongs to
    pub constellation: Constellation,
}

// Includes the SBAS definition database
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/sbas.rs"));

/// ̀[SV] parsing related issues.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ParsingError {
    #[error("constellation parsing error: {0}")]
    ConstellationParsing(#[from] ConstellationParsingError),

    #[error("failed to parse PRN numer")]
    PrnParsing,
}

impl SV {
    /// Builds desired Satellite Vehicle ([SV]) regardless of the
    /// both field values. Prefer [Self::new_sbas] to conveniently
    /// idenfity geostationnary vehicles.
    /// ```
    /// use gnss_rs::sv;
    /// use gnss_rs::prelude::*;
    ///
    /// use std::str::FromStr;
    /// use hifitime::{TimeScale, Epoch};
    ///
    /// // This method lets you construct satellites that may not exist
    /// let sv = SV::new(Constellation::GPS, 1);
    ///
    /// assert_eq!(sv.constellation, Constellation::GPS);
    /// assert_eq!(sv.prn, 1);
    /// assert_eq!(sv.launch_datetime(), None); // only for SBAS vehicles
    /// ```
    pub const fn new(constellation: Constellation, prn: u8) -> Self {
        Self { prn, constellation }
    }

    /// Tries to identify this [Constellation::SBAS] satellite from
    /// a PRN number ranging from 0..100 (RINEX like format).
    /// Simply substract 100 to the true satellite ID number.
    ///
    /// ```
    /// use gnss_rs::sv;
    /// use gnss_rs::prelude::*;
    ///
    /// use std::str::FromStr;
    /// use hifitime::{TimeScale, Epoch, MonthName};
    ///
    /// // This only works if satellite do exist in our database
    /// assert!(SV::new_sbas(1).is_none());
    ///
    /// let egnos_geo23 = SV::new_sbas(23)
    ///     .unwrap(); // GEO #123
    ///
    /// assert_eq!(egnos_geo23.prn, 23);
    /// assert!(egnos_geo23.constellation.is_sbas()); // obviously
    /// assert_eq!(egnos_geo23.constellation, Constellation::EGNOS); // smart builder
    ///
    /// let launch_date = egnos_geo23.launch_datetime()
    ///     .unwrap(); // only for detailed SBAS
    ///
    /// assert_eq!(launch_date.year(), 2021);
    /// assert_eq!(launch_date.month_name(), MonthName::November);
    /// ```
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn new_sbas(prn: u8) -> Option<Self> {
        let definition = Self::sbas_definitions(prn)?;

        if let Ok(constellation) = Constellation::from_str(definition.constellation) {
            Some(Self { prn, constellation })
        } else {
            None
        }
    }

    /// Returns [Timescale] to which [Self] belongs to.
    /// ```
    /// extern crate gnss_rs as gnss;
    ///
    /// use hifitime::TimeScale;
    /// use gnss::sv;
    /// use gnss::prelude::*;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(sv!("G01").timescale(), Some(TimeScale::GPST));
    /// assert_eq!(sv!("E13").timescale(), Some(TimeScale::GST));
    /// ```
    pub fn timescale(&self) -> Option<TimeScale> {
        self.constellation.timescale()
    }

    /// Explores SBAS detail database and tries to provide more detail from unique
    /// PRN number (+100).
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    fn sbas_definitions(prn: u8) -> Option<&'static SBASHelper<'static>> {
        let to_find = (prn as u16) + 100;
        SBAS_VEHICLES
            .iter()
            .filter(|e| e.prn == to_find)
            .reduce(|e, _| e)
    }

    /// Returns launch date and time expressed as UTC [Epoch].  
    /// This API is limited to [Constellation::SBAS] vehicles for which we have a builtin database.
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn launch_datetime(&self) -> Option<Epoch> {
        let definition = SV::sbas_definitions(self.prn)?;

        if let Ok(epoch) = Epoch::from_str(definition.launch) {
            // failures will not happen here,
            // all entires are tested in CI/CD
            Some(epoch)
        } else {
            None
        }
    }

    // /// Returns the [COSPAR] number (unique launch identification code)
    // /// for this satellite, if known. This API is limited to [Constellation::SBAS] vehicles
    // /// for which we have a builtin database.
    // #[cfg(feature = "cospar")]
    // #[cfg_attr(docsrs, doc(cfg(feature = "cospar")))]
    // pub fn cospar_number(&self) -> Option<COSPAR> {
    //     let definitions = SV::sbas_definitions(self.prn)?;
    //     let launch_datetime = self.launch_datetime()?;
    //     Some(COSPAR::new(
    //         launch_datetime.year(),
    //         definitions.cospar_number,
    //         definitions.cospar_code,
    //     ))
    // }

    /// Returns the space flight [Duration] at this particular point in time
    /// expressed as [Epoch], for this [SV]. This is limited to [Constellation::SBAS]
    /// vehicles for which we have a builtin database.
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn duration_since_launch(&self, now: Epoch) -> Option<Duration> {
        let datetime = self.launch_datetime()?;
        Some(now - datetime)
    }

    /// Returns True if this [SV] is a [Constellation::BeiDou] geostationnary satellite.
    pub fn is_beidou_geo(&self) -> bool {
        self.constellation == Constellation::BeiDou && (self.prn < 6 || self.prn > 58)
    }
}

#[cfg(feature = "std")]
impl core::str::FromStr for SV {
    type Err = ParsingError;
    /// Parses [SV] from "CNN" standard 3 letter code, where
    /// - C is a 1 letter constellation identifier
    /// - NN is a 2 digit PRN number
    ///
    /// When built with std library supported, the interpretation
    /// is more detailed for SBAS vehicles, because
    /// we have a database builtin. For example, S23 is EutelSAT 5WB.
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let constellation = Constellation::from_str(&string[0..1])?;

        if let Ok(prn) = string[1..].trim().parse::<u8>() {
            let mut ret = SV::new(constellation, prn);
            if constellation.is_sbas() {
                // map the SXX to meaningful SBAS
                if let Some(sbas) = SV::sbas_definitions(prn) {
                    // this can't fail because the SBAS database only
                    // contains valid Constellations
                    ret.constellation = Constellation::from_str(sbas.constellation).unwrap();
                }
            }
            Ok(ret)
        } else {
            Err(ParsingError::PrnParsing)
        }
    }
}

#[cfg(not(feature = "std"))]
impl core::str::FromStr for SV {
    type Err = ParsingError;
    /// Parses [SV] from "CNN" standard 3 letter code, where
    /// - C is a 1 letter constellation identifier
    /// - NN is a 2 digit PRN number
    ///
    /// When built without std library supported, the interpretation
    /// is limited to basic vehicles. For example "G01" is GPS 01,
    /// and S23 can only be interpreted as SBAS-23, because the SBAS
    /// database is not builtin without std library.
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let constellation = Constellation::from_str(&string[0..1])?;
        if let Ok(prn) = string[1..].trim().parse::<u8>() {
            Ok(SV::new(constellation, prn))
        } else {
            Err(ParsingError::PrnParsing)
        }
    }
}

#[cfg(not(feature = "std"))]
impl core::fmt::Display for SV {
    /// Formats this [SV] with possible details (if known in our database).
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:x}", self)
    }
}

#[cfg(feature = "std")]
impl core::fmt::Display for SV {
    /// Formats this [SV] with possible details (if known in our database).
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if self.constellation.is_sbas() {
            if let Some(sbas) = SV::sbas_definitions(self.prn) {
                write!(f, "{}", sbas.name)
            } else {
                write!(f, "{:x}", self)
            }
        } else {
            write!(f, "{:x}", self)
        }
    }
}

impl core::fmt::LowerHex for SV {
    /// Formats this [SV] in CNN format, where:
    /// - C is a single letter [Constellation] identifier
    /// - NN is a two-digit PRN number
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
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
        for (desc, parsed, displayed, lowerhex) in vec![
            ("S 3", SV::new(Constellation::SBAS, 3), "S03", "S03"),
            (
                "S22",
                SV::new(Constellation::AusNZ, 22),
                "INMARSAT-4F1",
                "S22",
            ),
            ("S23", SV::new(Constellation::EGNOS, 23), "ASTRA-5B", "S23"),
            ("S25", SV::new(Constellation::SDCM, 25), "Luch-5A", "S25"),
            ("S 5", SV::new(Constellation::SBAS, 5), "S05", "S05"),
            ("S48", SV::new(Constellation::ASAL, 48), "ALCOMSAT-1", "S48"),
        ] {
            let sv = SV::from_str(desc).unwrap();
            assert_eq!(sv, parsed, "failed to parse correct sv from \"{}\"", desc);
            assert_eq!(sv.to_string(), displayed);
            assert_eq!(format!("{:x}", sv), lowerhex);
            assert!(sv.constellation.is_sbas(), "should be sbas");
        }
    }

    #[test]
    fn test_database() {
        for sbas in SBAS_VEHICLES.iter() {
            assert!(sbas.prn > 100, "SBAS PRN should be >100");

            assert!(
                Constellation::from_str(sbas.constellation).is_ok(),
                "corrupt database content: \"{}\"",
                sbas.constellation,
            );

            assert!(
                Epoch::from_str(sbas.launch).is_ok(),
                "corrupt launch datetime: \"{}\"",
                sbas.launch
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
