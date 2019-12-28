// Copyright 2018-2020 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! A (work in progress) general purpose spherical/solid harmonics library in Rust.
//!
//! Documentation: [stable](https://doc.rs/sphrs/latest/sphrs),
//! [master](https://argmin-rs.github.io/sphrs/sphrs/).
//!
//! # Types of spherical/solid harmonics
//!
//! This crate supports these types of real SH via the enum `RealSHType`:
//!
//! * [Spherical](https://en.wikipedia.org/wiki/Spherical_harmonics)
//! * [RegularSolid and IrregularSolid](https://en.wikipedia.org/wiki/Solid_harmonics)
//!
//! TODO: complex SH
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! sphrs = "*"
//! ```
//!
//! # Example
//!
//! Compute the sum of all real SH up to 5th degree at position (1, 0, 0):
//!
//! ```rust
//! use sphrs::{RealSHType, RealHarmonics, Coordinates};
//! let degree = 5;
//! let sh: RealHarmonics<f64> = RealHarmonics::new(degree, RealSHType::Spherical);
//! let p = Coordinates::cartesian(1.0, 0.0, 0.0);
//! println!("SH up to degree {}: {:?}", degree, sh.eval(&p));
//! ```
//!
//! # Acknowledgements
//!
//! This crate is heavily inspired by Google's
//! [spherical-harmonics](https://github.com/google/spherical-harmonics) library and follows the
//! design documented
//! [here](https://pdfs.semanticscholar.org/83d9/28031e78f15d9813061b53d25a4e0274c751.pdf).
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
//!     ([LICENSE-APACHE](https://github.com/argmin-rs/argmin/blob/master/LICENSE-APACHE) or
//!     http://www.apache.org/licenses/LICENSE-2.0)
//!   * MIT License ([LICENSE-MIT](https://github.com/argmin-rs/argmin/blob/master/LICENSE-MIT) or
//!     http://opensource.org/licenses/MIT)
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

/// Coordinates
pub mod coords;
/// Spherical/solid harmonics
pub mod sh;

pub use crate::coords::*;
pub use crate::sh::*;
use num::{Complex, Float, FromPrimitive};
use num_traits::float::FloatConst;
use std::fmt::Debug;

/// Trait alias to simplify common trait bounds
pub trait SphrsFloat: Float + FloatConst + FromPrimitive + Debug {}
impl<I> SphrsFloat for I where I: Float + FloatConst + FromPrimitive + Debug {}

/// Available types of real spherical harmonics and solid harmonics
#[derive(Clone, Copy)]
pub enum RealSHType {
    /// Spherical harmonics
    Spherical,
    /// Regular solid harmonics
    RegularSolid,
    /// Irregular solid harmonics
    IrregularSolid,
}

/// Available types of complex spherical harmonics and solid harmonics
#[derive(Clone, Copy)]
pub enum ComplexSHType {
    /// Spherical harmonics
    Spherical,
    /// Regular solid harmonics
    RegularSolid,
    /// Irregular solid harmonics
    IrregularSolid,
}

/// SH eval trait (TODO)
pub trait SHEval<T: SphrsFloat, U> {
    /// Evaluate SH (l, m) at position `p`
    fn eval(&self, l: i64, m: i64, p: &dyn SHCoordinates<T>) -> U;
}

impl<T> SHEval<T, T> for RealSHType
where
    T: SphrsFloat,
{
    /// Evaluate real SH (l, m) at position `p`
    #[inline]
    fn eval(&self, l: i64, m: i64, p: &dyn SHCoordinates<T>) -> T {
        debug_assert!(m.abs() <= l);
        match self {
            Self::Spherical => real_SH_hardcoded(l, m, p),
            Self::RegularSolid => real_regular_solid_SH(l, m, p),
            Self::IrregularSolid => real_irregular_solid_SH(l, m, p),
        }
    }
}

