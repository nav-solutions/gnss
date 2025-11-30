use crate::prelude::{Constellation, DOMESTrackingPoint, Epoch, TimeScale, COSPAR, DOMES, SV};
use pyo3::prelude::*;

#[pymodule]
fn gnss(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Epoch>()?;
    m.add_class::<TimeScale>()?;
    m.add_class::<Constellation>()?;
    m.add_class::<SV>()?;
    m.add_class::<COSPAR>()?;
    m.add_class::<DOMES>()?;
    m.add_class::<DOMESTrackingPoint>()?;
    Ok(())
}
