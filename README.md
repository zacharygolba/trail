# trail

[![CircleCI branch](https://img.shields.io/circleci/project/github/zacharygolba/trail/master.svg?style=flat-square)](https://circleci.com/gh/zacharygolba/trail/tree/master) [![AppVeyor branch](https://img.shields.io/appveyor/ci/zacharygolba/trail/master.svg?logo=appveyor&style=flat-square)](https://ci.appveyor.com/project/zacharygolba/trail/branch/master) [![Crates.io](https://img.shields.io/crates/v/trail.svg?style=flat-square)](https://crates.io/crates/trail)

Build cross-platform paths at compile time.

## Installation

First, add `trail` to the dependencies section of your `Cargo.toml`:

```toml
[dependencies]
trail = "0.1"
```

Next, add the following snippet to the entry point of your crate (`lib.rs` or `main.rs`):

```rust
#[macro_use]
extern crate trail;
#
# fn main() {}
```

## Usage

You can also use [`trail!`] anywhere else an expression is expected. The expanded output
is functionally equivelant to calling [`Path::new`] with a hard coded literal.

*Posix*

```rust
assert_eq!(trail!("", "hello", "world"), Path::new("/hello/world"));
assert_eq!(trail!("hello", "world"), Path::new("hello/world"));
```

*Windows*

```rust
assert_eq!(trail!("", "hello", "world"), Path::new("\\hello\\world"));
assert_eq!(trail!("hello", "world"), Path::new("hello\\world"));
```

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
