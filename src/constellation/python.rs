use crate::prelude::{Constellation, TimeScale};
use std::str::FromStr;

use pyo3::{prelude::*, types::PyType};

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

    fn __format__(&self, spec: &str) -> PyResult<String> {
        match spec {
            "x" => Ok(format!("{:x}", self)),
            _ => Ok(format!("{}", self)),
        }
    }

    #[classmethod]
    #[pyo3(name = "from_country_code")]
    fn py_from_country_code(_cls: &Bound<'_, PyType>, code: &str) -> PyResult<Self> {
        // fn py_from_country_code(code: &str) -> PyResult<Self> {
        let constell = Self::from_country_code(code).ok_or(
            pyo3::exceptions::PyValueError::new_err(format!("invalid country code")),
        )?;
        Ok(constell)
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

    #[getter]
    fn py_get(&self) -> Constellation {
        *self
    }

    #[setter]
    fn py_set(&mut self, value: Constellation) -> PyResult<()> {
        *self = value;
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
