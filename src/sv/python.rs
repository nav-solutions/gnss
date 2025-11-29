use crate::prelude::{Constellation, TimeScale, SV};
use pyo3::prelude::*;

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
    fn py_new(constellation: Constellation, prn: u8) -> PyResult<Self> {
        Ok(Self { prn, constellation })
    }

    #[getter(prn)]
    fn get_prn(&self) -> PyResult<u8> {
        Ok(self.prn)
    }

    #[setter(prn)]
    fn set_prn(&mut self, value: u8) {
        self.prn = value;
    }

    #[getter(constellation)]
    fn get_constellation(&self) -> Constellation {
        self.constellation
    }

    #[setter(constellation)]
    fn set_constellation(&mut self, value: Constellation) {
        self.constellation = value;
    }

    #[pyo3(name = "timescale")]
    fn py_timescale(&self) -> PyResult<TimeScale> {
        let ts = self
            .timescale()
            .ok_or(pyo3::exceptions::PyValueError::new_err(format!(
                "timescale not defined"
            )))?;

        Ok(ts)
    }

    fn __str__(&self) -> String {
        format!("{:x}{:02}", self.constellation, self.prn)
    }
}
