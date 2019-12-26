[![Build Status](https://travis-ci.org/argmin-rs/sphrs.svg?branch=master)](https://travis-ci.org/argmin-rs/sphrs)

# sphrs

A (work in progress) general purpose spherical/solid harmonics library in Rust.

Documentation: [stable](https://doc.rs/sphrs/latest/sphrs),
[master](https://argmin-rs.github.io/sphrs/sphrs/).

## Types of spherical/solid harmonics

This crate supports these types of real SH via the enum `RealSHType`:

* [Standard](https://en.wikipedia.org/wiki/Spherical_harmonics)
* [RegularSolid and IrregularSolid](https://en.wikipedia.org/wiki/Solid_harmonics)

TODO: complex SH

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
sphrs = "*"
```

## Example

Compute the sum of all real SH up to 5th order at position (1, 0, 0):

```rust
let sh_type = RealSHType::Standard;
let order = 5;
let sh = RealSphericalHarmonics::new(order, sh_type);
let p = Coordinates::cartesian(1.0, 0.0, 0.0);
println!("Sum of SH up to order {}: {}", order, sh.eval(&p));
```

## Acknowledgements

This crate is heavily inspired by Google's
[spherical-harmonics](https://github.com/google/spherical-harmonics) library and follows the
design documented
[here](https://pdfs.semanticscholar.org/83d9/28031e78f15d9813061b53d25a4e0274c751.pdf).

## References

* Robin Green, ["Spherical Harmonic Lighting: The Gritty Details"](http://silviojemma.com/public/papers/lighting/spherical-harmonic-lighting.pdf)
* [Spherical harmonics (Wikipedia)](https://en.wikipedia.org/wiki/Spherical_harmonics)
* [Solid harmonics (Wikipedia)](https://en.wikipedia.org/wiki/Spherical_harmonics)

## License

Licensed under either of

  * Apache License, Version 2.0,
    ([LICENSE-APACHE](https://github.com/argmin-rs/argmin/blob/master/LICENSE-APACHE) or
    http://www.apache.org/licenses/LICENSE-2.0)
  * MIT License ([LICENSE-MIT](https://github.com/argmin-rs/argmin/blob/master/LICENSE-MIT) or
    http://opensource.org/licenses/MIT)

at your option.


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion
in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above,
without any additional terms or conditions.

License: MIT OR Apache-2.0
