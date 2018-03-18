//! Build cross-platform paths at compile time.
//!
//! ## Installation
//!
//! First, add [`trail`] to the dependencies section of your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! trail = "0.1"
//! ```
//!
//! Next, add the following snippet to the entry point of your crate (`lib.rs` or `main.rs`):
//!
//! ```rust
//! #[macro_use]
//! extern crate trail;
//! #
//! # fn main() {}
//! ```
//!
//! ## Usage
//!
//! You can also use [`trail!`] anywhere else an expression is expected. The expanded output
//! is functionally equivelant to calling [`Path::new`] with a hard coded literal.
//!
//! ```rust
//! # #[macro_use]
//! # extern crate trail;
//! #
//! # use std::path::Path;
//! #
//! # fn main() {
//! let absolute = &trail!("", "hello", "world");
//! let relative = &trail!("hello", "world");
//!
//! if cfg!(windows) {
//!     assert_eq!(*absolute, Path::new("\\hello\\world"));
//!     assert_eq!(*relative, Path::new("hello\\world"));
//! } else {
//!     assert_eq!(*absolute, Path::new("/hello/world"));
//!     assert_eq!(*relative, Path::new("hello/world"));
//! }
//! # }
//! ```
//!
//! [`trail!`]: ./macro.trail.html
//! [`trail`]: https://crates.io/crates/trail
//!
//! [`Path`]: https://doc.rust-lang.org/std/path/struct.Path.html
//! [`Path::new`]: https://doc.rust-lang.org/std/path/struct.Path.html#method.new

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

/// A macro for building a cross-platform [`Path`] at compile time.
///
/// ## Example
///
/// ```rust
/// # #[macro_use]
/// # extern crate trail;
/// #
/// # use std::path::Path;
/// #
/// # fn main() {
/// let absolute = &trail!("", "hello", "world");
/// let relative = &trail!("hello", "world");
///
/// if cfg!(windows) {
///     assert_eq!(*absolute, Path::new("\\hello\\world"));
///     assert_eq!(*relative, Path::new("hello\\world"));
/// } else {
///     assert_eq!(*absolute, Path::new("/hello/world"));
///     assert_eq!(*relative, Path::new("hello/world"));
/// }
/// # }
/// ```
///
/// [`Path`]: https://doc.rust-lang.org/std/path/struct.Path.html
#[macro_export]
macro_rules! trail {
    ( $($segment:expr),* ) => ( ::std::path::Path::new(__trail!($($segment),*)) );
}

#[doc(hidden)]
#[cfg(not(windows))]
#[macro_export]
macro_rules! __trail {
    () => ( "" );
    ( $base:expr ) => ( $base );
    ( $base:expr, $($segment:expr),+ ) => ( concat!($base, $("/", $segment),+) );
}

#[doc(hidden)]
#[cfg(windows)]
#[macro_export]
macro_rules! __trail {
    () => ( "" );
    ( $base:expr ) => ( $base );
    ( $base:expr, $($segment:expr),+ ) => ( concat!($base, $("\\", $segment),+) );
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn trail() {
        if cfg!(windows) {
            assert_eq!(trail!(), Path::new(""));
            assert_eq!(trail!(""), Path::new(""));
            assert_eq!(trail!("hello"), Path::new("hello"));
            assert_eq!(trail!("hello", "world"), Path::new("hello\\world"));
            assert_eq!(trail!("", "hello", "world"), Path::new("\\hello\\world"));
        } else {
            assert_eq!(trail!(), Path::new(""));
            assert_eq!(trail!(""), Path::new(""));
            assert_eq!(trail!("hello"), Path::new("hello"));
            assert_eq!(trail!("hello", "world"), Path::new("hello/world"));
            assert_eq!(trail!("", "hello", "world"), Path::new("/hello/world"));
        }
    }
}
