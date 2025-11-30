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
