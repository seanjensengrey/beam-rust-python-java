use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use levenshtein::levenshtein as lev_f;

/// This module is a python module implemented in Rust.
#[pymodule]
fn lev10(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "levenshtein")]
    fn levenshtein_py(_py: Python, a: &str, b: &str) -> usize {
        levenshtein(a,b)
    }
    Ok(())
}


pub fn levenshtein(a: &str, b: &str) -> usize {
    lev_f(a,b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lev_t0() {
        assert_eq!(1, levenshtein("foo","zoo"))
    }
}
