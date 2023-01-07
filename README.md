[![sphrs CI](https://github.com/argmin-rs/sphrs/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/argmin-rs/sphrs/actions/workflows/ci.yml)
[![Gitter chat](https://badges.gitter.im/argmin-rs/community.png)](https://gitter.im/argmin-rs/community)

# sphrs

sphrs is a general purpose spherical/solid harmonics library in Rust.

Documentation: [stable](https://docs.rs/sphrs/latest/sphrs),
[main](https://argmin-rs.github.io/sphrs/sphrs/).

## Types of spherical/solid harmonics

This crate supports these types of real and complex functions via the enums `RealSHType` and
`ComplexSHType`:

* [Spherical](https://en.wikipedia.org/wiki/Spherical_harmonics)
* [RegularSolid and IrregularSolid](https://en.wikipedia.org/wiki/Solid_harmonics)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
sphrs = "0.1.3"
```

## Examples

Compute the complex spherical harmonic function of degree 2 and order 1 at (spherical) position
(r = 1.0, theta = PI/4, phi = PI/4):

```rust
use sphrs::{ComplexSHType, Coordinates, SHEval};
use std::f64::consts::PI;

let sh = ComplexSHType::Spherical;
let degree = 2;
let order = 1;
let p = Coordinates::spherical(1.0, PI/4.0, PI/8.0);
println!("SH ({}, {}): {:?}", degree, order, sh.eval(degree, order, &p));
```

Compute all real SH up to 5th degree at (Cartesian) position (1, 0, 0):

```rust
use sphrs::{RealSHType, HarmonicsSet, Coordinates};
let degree = 5;
let sh: HarmonicsSet<f64, _, _> = HarmonicsSet::new(degree, RealSHType::Spherical);
let p = Coordinates::cartesian(1.0, 0.0, 0.0);
println!("SH up to degree {}: {:?}", degree, sh.eval(&p));
```

## Acknowledgements

This crate is heavily inspired by Google's
[spherical-harmonics](https://github.com/google/spherical-harmonics) library and follows the
design documented
[here](http://silviojemma.com/public/papers/lighting/spherical-harmonic-lighting.pdf).

## References

* Robin Green, ["Spherical Harmonic Lighting: The Gritty Details"](http://silviojemma.com/public/papers/lighting/spherical-harmonic-lighting.pdf)
* [Spherical harmonics (Wikipedia)](https://en.wikipedia.org/wiki/Spherical_harmonics)
* [Solid harmonics (Wikipedia)](https://en.wikipedia.org/wiki/Spherical_harmonics)

## License

Licensed under either of

  * Apache License, Version 2.0,
    ([LICENSE-APACHE](https://github.com/argmin-rs/argmin/blob/main/LICENSE-APACHE) or
    http://www.apache.org/licenses/LICENSE-2.0)
  * MIT License ([LICENSE-MIT](https://github.com/argmin-rs/argmin/blob/main/LICENSE-MIT) or
    http://opensource.org/licenses/MIT)

at your option.


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion
in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above,
without any additional terms or conditions.

License: MIT OR Apache-2.0
