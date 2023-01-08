// Copyright 2018-2023 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use num::Complex;

use crate::{irregular_solid_sh, regular_solid_sh, sh, SHCoordinates, SHEval, SphrsFloat};

/// Available types of complex spherical harmonics and solid harmonics
#[derive(Clone, Copy)]
pub enum ComplexSH {
    /// Spherical harmonics
    Spherical,
    /// Regular solid harmonics
    RegularSolid,
    /// Irregular solid harmonics
    IrregularSolid,
}

impl<T> SHEval<T> for ComplexSH
where
    T: SphrsFloat,
{
    type Output = Complex<T>;

    /// Evaluate complex SH (l, m) at position `p`
    #[inline(always)]
    fn eval(&self, l: i64, m: i64, p: &impl SHCoordinates<T>) -> Complex<T> {
        assert!(m.abs() <= l);
        match self {
            Self::Spherical => sh(l, m, p),
            Self::RegularSolid => regular_solid_sh(l, m, p),
            Self::IrregularSolid => irregular_solid_sh(l, m, p),
        }
    }
}
