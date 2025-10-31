//! GNSS constellations
use hifitime::TimeScale;
use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Constellation parsing & identification related errors
#[derive(Error, Clone, Debug, PartialEq)]
pub enum ParsingError {
    #[error("unknown constellation \"{0}\"")]
    Unknown(String),
}

/// Describes all known `GNSS` constellations
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Constellation {
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

    /// Autralia and New-Zealand Geostationary service
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
    Mixed,
}

impl std::fmt::Display for Constellation {
    /// Formats the the constellation full name along its country code.
    /// Any output here is compatible with [Constellation::from_str] reciprocal parsing.
    ///
    /// For example:
    /// - "GPS (US)" for american constellation
    /// - "Glonass (RU)" for russian constellation
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::GPS => write!(f, "GPS (US)"),
            Self::Glonass => write!(f, "Glonass (RU)"),
            Self::BeiDou => write!(f, "BDS (CH)"),
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
            Self::AusNZ => write!(f, "AusNZ (AUS)"),
            Self::GBAS => write!(f, "GBAS (UK)"),
            Self::NSAS => write!(f, "NSAS (NI)"),
            Self::ASAL => write!(f, "ASAL (AL)"),
            Self::Mixed => write!(f, "MIXED"),
        }
    }
}

impl std::fmt::UpperExp for Constellation {
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
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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

impl std::fmt::LowerHex for Constellation {
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
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
    /// Returns true if Self is an augmentation system
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
        )
    }

    /// Returns the country code two or three letter code,
    /// for this [Constellation], when that applies.
    /// For example:
    /// - "US" for [Constellation::GPS]
    /// - "EU" for [Constellation::Galileo]
    /// - None for [Constellation::SBAS]
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
        } else if lower.starts_with("aus") {
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

    #[test]
    fn formating() {
        for (constellation, displayed, upper_exp, upper_hex) in [
            (Constellation::GPS, "GPS (US)", "GPS", "G"),
            (Constellation::Glonass, "Glonass (RU)", "GLO", "R"),
            (Constellation::BeiDou, "BeiDou (CH)", "BDS", "C"),
        ] {
            assert_eq!(constellation.to_string(), displayed);
            assert_eq!(format!("{:E}", constellation), upper_exp);
            assert_eq!(format!("{:X}", constellation), upper_hex);

            // reciprocal
            assert_eq!(
                Constellation::from_str(displayed),
                constellation,
                "reciprocal failed for {}",
                displayed
            );
            assert_eq!(
                Constellation::from_str(upper_exp),
                constellation,
                "reciprocal failed for {}",
                upper_exp
            );
        }
    }

    #[test]
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
        ] {
            let constellation = Constellation::from_str(constellation_str).unwrap();

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
            ("WAAS", None),
            ("EGNOS", None),
            ("KASS", None),
            ("ASBAS", None),
            ("GBAS", None),
            ("GAGAN", None),
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
