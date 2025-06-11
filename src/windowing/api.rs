use pyo3::prelude::*;

use crate::windowing::Window;

pub fn install(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    parent.add_class::<Window>()?;

    parent.add("window", None::<Window>)?;

    Ok(())
}
