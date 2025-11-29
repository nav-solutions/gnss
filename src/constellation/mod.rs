//! Constellation definition
use hifitime::TimeScale;
use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "python")]
use pyo3::prelude::pyclass;

#[cfg(feature = "python")]
mod python;

/// Constellation parsing & identification related errors
#[derive(Error, Clone, Debug, PartialEq)]
pub enum ParsingError {
    #[error("Unknown constellation")]
    Unknown,
}

/// [Constellation] describes all known `GNSS` systems.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "python", pyclass)]
#[cfg_attr(feature = "python", pyo3(module = "gnss"))]
pub enum Constellation {
<<<<<<< HEAD:src/constellation/mod.rs
    /// [Self::GPS] american constellation
    #[default]
    GPS,

    /// [Self::Glonass] russian constellation
    Glonass,

    /// [Self::BeiDou] chinese constellation.
    /// This constellation contains both IGSO, GEO and standard
    /// orbits. Yet a SBAS augmentation also exists and is defined here.
    /// It is not clear whether BeiDou GEO vehicles belon to [Self::BDSBAS] yet.
    BeiDou,

    /// [Self::QZSS] japanese constellation
    QZSS,

    /// [Self::Galileo] european constellation
    Galileo,

    /// [Self::IRNSS] constellation, renamed "NavIC"
    IRNSS,

    /// [Self::WAAS] american augmentation system
    WAAS,

    /// [Self::EGNOS] european augmentation system
    EGNOS,

    /// [Self::MSAS] japanese augmentation system
    MSAS,

    /// [Self::GAGAN] indian augmentation system
    GAGAN,

    /// [Self::BDSBAS] chinese augmentation system.
    BDSBAS,

    /// [Self::KASS] south Korean augmentation system
    KASS,

    /// [Self::SDCM] russian augmentation system
    SDCM,

    /// [Self::ASBAS] south-african augmentation system
    ASBAS,

    /// [Self::SPAN] australian / NZ augmentation system
    SPAN,

    /// Generic [Self::SBAS] is used to describe SBAS
    /// systems undefined in this database.
    SBAS,

    /// [Self::AusNZ] australian / NZ geoscience system
    AusNZ,

    /// [Self::GBAS] UK augmentation system
    GBAS,

    /// [Self::NSAS] nigerian augmentation system
    NSAS,

    /// [Self::ASAL] algerian augmentation system
    ASAL,

    /// [Self::MIXED] is used to describe products or datasets
    /// that contain several [Constellation]s.
=======
    /// American constellation
    #[default]
    GPS,

    /// Russian constellation
    Glonass,

    /// Chinese constellation
    BeiDou,

    /// Japanese constellation
    QZSS,

    /// European constellation
    Galileo,

    /// Indian constellation, sometimes renamed NAV/IC
    IRNSS,

    /// American Geostationary service
    WAAS,

    /// European Geostationary service
    EGNOS,

    /// Japanese MTSAT Geostationary service
    MSAS,

    /// Indian Geostationary service
    GAGAN,

    /// Chinese Geostationary service
    BDSBAS,

    /// South Korean Geostationary service
    KASS,

    /// Russian Geostationary service
    SDCM,

    /// South African Geostationary service
    ASBAS,

    /// South-PAN Autralia and New-Zealand Geostationary service
    SPAN,

    /// Undetermined or generic Geostationary service.
    SBAS,

    /// Australia and New-Zealand geoscience service
    AusNZ,

    /// UK Geostationary service
    GBAS,

    /// Nigerian Geostationary service
    NSAS,

    /// Algerian Geostationary service
    ASAL,

    /// Describes the combination of [Constellation]s,
    /// used by modern receivers and RINEX files.
>>>>>>> main:src/constellation.rs
    Mixed,
}

