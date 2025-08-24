use crate::{constellation::Constellation, prelude::SV};

use pyo3::prelude::*;
// use pyo3::types::PyType;

#[pymethods]
impl SV {
    /// Builds a new SV
    /// ```
    /// extern crate gnss_rs as gnss;
    ///
    /// use gnss::sv;
    /// use gnss::prelude::*;
    /// use std::str::FromStr;
    /// use hifitime::{TimeScale, Epoch};
    ///
    /// let sv = SV::new(Constellation::GPS, 1);
    /// assert_eq!(sv.constellation, Constellation::GPS);
    /// assert_eq!(sv.prn, 1);
    /// assert_eq!(sv, sv!("G01"));
    /// assert_eq!(sv.launch_date(), None);
    ///
    /// let launch_date = Epoch::from_str("2021-11-01T00:00:00 UTC")
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     sv!("S23").launch_date(),
    ///     Some(launch_date));
    /// ```
    #[new]
    pub const fn new(constellation: Constellation, prn: u8) -> Self {
        Self { prn, constellation }
    }

    pub const fn prn(&self) -> u8 {
        self.prn
    }

    pub const fn constellation(&self) -> Constellation {
        self.constellation
    }


    fn __str__(&self) -> String {
        format!("{:x}{:02}", self.constellation, self.prn)
    }
}
