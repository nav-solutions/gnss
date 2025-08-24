use crate::prelude::{Constellation, TimeScale, SV};
use pyo3::prelude::*;
use std::str::FromStr;

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
    fn py_new(constellation: &str, prn: u8) -> PyResult<Self> {
        let constellation = Constellation::from_str(constellation).or(Err(
            pyo3::exceptions::PyValueError::new_err(format!(
                "Unknown constellation: {}",
                constellation,
            )),
        ))?;

        Ok(Self { prn, constellation })
    }

    #[getter(prn)]
    fn get_prn(&self) -> PyResult<u8> {
        Ok(self.prn)
    }

    #[setter(prn)]
    fn set_prn(&mut self, value: u8) -> PyResult<()> {
        self.prn = value;
        Ok(())
    }

    #[getter(constellation)]
    fn get_constellation(&self) -> String {
        self.constellation.to_string()
    }

    #[setter(constellation)]
    fn set_constellation(&mut self, value: &str) -> PyResult<()> {
        let constellation = Constellation::from_str(value).or(Err(
            pyo3::exceptions::PyValueError::new_err(format!("Unknown constellation: {}", value,)),
        ))?;

        self.constellation = constellation;
        Ok(())
    }

    #[pyo3(name = "timescale")]
    fn py_timescale(&self) -> PyResult<TimeScale> {
        let ts = self.timescale().ok_or(
            pyo3::exceptions::PyValueError::new_err(format!("timescale not defined"))
        )?;

        Ok(ts)
    }

    fn __str__(&self) -> String {
        format!("{:x}{:02}", self.constellation, self.prn)
    }
}
