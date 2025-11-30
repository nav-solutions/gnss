//! COSPAR (Launch) ID number
use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "python")]
use pyo3::{pymethods, PyResult};

#[cfg(feature = "python")]
use pyo3::prelude::pyclass;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid COSPAR number")]
    InvalidFormat,
}

/// COSPAR ID number
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "python", pyclass)]
#[cfg_attr(feature = "python", pyo3(module = "gnss"))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct COSPAR {
    /// Launch year
    pub year: u16,

    /// Launch number for that year, in chronological order.
    pub launch: u16,

    /// Up to three letter code representing the sequential
    /// identifier of a piece in a Launch.
    pub code: String,
}

impl COSPAR {
    /// Define a new [COSPAR] with updated year of launch.
    pub fn with_launch_year(&self, year: u16) -> Self {
        let mut s = self.clone();
        s.year = year;
        s
    }

    /// Define a new [COSPAR] with updated launch number (in that year).
    pub fn with_launch_number(&self, launch: u16) -> Self {
        let mut s = self.clone();
        s.launch = launch;
        s
    }

    /// Define a new [COSPAR] with updated 3 letter launch sequential code.
    pub fn with_launch_code(&self, code: &str) -> Self {
        let mut s = self.clone();
        s.code = code.to_string();
        s
    }
}

impl core::fmt::Display for COSPAR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:04}-{:03}{}", self.year, self.launch, self.code)
    }
}

impl core::str::FromStr for COSPAR {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 9 {
            return Err(Error::InvalidFormat);
        }
        let offset = s.find('-').ok_or(Error::InvalidFormat)?;
        let (year, rem) = s.split_at(offset);
        let year = year.parse::<u16>().map_err(|_| Error::InvalidFormat)?;
        let launch = rem[1..4]
            .trim()
            .parse::<u16>()
            .map_err(|_| Error::InvalidFormat)?;
        Ok(Self {
            year,
            launch,
            code: rem[4..].trim().to_string(),
        })
    }
}

#[cfg_attr(feature = "python", pymethods)]
impl COSPAR {
    /// Builds a new [COSPAR] identification number.
    /// The launch sequence number should be a 3 letter code (unchecked).
    #[new]
    fn py_new(year: u16, launch: u16, code: &str) -> Self {
        Self {
            year,
            launch,
            code: code.to_string(),
        }
    }

    fn __str__(&self) -> String {
        format!("{}", self)
    }

    fn __format__(&self, _specs: &str) -> PyResult<String> {
        Ok(format!("{}", self))
    }

    // #[getter(year)]
    // fn py_get(&self) -> u16 {
    //     self.year
    // }
    //
    // #[getter(launch)]
    // fn py_get(&self) -> u16 {
    //     self.launch
    // }
    //
    // #[getter(code)]
    // fn py_get(&self) -> &str {
    //     self.code
    // }
}

#[cfg(test)]
mod test {
    use crate::cospar::COSPAR;
    use std::str::FromStr;
    #[test]
    fn cospar() {
        for (desc, expected) in [
            (
                "2018-080A",
                COSPAR {
                    year: 2018,
                    launch: 80,
                    code: "A".to_string(),
                },
            ),
            (
                "1996-068A",
                COSPAR {
                    year: 1996,
                    launch: 68,
                    code: "A".to_string(),
                },
            ),
        ] {
            let cospar = COSPAR::from_str(desc).unwrap();
            assert_eq!(cospar, expected);
            let recip = cospar.to_string();
            assert_eq!(recip, desc, "cospar reciprocal");
        }
    }
}
