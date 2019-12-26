// Copyright 2018-2020 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! A general purpose spherical harmonics library in Rust.
//!
//! # Types of SH
//!
//! This crate supports these types of real SH via the enum `RealSHType`:
//!
//! * [Standard](https://en.wikipedia.org/wiki/Spherical_harmonics)
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
//! let sh_type = RealSHType::Standard;
//! let order = 5;
//! let sh = RealSphericalHarmonics::new(order, sh_type);
//! let p = Coordinates::cartesian(1.0, 0.0, 0.0);
//! println!("Sum of SH up to order {}: {}", order, sh.eval(&p));
//! ```
//!
//! # Acknowledgements
//!
//! This crate is heavily inspired by Google's
//! [spherical-harmonics](https://github.com/google/spherical-harmonics) library and follows the
//! design documented
//! [here](https://pdfs.semanticscholar.org/83d9/28031e78f15d9813061b53d25a4e0274c751.pdf).
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

pub mod coords;
pub mod sh;

pub use crate::coords::*;
pub use crate::sh::*;
use num::{Float, FromPrimitive};
use num_traits::float::FloatConst;
use std::fmt::Debug;
use std::ops::AddAssign;

pub trait SphrsFloat: Float + FloatConst + FromPrimitive + Debug {}
impl<I> SphrsFloat for I where I: Float + FloatConst + FromPrimitive + Debug {}

#[derive(Clone, Copy)]
pub enum RealSHType {
    Standard,
    RegularSolid,
    IrregularSolid,
}

impl RealSHType {
    #[inline]
    pub fn eval<T>(self, l: i64, m: i64, p: &dyn SHCoordinates<T>) -> T
    where
        T: SphrsFloat + AddAssign + Debug,
    {
        assert!(m.abs() <= l);
        match self {
            RealSHType::Standard => real_SH_hardcoded(l, m, p),
            RealSHType::RegularSolid => real_regular_solid_SH(l, m, p),
            RealSHType::IrregularSolid => real_irregular_solid_SH(l, m, p),
        }
    }
}

pub struct RealSphericalHarmonics<T>
where
    T: SphrsFloat + AddAssign + std::iter::Sum + Debug,
{
    order: usize,
    num_sh: usize,
    coefficients: Option<Vec<T>>,
    sh: RealSHType,
}

impl<'a, T> RealSphericalHarmonics<T>
where
    T: SphrsFloat + AddAssign + std::iter::Sum + Debug,
{
    pub fn new(order: usize, sh_type: RealSHType) -> RealSphericalHarmonics<T> {
        let n = (0..=order).map(|o| (2 * o + 1)).sum();

        RealSphericalHarmonics {
            order,
            num_sh: n,
            coefficients: None,
            sh: sh_type,
        }
    }

    pub fn with_coefficients(&mut self, coefficients: Vec<T>) -> &mut Self {
        assert_eq!(coefficients.len(), self.num_sh);
        self.coefficients = Some(coefficients);
        self
    }

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

    #[inline]
    fn eval_internal(&self, p: &dyn SHCoordinates<T>) -> Vec<T> {
        let mut sh = Vec::with_capacity(self.num_sh);
        sh.push(self.sh.eval(0, 0, p));

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
