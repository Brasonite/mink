use pyo3::prelude::*;

use crate::stats::Stats;

pub fn install(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    parent.add_class::<Stats>()?;

    parent.add("stats", None::<Stats>)?;

    Ok(())
}
