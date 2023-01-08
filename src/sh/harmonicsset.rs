// Copyright 2018-2023 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::marker::PhantomData;

use crate::{SHCoordinates, SHEval, SphrsFloat};

/// A set of spherical/solid harmonics up to a given degree
pub struct HarmonicsSet<T, E> {
    /// degree
    degree: usize,
    /// Total number of harmonics
    num_sh: usize,
    /// Type of harmonic
    sh: E,
    /// Float
    _ttt: PhantomData<T>,
}

impl<T, E> HarmonicsSet<T, E>
where
    T: SphrsFloat,
    E: SHEval<T>,
    E::Output: std::ops::Mul + Copy,
    Vec<E::Output>: std::iter::FromIterator<<E::Output as std::ops::Mul>::Output>,
{
    /// Create new `HarmonicsSet` struct
    pub fn new(degree: usize, sh_type: E) -> HarmonicsSet<T, E> {
        let num_sh = (0..=degree).map(|o| (2 * o + 1)).sum();

        HarmonicsSet {
            degree,
            num_sh,
            sh: sh_type,
            _ttt: PhantomData,
        }
    }

    /// Evaluate harmonics at position `p` without coefficients.
    pub fn eval<C>(&self, p: &C) -> Vec<E::Output>
    where
        C: SHCoordinates<T>,
    {
        self.eval_internal(p)
    }

    /// Evaluate harmonics at position `p` with a given vector of coefficients.
    pub fn eval_with_coefficients<C>(&self, p: &C, coefficients: Vec<E::Output>) -> Vec<E::Output>
    where
        C: SHCoordinates<T>,
    {
        assert_eq!(coefficients.len(), self.num_sh);
        self.eval_internal(p)
            .into_iter()
            .zip(coefficients.iter())
            .map(|(a, &b)| a * b)
            .collect()
    }

    /// Evaluate harmonics at position `p`. If available, hardcoded SH functions will be used.
    #[inline]
    fn eval_internal<C>(&self, p: &C) -> Vec<E::Output>
    where
        C: SHCoordinates<T>,
    {
        let mut sh = Vec::with_capacity(self.num_sh);
        sh.push(self.sh.eval(0, 0, p));

        // The following may seem weird, but apparently it allows the compiler to better optimize
        // the code compared to a executing a loop. Performance improvement is about a facter of
        // two. Would be great if there was a macro for this.
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
