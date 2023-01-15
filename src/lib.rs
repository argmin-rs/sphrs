// Copyright 2018-2023 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! A general purpose spherical/solid harmonics library in Rust.
//!
//! This crate offers real and complex spherical harmonics and solid harmonics.
//!
//! * [Spherical Harmonics (Wikipedia)](https://en.wikipedia.org/wiki/Spherical_harmonics)
//! * [Regular Solid and Irregular Solid Harmonics (Wikipedia)](https://en.wikipedia.org/wiki/Solid_harmonics)
//!
//!
//! Documentation:
//! * [latest released version](https://docs.rs/sphrs/latest/sphrs)
//! * [main branch](https://argmin-rs.github.io/sphrs/sphrs/)
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
#![doc = concat!("sphrs = \"", env!("CARGO_PKG_VERSION"), "\"")]
//! ```
//!
//! # Tutorial
//!
//! There are two enums [`RealSH`] and [`ComplexSH`], each with the following variants:
//!
//! * `Spherical`
//! * `RegularSolid`
//! * `IrregularSolid`
//!
//! These variants are used to define which kind of spherical/solid harmonic is to be computed.
//! Each enum implements the [`SHEval`] trait, which provides an [`SHEval::eval`] method.
//! This method is used to compute the spherical/solid harmonic for [`Coordinates`].
//! Coordinates define a position in space and can be constructed from Cartesian and spherical
//! coordinates via [`Coordinates::cartesian`] and [`Coordinates::spherical`], respectively.
//!
//! In order to compute real valued spherical harmonics, one needs to call the
//! [`eval`](`SHEval::eval`) method on the `Spherical` variant of the `RealSH` enum.
//! The `eval` method is part of the [`SHEval`] trait and as such this trait must be in scope.
//!
//! ```rust
//! use sphrs::{Coordinates, RealSH, SHEval};
//!
//! // l = 2
//! let degree = 2;
//! // m = 1
//! let order = 1;
//!
//! // Define the position where the SH will be evaluated at
//! // in Cartesian coordinates
//! let p = Coordinates::cartesian(1.0, 0.2, 1.4);
//!
//! // Compute the real-valued SH value at `p` for l = 2, m = 1
//! let computed_sh = RealSH::Spherical.eval(degree, order, &p);
//!
//! println!("SH ({}, {}): {:?}", degree, order, computed_sh);
//! ```
//!
//! This library can also compute [`HarmonicsSet`]s which contains all spherical/solid harmonics
//! up to a given order.
//!
//! The following example shows how to compute complex spherical harmonics up to third order at
//! the spherical coordinates (r, theta, phi) = (1.0, 0.8, 0.4):
//!
//! ```rust
//! use sphrs::{ComplexSH, Coordinates, HarmonicsSet};
//!
//! // l = 3
//! let degree = 3;
//!
//! // Create the harmonics set (in this case for complex SH)
//! let sh = HarmonicsSet::new(degree, ComplexSH::Spherical);
//!
//! // Position in spherical coordinates where the set is evaluated at
//! let p = Coordinates::spherical(1.0, 0.8, 0.4);
//!
//! // Evaluate. Returns a `Vec<f64>`
//! let set = sh.eval(&p);
//!
//! println!("SH up to degree {}: {:?}", degree, set);
//! ```
//!
//! The individual SH in the set can also be multiplied element-wise with a vector of coefficients
//! with the function [`HarmonicsSet::eval_with_coefficients`]:
//!
//! ```rust
//! # use sphrs::{ComplexSH, HarmonicsSet, Coordinates};
//! # let degree = 3;
//! let sh = HarmonicsSet::new(degree, ComplexSH::Spherical);
//! # let p = Coordinates::spherical(1.0, 0.8, 0.4);
//! // Must be the same length as the set.
//! let coeff = vec![2.0; sh.num_sh()];
//! let set = sh.eval_with_coefficients(&p, coeff.as_slice());
//! println!("SH up to degree {}: {:?}", degree, set);
//! ```
//!
//! # Advanced features
//!
//! Feel free to directly use the low level functions linked at the bottom of this page.
//!
//! # Acknowledgements
//!
//! This crate is heavily inspired by Google's
//! [spherical-harmonics](https://github.com/google/spherical-harmonics) library and follows the
//! design documented [here](https://theory.cpe.ku.ac.th/~pramook/files/spherical-harmonic-lighting.pdf).
//!
//! # References
//!
//! * Robin Green, ["Spherical Harmonic Lighting: The Gritty Details"](https://theory.cpe.ku.ac.th/~pramook/files/spherical-harmonic-lighting.pdf)
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