impl core::fmt::Display for Constellation {
    /// Formats the the constellation full name along its country code.
    /// Any output here is compatible with [Constellation::from_str] reciprocal parsing.
    ///
    /// For example:
    /// - "GPS (US)" for american constellation
    /// - "Glonass (RU)" for russian constellation
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::GPS => write!(f, "GPS (US)"),
            Self::Glonass => write!(f, "Glonass (RU)"),
            Self::BeiDou => write!(f, "BeiDou (CH)"),
            Self::QZSS => write!(f, "QZSS (JP)"),
            Self::Galileo => write!(f, "Galileo (EU)"),
            Self::IRNSS => write!(f, "IRNSS (IN)"),
            Self::WAAS => write!(f, "WAAS (US)"),
            Self::EGNOS => write!(f, "EGNOS (EU)"),
            Self::MSAS => write!(f, "MSAS (JP)"),
            Self::GAGAN => write!(f, "GAGAN (IN)"),
            Self::BDSBAS => write!(f, "BDSBAS (CH)"),
            Self::KASS => write!(f, "KASS (KR)"),
            Self::SDCM => write!(f, "SDCM (RU)"),
            Self::ASBAS => write!(f, "ASBAS (SA)"),
            Self::SPAN => write!(f, "SPAN (AUS)"),
            Self::SBAS => write!(f, "SBAS"),
            Self::AusNZ => write!(f, "AUS/NZ (AUS)"),
            Self::GBAS => write!(f, "GBAS (UK)"),
            Self::NSAS => write!(f, "NSAS (NI)"),
            Self::ASAL => write!(f, "ASAL (AL)"),
            Self::Mixed => write!(f, "MIXED"),
        }
    }
}

impl core::fmt::UpperExp for Constellation {
    /// Format the [Constellation] accronym, without the country code.
    /// Any output here is compatible with [Constellation::from_str] reciprocal parsing.
    ///
    /// For example:
    /// - "GPS" for american [Constellation]
    /// - "GLO" for russian [Constellation]
    /// - "BDS" for chinese [Constellation]
    /// - "IRN" for indian [Constellation]
    /// - "GAL" for european [Constellation]
    /// - "QZS" for japanese [Constellation]
    /// - "WAAS" for american geo service
    /// - "KASS" for korean geo service
    /// - "SDCM" for russian geo service
    /// - "EGNOS" for european geo service
    /// - "SPAN" for australian and NZ geo service
    /// - "AUS/NZ" for australian and NZ geoscience service
    /// - "GBAS" for UK geo service
    /// - "MIX" for [Constellation::MIXED] setup
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::GPS => write!(f, "GPS"),
            Self::Glonass => write!(f, "GLO"),
            Self::BeiDou => write!(f, "BDS"),
            Self::QZSS => write!(f, "QZSS"),
            Self::Galileo => write!(f, "GAL"),
            Self::IRNSS => write!(f, "IRNSS"),
            Self::WAAS => write!(f, "WAAS"),
            Self::EGNOS => write!(f, "EGNOS"),
            Self::MSAS => write!(f, "MSAS"),
            Self::GAGAN => write!(f, "GAGAN"),
            Self::BDSBAS => write!(f, "BDSBAS"),
            Self::KASS => write!(f, "KASS"),
            Self::SDCM => write!(f, "SDCM"),
            Self::ASBAS => write!(f, "ASBAS"),
            Self::SPAN => write!(f, "SPAN"),
            Self::SBAS => write!(f, "SBAS"),
            Self::AusNZ => write!(f, "AUS/NZ"),
            Self::GBAS => write!(f, "GBAS"),
            Self::NSAS => write!(f, "NSAS"),
            Self::ASAL => write!(f, "ASAL"),
            Self::Mixed => write!(f, "MIX"),
        }
    }
}

impl core::fmt::LowerHex for Constellation {
    /// Formats this [Constellation] using a single letter,
    /// as per the standard RINEX file naming convention.
    /// This will generate 'S' for any geo service.
    ///
    /// For example:
    /// - 'G' for [Constellation::GPS]
    /// - 'R' for [Constellation::Glonass]
    /// - 'C' for [Constellation::BeiDou]
    /// - 'E' for [Constellation::Galileo]
    /// - 'J' for [Constellation::QZSS]
    /// - 'I' for [Constellation::IRNSS]
    /// - 'S' for any [Constellation::SBAS]
    /// - 'M' for any [Constellation::MIXED]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::GPS => write!(f, "G"),
            Self::Glonass => write!(f, "R"),
            Self::Galileo => write!(f, "E"),
            Self::BeiDou => write!(f, "C"),
            Self::QZSS => write!(f, "J"),
            Self::IRNSS => write!(f, "I"),
            other => {
                if other.is_sbas() {
                    write!(f, "S")
                } else if other.is_mixed() {
                    write!(f, "M")
                } else {
                    unreachable!("non existing case");
                }
            },
        }
    }
}

