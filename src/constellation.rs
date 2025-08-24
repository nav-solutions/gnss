//! Constellation definition
use hifitime::TimeScale;
use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "python")]
use pyo3::prelude::pyclass;

/// Constellation parsing & identification related errors
#[derive(Error, Clone, Debug, PartialEq)]
pub enum ParsingError {
    #[error("unknown constellation \"{0}\"")]
    Unknown(String),
}

/// Describes all known `GNSS` constellations
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "python", pyclass)]
#[cfg_attr(feature = "python", pyo3(module = "gnss"))]
pub enum Constellation {
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
    Mixed,
}

impl std::fmt::Display for Constellation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::GPS => write!(f, "GPS"),
            Self::Glonass => write!(f, "Glonass"),
            Self::BeiDou => write!(f, "BDS"),
            Self::QZSS => write!(f, "QZSS"),
            Self::Galileo => write!(f, "Galileo"),
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
            Self::AusNZ => write!(f, "AusNZ"),
            Self::GBAS => write!(f, "GBAS"),
            Self::NSAS => write!(f, "NSAS"),
            Self::ASAL => write!(f, "ASAL"),
            Self::Mixed => write!(f, "MIXED"),
        }
    }
}

impl Constellation {
    /// Returns true if this [Constellation] is an augmentation system
    pub fn is_sbas(&self) -> bool {
        matches!(
            *self,
            Constellation::WAAS
                | Constellation::KASS
                | Constellation::BDSBAS
                | Constellation::EGNOS
                | Constellation::GAGAN
                | Constellation::SDCM
                | Constellation::ASBAS
                | Constellation::SPAN
                | Constellation::MSAS
                | Constellation::NSAS
                | Constellation::ASAL
                | Constellation::AusNZ
                | Constellation::SBAS
        )
    }

    pub(crate) fn is_mixed(&self) -> bool {
        *self == Constellation::Mixed
    }

    /// Returns associated time scale. Returns None
    /// if related time scale is not supported.
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

impl std::str::FromStr for Constellation {
    type Err = ParsingError;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let s = string.trim().to_lowercase();
        match s.as_str() {
            "g" | "gps" => Ok(Self::GPS),
            "c" | "bds" => Ok(Self::BeiDou),
            "e" | "gal" => Ok(Self::Galileo),
            "r" | "glo" => Ok(Self::Glonass),
            "j" | "qzss" => Ok(Self::QZSS),
            "i" | "irnss" => Ok(Self::IRNSS),
            "s" | "sbas" => Ok(Self::SBAS),
            "m" | "mixed" => Ok(Self::Mixed),
            "ausnz" => Ok(Self::AusNZ),
            "egnos" => Ok(Self::EGNOS),
            "waas" => Ok(Self::WAAS),
            "kass" => Ok(Self::KASS),
            "gagan" => Ok(Self::GAGAN),
            "asbas" => Ok(Self::ASBAS),
            "nsas" => Ok(Self::NSAS),
            "asal" => Ok(Self::ASAL),
            "msas" => Ok(Self::MSAS),
            "span" => Ok(Self::SPAN),
            "gbas" => Ok(Self::GBAS),
            "sdcm" => Ok(Self::SDCM),
            "bdsbas" => Ok(Self::BDSBAS),
            _ if s.contains("gps") => Ok(Self::GPS),
            _ if s.contains("glonass") => Ok(Self::Glonass),
            _ if s.contains("beidou") => Ok(Self::BeiDou),
            _ if s.contains("galileo") => Ok(Self::Galileo),
            _ if s.contains("qzss") => Ok(Self::QZSS),
            _ if s.contains("sbas") | s.contains("geo") => Ok(Self::SBAS),
            _ if s.contains("irnss") | s.contains("navic") => Ok(Self::IRNSS),
            _ if s.contains("mix") => Ok(Self::Mixed),
            _ => Err(ParsingError::Unknown(string.to_string())),
        }
    }
}

impl std::fmt::LowerHex for Constellation {
    /*
     * {:x}: formats Self as single letter standard code
     */
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
    /*
     * {:X} formats Self as 3 letter standard code
     */
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hifitime::TimeScale;
    use std::str::FromStr;
    #[test]
    fn from_str() {
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

        for desc in ["X", "x", "GPX", "gpx", "unknown", "blah"] {
            assert!(Constellation::from_str(desc).is_err());
        }
    }
    #[test]
    fn format() {
        for (constell, expected) in [
            (Constellation::GPS, "GPS"),
            (Constellation::BeiDou, "BDS"),
            (Constellation::Glonass, "Glonass"),
            (Constellation::Galileo, "Galileo"),
            (Constellation::QZSS, "QZSS"),
            (Constellation::IRNSS, "IRNSS"),
            (Constellation::WAAS, "WAAS"),
            (Constellation::Mixed, "MIXED"),
        ] {
            assert_eq!(constell.to_string(), expected);
        }
    }
    #[test]
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
        ] {
            assert_eq!(gnss.timescale(), Some(expected));
        }
    }
}
