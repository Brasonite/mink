use pyo3::prelude::*;

use crate::graphics::draw::Draw;

pub fn install(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    parent.add_class::<Draw>()?;

    parent.add("draw", None::<Draw>)?;

    Ok(())
}