impl Constellation {
    /// Returns true if this [Constellation] is an augmentation system
    pub fn is_sbas(&self) -> bool {
        matches!(
            *self,
            Self::WAAS
                | Self::KASS
                | Self::BDSBAS
                | Self::EGNOS
                | Self::GAGAN
                | Self::SDCM
                | Self::ASBAS
                | Self::SPAN
                | Self::MSAS
                | Self::NSAS
                | Self::ASAL
                | Self::AusNZ
                | Self::SBAS
                | Self::GBAS
        )
    }

<<<<<<< HEAD:src/constellation/mod.rs
    pub(crate) fn is_mixed(&self) -> bool {
        matches!(self, Constellation::Mixed)
    }

    /// Returns associated time scale. Returns None
    /// if related time scale is not supported.
=======
    /// Returns the country code two or three letter code,
    /// for this [Constellation], when that applies.
    /// For example:
    /// - "US" for [Constellation::GPS]
    /// - "EU" for [Constellation::Galileo]
    /// - None for [Constellation::SBAS]
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn country_code(&self) -> Option<String> {
        let code = match self {
            Self::GPS | Self::WAAS => Some("US"),
            Self::Glonass | Self::SDCM => Some("RU"),
            Self::BeiDou | Self::BDSBAS => Some("CH"),
            Self::QZSS | Self::MSAS => Some("JP"),
            Self::Galileo | Self::EGNOS => Some("EU"),
            Self::IRNSS | Self::GAGAN => Some("IN"),
            Self::KASS => Some("KR"),
            Self::ASBAS => Some("SA"),
            Self::GBAS => Some("UK"),
            Self::NSAS => Some("NI"),
            Self::ASAL => Some("AL"),
            Self::SPAN | Self::AusNZ => Some("AUS/NZ"),
            _ => None,
        };

        let code = code?;
        Some(code.to_string())
    }

    /// Tries to build a [Constellation] from a country-code
    /// (not case sensitive).
    ///
    /// Returns:
    /// - [Constellation::GPS] for "US" or "USA"
    /// - [Constellation::Galileo] for "EU" or "Europe"
    /// - [Constellation::BeiDou] for "CH" or "China"
    /// - [Constellation::Glonass] for "RU" or "Russia"
    /// - [Constellation::QZSS] for "JP" or "Japan"
    /// - [Constellation::IRNSS] for "IN" or "India"
    pub fn from_country_code(code: &str) -> Option<Self> {
        let lower = code.to_lowercase();

        if lower.starts_with("us") {
            Some(Self::GPS)
        } else if lower.starts_with("eu") {
            Some(Self::Galileo)
        } else if lower.starts_with("ch") {
            Some(Self::BeiDou)
        } else if lower.starts_with("ru") {
            Some(Self::Glonass)
        } else if lower.starts_with("jp") {
            Some(Self::QZSS)
        } else if lower.starts_with("jap") {
            Some(Self::QZSS)
        } else if lower.starts_with("in") {
            Some(Self::IRNSS)
        } else {
            None
        }
    }

