// Copyright 2018-2020 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::{
    real_irregular_solid_sh, real_regular_solid_sh, real_sh_hardcoded, SHCoordinates, SHEval,
    SphrsFloat,
};

/// Available types of real spherical harmonics and solid harmonics
#[derive(Clone, Copy)]
pub enum RealSH {
    /// Spherical harmonics
    Spherical,
    /// Regular solid harmonics
    RegularSolid,
    /// Irregular solid harmonics
    IrregularSolid,
}

impl<T> SHEval<T> for RealSH
where
    T: SphrsFloat,
{
    type Output = T;

    /// Evaluate real SH (l, m) at position `p`
    #[inline(always)]
    fn eval(&self, l: i64, m: i64, p: &impl SHCoordinates<T>) -> Self::Output {
        assert!(m.abs() <= l);
        match self {
            Self::Spherical => real_sh_hardcoded(l, m, p),
            Self::RegularSolid => real_regular_solid_sh(l, m, p),
            Self::IrregularSolid => real_irregular_solid_sh(l, m, p),
        }
    }
}
