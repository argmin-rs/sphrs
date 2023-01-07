// Copyright 2018-2020 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! A general purpose spherical/solid harmonics library in Rust.
//!
//! Documentation: [stable](https://docs.rs/sphrs/latest/sphrs),
//! [main](https://argmin-rs.github.io/sphrs/sphrs/).
//!
//! # Types of spherical/solid harmonics
//!
//! This crate supports these types of real and complex functions via the enums `RealSH` and
//! `ComplexSH`:
//!
//! * [Spherical](https://en.wikipedia.org/wiki/Spherical_harmonics)
//! * [RegularSolid and IrregularSolid](https://en.wikipedia.org/wiki/Solid_harmonics)
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! sphrs = "0.1.3"
//! ```
//!
//! # Examples
//!
//! Compute the complex spherical harmonic function of degree 2 and order 1 at (spherical) position
//! (r = 1.0, theta = PI/4, phi = PI/4):
//!
//! ```rust
//! use sphrs::{ComplexSH, Coordinates, SHEval};
//! use std::f64::consts::PI;
//!
//! let sh = ComplexSH::Spherical;
//! let degree = 2;
//! let order = 1;
//! let p = Coordinates::spherical(1.0, PI/4.0, PI/8.0);
//! println!("SH ({}, {}): {:?}", degree, order, sh.eval(degree, order, &p));
//! ```
//!
//! Compute all real SH up to 4th degree at (Cartesian) position (1, 0, 0):
//!
//! ```rust
//! use sphrs::{RealSH, HarmonicsSet, Coordinates};
//! let degree = 4;
//! let sh = HarmonicsSet::new(degree, RealSH::Spherical);
//! let p = Coordinates::cartesian(1.0, 0.0, 0.0);
//! println!("SH up to degree {}: {:?}", degree, sh.eval(&p));
//! ```
//!
//! # Acknowledgements
//!
//! This crate is heavily inspired by Google's
//! [spherical-harmonics](https://github.com/google/spherical-harmonics) library and follows the
//! design documented
//! [here](http://silviojemma.com/public/papers/lighting/spherical-harmonic-lighting.pdf).
//!
//! # References
//!
//! * Robin Green, ["Spherical Harmonic Lighting: The Gritty Details"](http://silviojemma.com/public/papers/lighting/spherical-harmonic-lighting.pdf)
//! * [Spherical harmonics (Wikipedia)](https://en.wikipedia.org/wiki/Spherical_harmonics)
//! * [Solid harmonics (Wikipedia)](https://en.wikipedia.org/wiki/Spherical_harmonics)
//!
//! # License
//!
//! Licensed under either of
//!
//!   * Apache License, Version 2.0,
//!     ([LICENSE-APACHE](https://github.com/argmin-rs/argmin/blob/main/LICENSE-APACHE) or
//!     <http://www.apache.org/licenses/LICENSE-2.0>)
//!   * MIT License ([LICENSE-MIT](https://github.com/argmin-rs/argmin/blob/main/LICENSE-MIT) or
//!     <http://opensource.org/licenses/MIT>)
//!
//! at your option.
//!
//!
//! ## Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion
//! in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above,
//! without any additional terms or conditions.

#![warn(missing_docs)]

mod coordinates;
mod float;
mod sh;

pub use crate::coordinates::{Coordinates, SHCoordinates};
pub use crate::float::SphrsFloat;
pub use crate::sh::*;