    /// Tries to build a specific [Constellation::SBAS] from a country-code
    /// (not case sensitive).
    ///
    /// Returns:
    /// - [Constellation::WAAS] for "US" or "USA"
    /// - [Constellation::EGNOS] for "EU" or "Europe"
    /// - [Constellation::BDSBAS] for "CH" or "China"
    /// - [Constellation::SDCM] for "RU" or "Russia"
    /// - [Constellation::MSAS] for "JP" or "Japan"
    /// - [Constellation::GAGAN] for "IN" or "India"
    /// - [Constellation::GBAS] for "UK"
    /// - [Constellation::KASS] for "KR" or "Korea"
    /// - [Constellation::ASBAS] for "SA" or "South-Africa"
    /// - [Constellation::SPAN] for "Australia" or "New-Zealand" or "NZ"
    pub fn from_sbas_country_code(code: &str) -> Option<Self> {
        let lower = code.to_lowercase();

        if lower.starts_with("us") {
            Some(Self::WAAS)
        } else if lower.starts_with("eu") {
            Some(Self::EGNOS)
        } else if lower.starts_with("ch") {
            Some(Self::BDSBAS)
        } else if lower.starts_with("ru") {
            Some(Self::SDCM)
        } else if lower.starts_with("jp") {
            Some(Self::MSAS)
        } else if lower.starts_with("jap") {
            Some(Self::MSAS)
        } else if lower.starts_with("in") {
            Some(Self::GAGAN)
        } else if lower.starts_with("uk") {
            Some(Self::GBAS)
        } else if lower.starts_with("sa") {
            Some(Self::ASBAS)
        } else if lower.starts_with("south-af") {
            Some(Self::ASBAS)
        } else if lower.starts_with("kr") {
            Some(Self::KASS)
        } else if lower.starts_with("kor") {
            Some(Self::KASS)
        } else if lower.starts_with("australia") {
            Some(Self::SPAN)
        } else if lower.starts_with("new-zea") {
            Some(Self::SPAN)
        } else if lower.starts_with("nz") {
            Some(Self::SPAN)
        } else {
            None
        }
    }

    fn is_mixed(&self) -> bool {
        *self == Self::Mixed
    }

    /// Returns the [TimeScale] this [Constellation] represents.
    /// Returns [None] when this operation does not apply to given [Constellation].
    /// [Constellation::SBAS] are said to be refered to [TimeScale::GPST]
>>>>>>> main:src/constellation.rs
    pub fn timescale(&self) -> Option<TimeScale> {
        match self {
            Self::GPS => Some(TimeScale::GPST),
            Self::QZSS => Some(TimeScale::QZSST),
            Self::Galileo => Some(TimeScale::GST),
            Self::BeiDou => Some(TimeScale::BDT),
            Self::Glonass => Some(TimeScale::UTC),
            c => {
                if c.is_sbas() {
                    Some(TimeScale::GPST)
                } else {
                    None
                }
            },
        }
    }
}

impl core::str::FromStr for Constellation {
    type Err = ParsingError;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let s = string.trim().to_lowercase();

        // single letter reciprocal
        match s.as_str() {
            "g" => return Ok(Self::GPS),
            "c" => return Ok(Self::BeiDou),
            "e" => return Ok(Self::Galileo),
            "r" => return Ok(Self::Glonass),
            "j" => return Ok(Self::QZSS),
            "i" => return Ok(Self::IRNSS),
            "s" => return Ok(Self::SBAS),
            "m" => return Ok(Self::Mixed),
            _ => {},
        }

<<<<<<< HEAD:src/constellation/mod.rs
impl std::fmt::LowerHex for Constellation {
    /// Formats this [Constellation] using a single letter.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::GPS => write!(f, "G"),
            Self::Glonass => write!(f, "R"),
            Self::Galileo => write!(f, "E"),
            Self::BeiDou => write!(f, "C"),
            Self::QZSS => write!(f, "J"),
            Self::IRNSS => write!(f, "I"),
            c => {
                if c.is_sbas() {
                    write!(f, "S")
                } else if c.is_mixed() {
                    write!(f, "M")
                } else {
                    Err(std::fmt::Error)
                }
            },
        }
    }
}

