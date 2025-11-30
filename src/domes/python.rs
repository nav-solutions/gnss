use crate::domes::{TrackingPoint, DOMES};
use pyo3::{pymethods, PyResult};

#[pymethods]
impl TrackingPoint {
    fn __str__(&self) -> String {
        format!("{}", self)
    }

    fn __format__(&self, specs: &str) -> PyResult<String> {
        match specs {
            "x" => Ok(format!("{:x}", self)),
            _ => Ok(format!("{}", self)),
        }
    }
}

#[pymethods]
impl DOMES {
    #[new]
    fn new(area: u16, site: u8, point: TrackingPoint, sequential: u16) -> Self {
        Self {
            area,
            site,
            point,
            sequential,
        }
    }

    fn __str__(&self) -> String {
        format!("{}", self)
    }

    fn __format__(&self, _specs: &str) -> PyResult<String> {
        Ok(format!("{}", self))
    }
}
