use pyo3::prelude::*;

use crate::input::Input;

pub fn install(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    parent.add_class::<Input>()?;

    parent.add("input", None::<Input>)?;

    Ok(())
}
