use pyo3::prelude::*;

use crate::time::Time;

pub fn install(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    parent.add_class::<Time>()?;

    parent.add("time", None::<Time>)?;

    Ok(())
}
