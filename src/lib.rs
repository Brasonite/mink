use pyo3::prelude::*;

mod assets;
mod audio;
mod graphics;
mod input;
mod math;
mod runtime;
mod stats;
mod time;
mod windowing;

#[pymodule]
fn mink(m: &Bound<'_, PyModule>) -> PyResult<()> {
    assets::api::install(m)?;
    audio::api::install(m)?;
    graphics::api::install(m)?;
    input::api::install(m)?;
    math::api::install(m)?;
    stats::api::install(m)?;
    time::api::install(m)?;
    windowing::api::install(m)?;
    m.add_function(wrap_pyfunction!(runtime::run, m)?)?;

    Ok(())
}
