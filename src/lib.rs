pub mod core;

use pyo3::prelude::*;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn distance_haversine_m(lat1: f32, lon1: f32, lat2: f32, lon2:f32) -> PyResult<f32>{
    let res = core::distance::distance_haversine_m(lat1, lon1, lat2, lon2);
    Ok(res)
}

/// A Python module implemented in Rust.
#[pymodule]
fn geopapyrus(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(distance_haversine_m, m)?)?;

    Ok(())
}