use pyo3::prelude::*;

mod process;

#[pymodule]
fn rsutil(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<process::Process>()?;
    Ok(())
}
