/// Crate for verifying functionality of feature-dependent optional dependencies
/// # Features
/// - `pyo3-log` - Disable functionality that relies on `pyo3-log` crate.  Enabled by default --
///   compile with `--no-default-features` flag to disable.

fn main() {
    #[cfg(not(feature = "pyo3-log"))]
    println!("You successfully compiled and ran the whole crate WITHOUT `pyo3-log` enabled!");
    #[cfg(feature = "pyo3-log")]
    println!("You successfully compiled and ran the whole crate WITH `pyo3-log` enabled!");
}