impl std::fmt::UpperHex for Constellation {
    /// Formats this [Constellation] using a standard 3 letter code.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::GPS => write!(f, "GPS"),
            Self::Glonass => write!(f, "GLO"),
            Self::Galileo => write!(f, "GAL"),
            Self::BeiDou => write!(f, "BDS"),
            Self::QZSS => write!(f, "QZSS"),
            Self::IRNSS => write!(f, "IRNSS"),
            Self::SBAS => write!(f, "SBAS"),
            Self::WAAS => write!(f, "WAAS"),
            Self::AusNZ => write!(f, "AUSNZ"),
            Self::EGNOS => write!(f, "EGNOS"),
            Self::KASS => write!(f, "KASS"),
            Self::GAGAN => write!(f, "GAGAN"),
            Self::GBAS => write!(f, "GBAS"),
            Self::NSAS => write!(f, "NSAS"),
            Self::MSAS => write!(f, "MSAS"),
            Self::SPAN => write!(f, "SPAN"),
            Self::SDCM => write!(f, "SDCM"),
            Self::BDSBAS => write!(f, "BDSBAS"),
            Self::ASBAS => write!(f, "ASBAS"),
            Self::ASAL => write!(f, "ASAL"),
            Self::Mixed => write!(f, "MIXED"),
=======
        // smart guess
        if s.contains("gps") {
            Ok(Self::GPS)
        } else if s.contains("glo") {
            Ok(Self::Glonass)
        } else if s.contains("glonass") {
            Ok(Self::Glonass)
        } else if s.contains("beidou") {
            Ok(Self::BeiDou)
        } else if s.contains("bdsbas") {
            Ok(Self::BDSBAS)
        } else if s.contains("bds") {
            Ok(Self::BeiDou)
        } else if s.contains("galileo") {
            Ok(Self::Galileo)
        } else if s.contains("qzss") {
            Ok(Self::QZSS)
        } else if s.contains("irnss") {
            Ok(Self::IRNSS)
        } else if s.contains("nav/ic") {
            Ok(Self::IRNSS)
        } else if s.contains("span") {
            Ok(Self::SPAN)
        } else if s.contains("south-pan") {
            Ok(Self::SPAN)
        } else if s.contains("south pan") {
            Ok(Self::SPAN)
        } else if s.contains("aus/nz") {
            Ok(Self::AusNZ)
        } else if s.contains("australia") {
            Ok(Self::SPAN)
        } else if s.contains("new-zealand") {
            Ok(Self::SPAN)
        } else if s.contains("new zealand") {
            Ok(Self::SPAN)
        } else if s.contains("waas") {
            Ok(Self::WAAS)
        } else if s.contains("kass") {
            Ok(Self::KASS)
        } else if s.contains("egnos") {
            Ok(Self::EGNOS)
        } else if s.contains("gagan") {
            Ok(Self::GAGAN)
        } else if s.contains("gbas") {
            Ok(Self::GBAS)
        } else if s.contains("sdcm") {
            Ok(Self::SDCM)
        } else if s.contains("msas") {
            Ok(Self::MSAS)
        } else if s.contains("nsas") {
            Ok(Self::NSAS)
        } else if s.contains("span") {
            Ok(Self::SPAN)
        } else if s.contains("asbas") {
            Ok(Self::ASBAS)
        } else if s.contains("asal") {
            Ok(Self::ASAL)
        } else if s.contains("gal") {
            Ok(Self::Galileo)
        } else if s.contains("mix") {
            Ok(Self::Mixed)
        } else if s.contains("sbas") {
            Ok(Self::SBAS)
        } else {
            Err(ParsingError::Unknown)
>>>>>>> main:src/constellation.rs
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hifitime::TimeScale;
    use std::str::FromStr;

    #[test]
    fn parsing() {
        for (desc, expected) in vec![
            ("G", Ok(Constellation::GPS)),
            ("GPS", Ok(Constellation::GPS)),
            ("R", Ok(Constellation::Glonass)),
            ("GLO", Ok(Constellation::Glonass)),
            ("J", Ok(Constellation::QZSS)),
            ("M", Ok(Constellation::Mixed)),
            ("WAAS", Ok(Constellation::WAAS)),
            ("KASS", Ok(Constellation::KASS)),
            ("GBAS", Ok(Constellation::GBAS)),
            ("NSAS", Ok(Constellation::NSAS)),
            ("SPAN", Ok(Constellation::SPAN)),
            ("EGNOS", Ok(Constellation::EGNOS)),
            ("ASBAS", Ok(Constellation::ASBAS)),
            ("MSAS", Ok(Constellation::MSAS)),
            ("GAGAN", Ok(Constellation::GAGAN)),
            ("BDSBAS", Ok(Constellation::BDSBAS)),
            ("ASAL", Ok(Constellation::ASAL)),
            ("SDCM", Ok(Constellation::SDCM)),
        ] {
            assert_eq!(
                Constellation::from_str(desc),
                expected,
                "failed to parse constellation from \"{}\"",
                desc
            );
        }
    }

<<<<<<< HEAD:src/constellation/mod.rs
        for desc in ["X", "x", "GPX", "gpx", "unknown", "blah"] {
            assert!(Constellation::from_str(desc).is_err());
        }
    }

=======
>>>>>>> main:src/constellation.rs
    #[test]
    fn formating() {
        for (constellation, displayed, upper_exp, upper_hex) in [
            (Constellation::GPS, "GPS (US)", "GPS", "G"),
            (Constellation::Glonass, "Glonass (RU)", "GLO", "R"),
            (Constellation::BeiDou, "BeiDou (CH)", "BDS", "C"),
        ] {
            assert_eq!(constellation.to_string(), displayed);
            assert_eq!(format!("{:E}", constellation), upper_exp);
            assert_eq!(format!("{:x}", constellation), upper_hex);

            // reciprocal
            assert_eq!(
                Constellation::from_str(displayed),
                Ok(constellation),
                "reciprocal failed for {}",
                displayed
            );
            assert_eq!(
                Constellation::from_str(upper_exp),
                Ok(constellation),
                "reciprocal failed for {}",
                upper_exp
            );
        }
    }