impl<T> SHEval<T, Complex<T>> for ComplexSHType
where
    T: SphrsFloat,
{
    /// Evaluate complex SH (l, m) at position `p`
    #[inline]
    fn eval(&self, l: i64, m: i64, p: &dyn SHCoordinates<T>) -> Complex<T> {
        debug_assert!(m.abs() <= l);
        match self {
            Self::Spherical => SH(l, m, p),
            Self::RegularSolid => regular_solid_SH(l, m, p),
            Self::IrregularSolid => irregular_solid_SH(l, m, p),
        }
    }
}

/// Real spherical/solid harmonics
pub struct RealHarmonics<T, E> {
    /// degree
    degree: usize,
    /// Total number of harmonics
    num_sh: usize,
    /// Optional coefficients
    coefficients: Option<Vec<T>>,
    /// Type of harmonic
    sh: E,
}

impl<T, E> RealHarmonics<T, E>
where
    T: SphrsFloat,
    E: SHEval<T, T>,
{
    /// Create new `RealHarmonics` struct
    pub fn new(degree: usize, sh_type: E) -> RealHarmonics<T, E> {
        let num_sh = (0..=degree).map(|o| (2 * o + 1)).sum();

        RealHarmonics {
            degree,
            num_sh,
            coefficients: None,
            sh: sh_type,
        }
    }

    /// Add coefficients
    pub fn with_coefficients(&mut self, coefficients: Vec<T>) -> &mut Self {
        assert_eq!(coefficients.len(), self.num_sh);
        self.coefficients = Some(coefficients);
        self
    }

    /// Evaluate harmonics at postion `p`. This will respect coefficients if they are provided.
    #[inline]
    pub fn eval(&self, p: &dyn SHCoordinates<T>) -> Vec<T> {
        if let Some(ref coefficients) = self.coefficients {
            self.eval_internal(p)
                .iter()
                .zip(coefficients.iter())
                .map(|(&a, &b)| a * b)
                .collect()
        } else {
            self.eval_internal(p)
        }
    }

    /// Evaluate harmonics at postion `p`. If available, hardcoded SH functions will be used.
    #[inline]
    fn eval_internal(&self, p: &dyn SHCoordinates<T>) -> Vec<T> {
        let mut sh = Vec::with_capacity(self.num_sh);
        sh.push(self.sh.eval(0, 0, p));

        // The following may seem weird, but apparently it allows the compiler to better optimize
        // the code compared to a executing a loop. Performance improvement is about a facter of
        // two. I'd appreciate it if someone is able to write a macro for this.
        if self.degree >= 1 {
            sh.push(self.sh.eval(1, -1, p));
            sh.push(self.sh.eval(1, 0, p));
            sh.push(self.sh.eval(1, 1, p));
        }

        if self.degree >= 2 {
            sh.push(self.sh.eval(2, -2, p));
            sh.push(self.sh.eval(2, -1, p));
            sh.push(self.sh.eval(2, 0, p));
            sh.push(self.sh.eval(2, 1, p));
            sh.push(self.sh.eval(2, 2, p));
        }

        if self.degree >= 3 {
            sh.push(self.sh.eval(3, -3, p));
            sh.push(self.sh.eval(3, -2, p));
            sh.push(self.sh.eval(3, -1, p));
            sh.push(self.sh.eval(3, 0, p));
            sh.push(self.sh.eval(3, 1, p));
            sh.push(self.sh.eval(3, 2, p));
            sh.push(self.sh.eval(3, 3, p));
        }

        if self.degree >= 4 {
            sh.push(self.sh.eval(4, -4, p));
            sh.push(self.sh.eval(4, -3, p));
            sh.push(self.sh.eval(4, -2, p));
            sh.push(self.sh.eval(4, -1, p));
            sh.push(self.sh.eval(4, 0, p));
            sh.push(self.sh.eval(4, 1, p));
            sh.push(self.sh.eval(4, 2, p));
            sh.push(self.sh.eval(4, 3, p));
            sh.push(self.sh.eval(4, 4, p));
        }

        if self.degree >= 5 {
            sh.push(self.sh.eval(5, -5, p));
            sh.push(self.sh.eval(5, -4, p));
            sh.push(self.sh.eval(5, -3, p));
            sh.push(self.sh.eval(5, -2, p));
            sh.push(self.sh.eval(5, -1, p));
            sh.push(self.sh.eval(5, 0, p));
            sh.push(self.sh.eval(5, 1, p));
            sh.push(self.sh.eval(5, 2, p));
            sh.push(self.sh.eval(5, 3, p));
            sh.push(self.sh.eval(5, 4, p));
            sh.push(self.sh.eval(5, 5, p));
        }

        if self.degree >= 6 {
            sh.push(self.sh.eval(6, -6, p));
            sh.push(self.sh.eval(6, -4, p));
            sh.push(self.sh.eval(6, -3, p));
            sh.push(self.sh.eval(6, -2, p));
            sh.push(self.sh.eval(6, -1, p));
            sh.push(self.sh.eval(6, 0, p));
            sh.push(self.sh.eval(6, 1, p));
            sh.push(self.sh.eval(6, 2, p));
            sh.push(self.sh.eval(6, 3, p));
            sh.push(self.sh.eval(6, 4, p));
            sh.push(self.sh.eval(6, 5, p));
            sh.push(self.sh.eval(6, 6, p));
        }

        if self.degree >= 7 {
            sh.push(self.sh.eval(7, -7, p));
            sh.push(self.sh.eval(7, -6, p));
            sh.push(self.sh.eval(7, -4, p));
            sh.push(self.sh.eval(7, -3, p));
            sh.push(self.sh.eval(7, -2, p));
            sh.push(self.sh.eval(7, -1, p));
            sh.push(self.sh.eval(7, 0, p));
            sh.push(self.sh.eval(7, 1, p));
            sh.push(self.sh.eval(7, 2, p));
            sh.push(self.sh.eval(7, 3, p));
            sh.push(self.sh.eval(7, 4, p));
            sh.push(self.sh.eval(7, 5, p));
            sh.push(self.sh.eval(7, 6, p));
            sh.push(self.sh.eval(7, 7, p));
        }

        if self.degree >= 8 {
            sh.push(self.sh.eval(8, -8, p));
            sh.push(self.sh.eval(8, -7, p));
            sh.push(self.sh.eval(8, -6, p));
            sh.push(self.sh.eval(8, -4, p));
            sh.push(self.sh.eval(8, -3, p));
            sh.push(self.sh.eval(8, -2, p));
            sh.push(self.sh.eval(8, -1, p));
            sh.push(self.sh.eval(8, 0, p));
            sh.push(self.sh.eval(8, 1, p));
            sh.push(self.sh.eval(8, 2, p));
            sh.push(self.sh.eval(8, 3, p));
            sh.push(self.sh.eval(8, 4, p));
            sh.push(self.sh.eval(8, 5, p));
            sh.push(self.sh.eval(8, 6, p));
            sh.push(self.sh.eval(8, 7, p));
            sh.push(self.sh.eval(8, 8, p));
        }

        if self.degree >= 9 {
            sh.push(self.sh.eval(9, -9, p));
            sh.push(self.sh.eval(9, -8, p));
            sh.push(self.sh.eval(9, -7, p));
            sh.push(self.sh.eval(9, -6, p));
            sh.push(self.sh.eval(9, -4, p));
            sh.push(self.sh.eval(9, -3, p));
            sh.push(self.sh.eval(9, -2, p));
            sh.push(self.sh.eval(9, -1, p));
            sh.push(self.sh.eval(9, 0, p));
            sh.push(self.sh.eval(9, 1, p));
            sh.push(self.sh.eval(9, 2, p));
            sh.push(self.sh.eval(9, 3, p));
            sh.push(self.sh.eval(9, 4, p));
            sh.push(self.sh.eval(9, 5, p));
            sh.push(self.sh.eval(9, 6, p));
            sh.push(self.sh.eval(9, 7, p));
            sh.push(self.sh.eval(9, 8, p));
            sh.push(self.sh.eval(9, 9, p));
        }

        if self.degree >= 10 {
            sh.push(self.sh.eval(10, -10, p));
            sh.push(self.sh.eval(10, -9, p));
            sh.push(self.sh.eval(10, -8, p));
            sh.push(self.sh.eval(10, -7, p));
            sh.push(self.sh.eval(10, -6, p));
            sh.push(self.sh.eval(10, -4, p));
            sh.push(self.sh.eval(10, -3, p));
            sh.push(self.sh.eval(10, -2, p));
            sh.push(self.sh.eval(10, -1, p));
            sh.push(self.sh.eval(10, 0, p));
            sh.push(self.sh.eval(10, 1, p));
            sh.push(self.sh.eval(10, 2, p));
            sh.push(self.sh.eval(10, 3, p));
            sh.push(self.sh.eval(10, 4, p));
            sh.push(self.sh.eval(10, 5, p));
            sh.push(self.sh.eval(10, 6, p));
            sh.push(self.sh.eval(10, 7, p));
            sh.push(self.sh.eval(10, 8, p));
            sh.push(self.sh.eval(10, 9, p));
            sh.push(self.sh.eval(10, 10, p));
        }

        if self.degree >= 11 {
            sh.push(self.sh.eval(11, -11, p));
            sh.push(self.sh.eval(11, -10, p));
            sh.push(self.sh.eval(11, -9, p));
            sh.push(self.sh.eval(11, -8, p));
            sh.push(self.sh.eval(11, -7, p));
            sh.push(self.sh.eval(11, -6, p));
            sh.push(self.sh.eval(11, -4, p));
            sh.push(self.sh.eval(11, -3, p));
            sh.push(self.sh.eval(11, -2, p));
            sh.push(self.sh.eval(11, -1, p));
            sh.push(self.sh.eval(11, 0, p));
            sh.push(self.sh.eval(11, 1, p));
            sh.push(self.sh.eval(11, 2, p));
            sh.push(self.sh.eval(11, 3, p));
            sh.push(self.sh.eval(11, 4, p));
            sh.push(self.sh.eval(11, 5, p));
            sh.push(self.sh.eval(11, 6, p));
            sh.push(self.sh.eval(11, 7, p));
            sh.push(self.sh.eval(11, 8, p));
            sh.push(self.sh.eval(11, 9, p));
            sh.push(self.sh.eval(11, 10, p));
            sh.push(self.sh.eval(11, 11, p));
        }

        if self.degree >= 12 {
            sh.push(self.sh.eval(12, -12, p));
            sh.push(self.sh.eval(12, -11, p));
            sh.push(self.sh.eval(12, -10, p));
            sh.push(self.sh.eval(12, -9, p));
            sh.push(self.sh.eval(12, -8, p));
            sh.push(self.sh.eval(12, -7, p));
            sh.push(self.sh.eval(12, -6, p));
            sh.push(self.sh.eval(12, -4, p));
            sh.push(self.sh.eval(12, -3, p));
            sh.push(self.sh.eval(12, -2, p));
            sh.push(self.sh.eval(12, -1, p));
            sh.push(self.sh.eval(12, 0, p));
            sh.push(self.sh.eval(12, 1, p));
            sh.push(self.sh.eval(12, 2, p));
            sh.push(self.sh.eval(12, 3, p));
            sh.push(self.sh.eval(12, 4, p));
            sh.push(self.sh.eval(12, 5, p));
            sh.push(self.sh.eval(12, 6, p));
            sh.push(self.sh.eval(12, 7, p));
            sh.push(self.sh.eval(12, 8, p));
            sh.push(self.sh.eval(12, 9, p));
            sh.push(self.sh.eval(12, 10, p));
            sh.push(self.sh.eval(12, 11, p));
            sh.push(self.sh.eval(12, 12, p));
        }

        if self.degree >= 13 {
            sh.push(self.sh.eval(13, -13, p));
            sh.push(self.sh.eval(13, -12, p));
            sh.push(self.sh.eval(13, -11, p));
            sh.push(self.sh.eval(13, -10, p));
            sh.push(self.sh.eval(13, -9, p));
            sh.push(self.sh.eval(13, -8, p));
            sh.push(self.sh.eval(13, -7, p));
            sh.push(self.sh.eval(13, -6, p));
            sh.push(self.sh.eval(13, -4, p));
            sh.push(self.sh.eval(13, -3, p));
            sh.push(self.sh.eval(13, -2, p));
            sh.push(self.sh.eval(13, -1, p));
            sh.push(self.sh.eval(13, 0, p));
            sh.push(self.sh.eval(13, 1, p));
            sh.push(self.sh.eval(13, 2, p));
            sh.push(self.sh.eval(13, 3, p));
            sh.push(self.sh.eval(13, 4, p));
            sh.push(self.sh.eval(13, 5, p));
            sh.push(self.sh.eval(13, 6, p));
            sh.push(self.sh.eval(13, 7, p));
            sh.push(self.sh.eval(13, 8, p));
            sh.push(self.sh.eval(13, 9, p));
            sh.push(self.sh.eval(13, 10, p));
            sh.push(self.sh.eval(13, 11, p));
            sh.push(self.sh.eval(13, 12, p));
            sh.push(self.sh.eval(13, 13, p));
        }

        if self.degree >= 14 {
            sh.push(self.sh.eval(14, -14, p));
            sh.push(self.sh.eval(14, -13, p));
            sh.push(self.sh.eval(14, -12, p));
            sh.push(self.sh.eval(14, -11, p));
            sh.push(self.sh.eval(14, -10, p));
            sh.push(self.sh.eval(14, -9, p));
            sh.push(self.sh.eval(14, -8, p));
            sh.push(self.sh.eval(14, -7, p));
            sh.push(self.sh.eval(14, -6, p));
            sh.push(self.sh.eval(14, -4, p));
            sh.push(self.sh.eval(14, -3, p));
            sh.push(self.sh.eval(14, -2, p));
            sh.push(self.sh.eval(14, -1, p));
            sh.push(self.sh.eval(14, 0, p));
            sh.push(self.sh.eval(14, 1, p));
            sh.push(self.sh.eval(14, 2, p));
            sh.push(self.sh.eval(14, 3, p));
            sh.push(self.sh.eval(14, 4, p));
            sh.push(self.sh.eval(14, 5, p));
            sh.push(self.sh.eval(14, 6, p));
            sh.push(self.sh.eval(14, 7, p));
            sh.push(self.sh.eval(14, 8, p));
            sh.push(self.sh.eval(14, 9, p));
            sh.push(self.sh.eval(14, 10, p));
            sh.push(self.sh.eval(14, 11, p));
            sh.push(self.sh.eval(14, 12, p));
            sh.push(self.sh.eval(14, 13, p));
            sh.push(self.sh.eval(14, 14, p));
        }

        if self.degree >= 15 {
            sh.push(self.sh.eval(15, -15, p));
            sh.push(self.sh.eval(15, -14, p));
            sh.push(self.sh.eval(15, -13, p));
            sh.push(self.sh.eval(15, -12, p));
            sh.push(self.sh.eval(15, -11, p));
            sh.push(self.sh.eval(15, -10, p));
            sh.push(self.sh.eval(15, -9, p));
            sh.push(self.sh.eval(15, -8, p));
            sh.push(self.sh.eval(15, -7, p));
            sh.push(self.sh.eval(15, -6, p));
            sh.push(self.sh.eval(15, -4, p));
            sh.push(self.sh.eval(15, -3, p));
            sh.push(self.sh.eval(15, -2, p));
            sh.push(self.sh.eval(15, -1, p));
            sh.push(self.sh.eval(15, 0, p));
            sh.push(self.sh.eval(15, 1, p));
            sh.push(self.sh.eval(15, 2, p));
            sh.push(self.sh.eval(15, 3, p));
            sh.push(self.sh.eval(15, 4, p));
            sh.push(self.sh.eval(15, 5, p));
            sh.push(self.sh.eval(15, 6, p));
            sh.push(self.sh.eval(15, 7, p));
            sh.push(self.sh.eval(15, 8, p));
            sh.push(self.sh.eval(15, 9, p));
            sh.push(self.sh.eval(15, 10, p));
            sh.push(self.sh.eval(15, 11, p));
            sh.push(self.sh.eval(15, 12, p));
            sh.push(self.sh.eval(15, 13, p));
            sh.push(self.sh.eval(15, 14, p));
            sh.push(self.sh.eval(15, 15, p));
        }

        if self.degree >= 16 {
            sh.push(self.sh.eval(16, -16, p));
            sh.push(self.sh.eval(16, -15, p));
            sh.push(self.sh.eval(16, -14, p));
            sh.push(self.sh.eval(16, -13, p));
            sh.push(self.sh.eval(16, -12, p));
            sh.push(self.sh.eval(16, -11, p));
            sh.push(self.sh.eval(16, -10, p));
            sh.push(self.sh.eval(16, -9, p));
            sh.push(self.sh.eval(16, -8, p));
            sh.push(self.sh.eval(16, -7, p));
            sh.push(self.sh.eval(16, -6, p));
            sh.push(self.sh.eval(16, -4, p));
            sh.push(self.sh.eval(16, -3, p));
            sh.push(self.sh.eval(16, -2, p));
            sh.push(self.sh.eval(16, -1, p));
            sh.push(self.sh.eval(16, 0, p));
            sh.push(self.sh.eval(16, 1, p));
            sh.push(self.sh.eval(16, 2, p));
            sh.push(self.sh.eval(16, 3, p));
            sh.push(self.sh.eval(16, 4, p));
            sh.push(self.sh.eval(16, 5, p));
            sh.push(self.sh.eval(16, 6, p));
            sh.push(self.sh.eval(16, 7, p));
            sh.push(self.sh.eval(16, 8, p));
            sh.push(self.sh.eval(16, 9, p));
            sh.push(self.sh.eval(16, 10, p));
            sh.push(self.sh.eval(16, 11, p));
            sh.push(self.sh.eval(16, 12, p));
            sh.push(self.sh.eval(16, 13, p));
            sh.push(self.sh.eval(16, 14, p));
            sh.push(self.sh.eval(16, 15, p));
            sh.push(self.sh.eval(16, 16, p));
        }

        if self.degree >= 17 {
            sh.push(self.sh.eval(17, -17, p));
            sh.push(self.sh.eval(17, -16, p));
            sh.push(self.sh.eval(17, -15, p));
            sh.push(self.sh.eval(17, -14, p));
            sh.push(self.sh.eval(17, -13, p));
            sh.push(self.sh.eval(17, -12, p));
            sh.push(self.sh.eval(17, -11, p));
            sh.push(self.sh.eval(17, -10, p));
            sh.push(self.sh.eval(17, -9, p));
            sh.push(self.sh.eval(17, -8, p));
            sh.push(self.sh.eval(17, -7, p));
            sh.push(self.sh.eval(17, -6, p));
            sh.push(self.sh.eval(17, -4, p));
            sh.push(self.sh.eval(17, -3, p));
            sh.push(self.sh.eval(17, -2, p));
            sh.push(self.sh.eval(17, -1, p));
            sh.push(self.sh.eval(17, 0, p));
            sh.push(self.sh.eval(17, 1, p));
            sh.push(self.sh.eval(17, 2, p));
            sh.push(self.sh.eval(17, 3, p));
            sh.push(self.sh.eval(17, 4, p));
            sh.push(self.sh.eval(17, 5, p));
            sh.push(self.sh.eval(17, 6, p));
            sh.push(self.sh.eval(17, 7, p));
            sh.push(self.sh.eval(17, 8, p));
            sh.push(self.sh.eval(17, 9, p));
            sh.push(self.sh.eval(17, 10, p));
            sh.push(self.sh.eval(17, 11, p));
            sh.push(self.sh.eval(17, 12, p));
            sh.push(self.sh.eval(17, 13, p));
            sh.push(self.sh.eval(17, 14, p));
            sh.push(self.sh.eval(17, 15, p));
            sh.push(self.sh.eval(17, 16, p));
            sh.push(self.sh.eval(17, 17, p));
        }

        if self.degree >= 18 {
            sh.push(self.sh.eval(18, -18, p));
            sh.push(self.sh.eval(18, -17, p));
            sh.push(self.sh.eval(18, -16, p));
            sh.push(self.sh.eval(18, -15, p));
            sh.push(self.sh.eval(18, -14, p));
            sh.push(self.sh.eval(18, -13, p));
            sh.push(self.sh.eval(18, -12, p));
            sh.push(self.sh.eval(18, -11, p));
            sh.push(self.sh.eval(18, -10, p));
            sh.push(self.sh.eval(18, -9, p));
            sh.push(self.sh.eval(18, -8, p));
            sh.push(self.sh.eval(18, -7, p));
            sh.push(self.sh.eval(18, -6, p));
            sh.push(self.sh.eval(18, -4, p));
            sh.push(self.sh.eval(18, -3, p));
            sh.push(self.sh.eval(18, -2, p));
            sh.push(self.sh.eval(18, -1, p));
            sh.push(self.sh.eval(18, 0, p));
            sh.push(self.sh.eval(18, 1, p));
            sh.push(self.sh.eval(18, 2, p));
            sh.push(self.sh.eval(18, 3, p));
            sh.push(self.sh.eval(18, 4, p));
            sh.push(self.sh.eval(18, 5, p));
            sh.push(self.sh.eval(18, 6, p));
            sh.push(self.sh.eval(18, 7, p));
            sh.push(self.sh.eval(18, 8, p));
            sh.push(self.sh.eval(18, 9, p));
            sh.push(self.sh.eval(18, 10, p));
            sh.push(self.sh.eval(18, 11, p));
            sh.push(self.sh.eval(18, 12, p));
            sh.push(self.sh.eval(18, 13, p));
            sh.push(self.sh.eval(18, 14, p));
            sh.push(self.sh.eval(18, 15, p));
            sh.push(self.sh.eval(18, 16, p));
            sh.push(self.sh.eval(18, 17, p));
            sh.push(self.sh.eval(18, 18, p));
        }

        if self.degree >= 19 {
            sh.push(self.sh.eval(19, -19, p));
            sh.push(self.sh.eval(19, -18, p));
            sh.push(self.sh.eval(19, -17, p));
            sh.push(self.sh.eval(19, -16, p));
            sh.push(self.sh.eval(19, -15, p));
            sh.push(self.sh.eval(19, -14, p));
            sh.push(self.sh.eval(19, -13, p));
            sh.push(self.sh.eval(19, -12, p));
            sh.push(self.sh.eval(19, -11, p));
            sh.push(self.sh.eval(19, -10, p));
            sh.push(self.sh.eval(19, -9, p));
            sh.push(self.sh.eval(19, -8, p));
            sh.push(self.sh.eval(19, -7, p));
            sh.push(self.sh.eval(19, -6, p));
            sh.push(self.sh.eval(19, -4, p));
            sh.push(self.sh.eval(19, -3, p));
            sh.push(self.sh.eval(19, -2, p));
            sh.push(self.sh.eval(19, -1, p));
            sh.push(self.sh.eval(19, 0, p));
            sh.push(self.sh.eval(19, 1, p));
            sh.push(self.sh.eval(19, 2, p));
            sh.push(self.sh.eval(19, 3, p));
            sh.push(self.sh.eval(19, 4, p));
            sh.push(self.sh.eval(19, 5, p));
            sh.push(self.sh.eval(19, 6, p));
            sh.push(self.sh.eval(19, 7, p));
            sh.push(self.sh.eval(19, 8, p));
            sh.push(self.sh.eval(19, 9, p));
            sh.push(self.sh.eval(19, 10, p));
            sh.push(self.sh.eval(19, 11, p));
            sh.push(self.sh.eval(19, 12, p));
            sh.push(self.sh.eval(19, 13, p));
            sh.push(self.sh.eval(19, 14, p));
            sh.push(self.sh.eval(19, 15, p));
            sh.push(self.sh.eval(19, 16, p));
            sh.push(self.sh.eval(19, 17, p));
            sh.push(self.sh.eval(19, 18, p));
            sh.push(self.sh.eval(19, 19, p));
        }

        if self.degree >= 20 {
            sh.push(self.sh.eval(20, -20, p));
            sh.push(self.sh.eval(20, -19, p));
            sh.push(self.sh.eval(20, -18, p));
            sh.push(self.sh.eval(20, -17, p));
            sh.push(self.sh.eval(20, -16, p));
            sh.push(self.sh.eval(20, -15, p));
            sh.push(self.sh.eval(20, -14, p));
            sh.push(self.sh.eval(20, -13, p));
            sh.push(self.sh.eval(20, -12, p));
            sh.push(self.sh.eval(20, -11, p));
            sh.push(self.sh.eval(20, -10, p));
            sh.push(self.sh.eval(20, -9, p));
            sh.push(self.sh.eval(20, -8, p));
            sh.push(self.sh.eval(20, -7, p));
            sh.push(self.sh.eval(20, -6, p));
            sh.push(self.sh.eval(20, -4, p));
            sh.push(self.sh.eval(20, -3, p));
            sh.push(self.sh.eval(20, -2, p));
            sh.push(self.sh.eval(20, -1, p));
            sh.push(self.sh.eval(20, 0, p));
            sh.push(self.sh.eval(20, 1, p));
            sh.push(self.sh.eval(20, 2, p));
            sh.push(self.sh.eval(20, 3, p));
            sh.push(self.sh.eval(20, 4, p));
            sh.push(self.sh.eval(20, 5, p));
            sh.push(self.sh.eval(20, 6, p));
            sh.push(self.sh.eval(20, 7, p));
            sh.push(self.sh.eval(20, 8, p));
            sh.push(self.sh.eval(20, 9, p));
            sh.push(self.sh.eval(20, 10, p));
            sh.push(self.sh.eval(20, 11, p));
            sh.push(self.sh.eval(20, 12, p));
            sh.push(self.sh.eval(20, 13, p));
            sh.push(self.sh.eval(20, 14, p));
            sh.push(self.sh.eval(20, 15, p));
            sh.push(self.sh.eval(20, 16, p));
            sh.push(self.sh.eval(20, 17, p));
            sh.push(self.sh.eval(20, 18, p));
            sh.push(self.sh.eval(20, 19, p));
            sh.push(self.sh.eval(20, 20, p));
        }
        for l in 21..=self.degree {
            let l = l as i64;
            for m in -l..=l {
                sh.push(self.sh.eval(l, m, p));
            }
        }

        sh
    }
}

