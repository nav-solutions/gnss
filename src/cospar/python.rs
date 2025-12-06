use crate::prelude::COSPAR;
use pyo3::{pymethods, PyResult};

#[pymethods]
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