    #[test]
<<<<<<< HEAD:src/constellation/mod.rs
    fn test_sbas() {
        for sbas in ["WAAS", "KASS", "EGNOS", "ASBAS", "MSAS", "GAGAN", "ASAL"] {
            assert!(Constellation::from_str(sbas).unwrap().is_sbas());
        }
    }

    #[test]
    fn timescale() {
        for (gnss, expected) in [
            (Constellation::GPS, TimeScale::GPST),
            (Constellation::Galileo, TimeScale::GST),
            (Constellation::BeiDou, TimeScale::BDT),
=======
    fn test_is_sbas() {
        for (constellation_str, is_sbas) in [
            ("WAAS", true),
            ("EGNOS", true),
            ("KASS", true),
            ("ASBAS", true),
            ("GBAS", true),
            ("GAGAN", true),
            ("GPS", false),
            ("GAL", false),
            ("BeiDou", false),
            ("BDS", false),
            ("QZSS", false),
>>>>>>> main:src/constellation.rs
        ] {
            let constellation = Constellation::from_str(constellation_str).unwrap_or_else(|_| {
                panic!(
                    "failed to parse constellation from \"{}\"",
                    constellation_str
                );
            });

            assert_eq!(
                constellation.is_sbas(),
                is_sbas,
                "invalid results for {}",
                constellation_str
            );
        }
    }

    #[test]
    fn test_timescale() {
        for (constellation_str, timescale) in [
            ("GPS", Some(TimeScale::GPST)),
            ("GAL", Some(TimeScale::GST)),
            ("BeiDou", Some(TimeScale::BDT)),
            ("BDS", Some(TimeScale::BDT)),
            ("QZSS", Some(TimeScale::QZSST)),
            ("WAAS", Some(TimeScale::GPST)),
            ("EGNOS", Some(TimeScale::GPST)),
            ("KASS", Some(TimeScale::GPST)),
            ("ASBAS", Some(TimeScale::GPST)),
            ("GBAS", Some(TimeScale::GPST)),
            ("GAGAN", Some(TimeScale::GPST)),
        ] {
            let constellation = Constellation::from_str(constellation_str).unwrap();

            assert_eq!(
                constellation.timescale(),
                timescale,
                "invalid results for {}",
                constellation_str
            );
        }
    }
}
