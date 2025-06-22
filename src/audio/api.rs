use pyo3::prelude::*;

use crate::audio::Audio;

pub fn install(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    parent.add_class::<Audio>()?;
    parent.add("audio", None::<Audio>)?;

    Ok(())
}
