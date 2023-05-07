use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
/// [pyfunction]をつけると、Pythonバインディングできるようになる
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
/// [pymodule]をつけることで、パッケージとして出すことができる
/// m.add_functionでバインディングする関数を定義することでpythonから使えるようになる
#[pymodule]
fn maturin_handson(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}