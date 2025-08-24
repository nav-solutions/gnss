use crate::prelude::{Constellation, SV};
use pyo3::prelude::*;

#[pymodule]
fn gnss(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Constellation>()?;
    m.add_class::<SV>()?;
    Ok(())
}