/// Complex spherical/solid harmonics
pub struct ComplexHarmonics<T> {
    /// degree
    degree: usize,
    /// Total number of harmonics
    num_sh: usize,
    /// Optional coefficients
    coefficients: Option<Vec<Complex<T>>>,
    /// Type of harmonic
    sh: ComplexSHType,
}

impl<T> ComplexHarmonics<T>
where
    T: SphrsFloat,
{
    /// Create new `ComplexHarmonics` struct
    pub fn new(degree: usize, sh_type: ComplexSHType) -> ComplexHarmonics<T> {
        let num_sh = (0..=degree).map(|o| (2 * o + 1)).sum();

        ComplexHarmonics {
            degree,
            num_sh,
            coefficients: None,
            sh: sh_type,
        }
    }

    /// Add coefficients
    pub fn with_coefficients(&mut self, coefficients: Vec<Complex<T>>) -> &mut Self {
        assert_eq!(coefficients.len(), self.num_sh);
        self.coefficients = Some(coefficients);
        self
    }

    /// Evaluate harmonics at postion `p`. This will respect coefficients if they are provided.
    #[inline]
    pub fn eval(&self, p: &dyn SHCoordinates<T>) -> Vec<Complex<T>> {
        if let Some(ref coefficients) = self.coefficients {
            self.eval_internal(p)
                .iter()
                .zip(coefficients.iter())
                .map(|(&a, &b)| a * b)
                .collect()
        } else {
            self.eval_internal(p)
        }
    }

    /// Evaluate harmonics at postion `p`.
    #[inline]
    fn eval_internal(&self, p: &dyn SHCoordinates<T>) -> Vec<Complex<T>> {
        let mut sh = Vec::with_capacity(self.num_sh);
        for l in 0..=self.degree {
            let l = l as i64;
            for m in -l..=l {
                sh.push(self.sh.eval(l, m, p));
            }
        }

        sh
    }
}
