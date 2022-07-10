
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn sum_list_of_ints(v: Vec<usize>) -> PyResult<usize> {
    let mut sum: usize = 0;
    for n in v {
        sum += n;
    }
    Ok(sum)
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_maturin_test(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(sum_list_of_ints, m)?)?;
    Ok(())
}
