use pyo3::prelude::*;
use std::str::FromStr;
use crate::prelude::{Constellation, TimeScale};

#[pymethods]
impl Constellation {
    #[new]
    fn new(value: &str) -> PyResult<Self> {
        let constellation = Constellation::from_str(value).or(Err(
            pyo3::exceptions::PyValueError::new_err(format!("Unknown constellation: {}", value,)),
        ))?;

        Ok(constellation)
    }

    fn __str__(&self) -> String {
        format!("{}", self)
    }

    #[pyo3(name = "timescale")]
    fn py_timescale(&self) -> PyResult<TimeScale> {
        let ts = self.timescale().ok_or(
            pyo3::exceptions::PyValueError::new_err(format!("timescale not defined"))
        )?;

        Ok(ts)
    }

    #[getter]
    fn py_get(&self) -> String {
        self.to_string()
    }

    #[setter]
    fn py_set(&mut self, value: &str) -> PyResult<()> {
        let constellation = Constellation::from_str(value).or(Err(
            pyo3::exceptions::PyValueError::new_err(format!("Unknown constellation: {}", value,)),
        ))?;

        *self = constellation;
        Ok(())
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
