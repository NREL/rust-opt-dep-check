/// Crate for verifying functionality of feature-dependent optional dependencies
/// # Features
/// - `directories` - Disable functionality that relies on `directories` crate.  Enabled by default --
///   compile with `--no-default-features` flag to disable.

fn main() {
    #[cfg(not(feature = "directories"))]
    println!("You successfully compiled and ran the whole crate WITHOUT `directories` enabled!");
    #[cfg(feature = "directories")]
    println!("You successfully compiled and ran the whole crate WITH `directories` enabled!");
    #[cfg(feature = "directories")]
    println!("Home dir: {:?}", directories::UserDirs::new().unwrap().home_dir());
}
