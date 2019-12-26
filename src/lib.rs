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
//! Compute the sum of all real SH up to 5th order at position (1, 0, 0):
//!
//! ```rust
//! use sphrs::{RealSHType, RealHarmonics, Coordinates};
//! let order = 5;
//! let sh = RealHarmonics::new(order, RealSHType::Spherical);
//! let p = Coordinates::cartesian(1.0, 0.0, 0.0);
//! println!("SH up to order {}: {:?}", order, sh.eval(&p));
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

/// Coordi
pub mod coords;
/// Spherical/solid harmonics
pub mod sh;

pub use crate::coords::*;
pub use crate::sh::*;
use num::{Complex, Float, FromPrimitive};
use num_traits::float::FloatConst;
use std::fmt::Debug;
use std::ops::AddAssign;

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

impl RealSHType {
    /// Evaluate real SH (l, m) at position `p`
    #[inline]
    pub fn eval<T>(self, l: i64, m: i64, p: &dyn SHCoordinates<T>) -> T
    where
        T: SphrsFloat + AddAssign + Debug,
    {
        assert!(m.abs() <= l);
        match self {
            RealSHType::Spherical => real_SH_hardcoded(l, m, p),
            RealSHType::RegularSolid => real_regular_solid_SH(l, m, p),
            RealSHType::IrregularSolid => real_irregular_solid_SH(l, m, p),
        }
    }
}

impl ComplexSHType {
    /// Evaluate complex SH (l, m) at position `p`
    #[inline]
    pub fn eval<T>(self, l: i64, m: i64, p: &dyn SHCoordinates<T>) -> Complex<T>
    where
        T: SphrsFloat + AddAssign + Debug,
    {
        assert!(m.abs() <= l);
        match self {
            ComplexSHType::Spherical => SH(l, m, p),
            ComplexSHType::RegularSolid => regular_solid_SH(l, m, p),
            ComplexSHType::IrregularSolid => irregular_solid_SH(l, m, p),
        }
    }
}

/// Real spherical/solid harmonics
pub struct RealHarmonics<T>
where
    T: SphrsFloat + AddAssign + std::iter::Sum + Debug,
{
    /// Order
    order: usize,
    /// Total number of harmonics
    num_sh: usize,
    /// Optional coefficients
    coefficients: Option<Vec<T>>,
    /// Type of harmonic
    sh: RealSHType,
}

impl<'a, T> RealHarmonics<T>
where
    T: SphrsFloat + AddAssign + std::iter::Sum + Debug,
{
    /// Create new `RealHarmonics` struct
    pub fn new(order: usize, sh_type: RealSHType) -> RealHarmonics<T> {
        let n = (0..=order).map(|o| (2 * o + 1)).sum();

        RealHarmonics {
            order,
            num_sh: n,
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

        // TODO: Check if this is necessary. I think my hope was that explicitly stating m and l
        // will allow the compiler to evaluate the code partially. I'll have to check if this is
        // the case (I doubt it).
        if self.order >= 1 {
            sh.push(self.sh.eval(1, -1, p));
            sh.push(self.sh.eval(1, 0, p));
            sh.push(self.sh.eval(1, 1, p));
        }

        if self.order >= 2 {
            sh.push(self.sh.eval(2, -2, p));
            sh.push(self.sh.eval(2, -1, p));
            sh.push(self.sh.eval(2, 0, p));
            sh.push(self.sh.eval(2, 1, p));
            sh.push(self.sh.eval(2, 2, p));
        }

        if self.order >= 3 {
            sh.push(self.sh.eval(3, -3, p));
            sh.push(self.sh.eval(3, -2, p));
            sh.push(self.sh.eval(3, -1, p));
            sh.push(self.sh.eval(3, 0, p));
            sh.push(self.sh.eval(3, 1, p));
            sh.push(self.sh.eval(3, 2, p));
            sh.push(self.sh.eval(3, 3, p));
        }

        if self.order >= 4 {
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

        if self.order >= 5 {
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

        for l in 6..=self.order {
            let l = l as i64;
            for m in -l..=l {
                sh.push(self.sh.eval(l, m, p));
            }
        }

        sh
    }
}

/// Complex spherical/solid harmonics
pub struct ComplexHarmonics<T>
where
    T: SphrsFloat + AddAssign + std::iter::Sum + Debug,
{
    /// Order
    order: usize,
    /// Total number of harmonics
    num_sh: usize,
    /// Optional coefficients
    coefficients: Option<Vec<Complex<T>>>,
    /// Type of harmonic
    sh: ComplexSHType,
}

impl<'a, T> ComplexHarmonics<T>
where
    T: SphrsFloat + AddAssign + std::iter::Sum + Debug,
{
    /// Create new `ComplexHarmonics` struct
    pub fn new(order: usize, sh_type: RealSHType) -> RealHarmonics<T> {
        let n = (0..=order).map(|o| (2 * o + 1)).sum();

        RealHarmonics {
            order,
            num_sh: n,
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
        for l in 0..=self.order {
            let l = l as i64;
            for m in -l..=l {
                sh.push(self.sh.eval(l, m, p));
            }
        }

        sh
    }
}
