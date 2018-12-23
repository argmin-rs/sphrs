// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! todo

pub mod coords;
pub mod sh;

pub use crate::coords::*;
pub use crate::sh::*;
use num::{Float, FromPrimitive};
use num_traits::float::FloatConst;
// use num_complex::Complex;

pub struct SphericalHarmonics<T>
where
    T: Float + FromPrimitive + FloatConst,
{
    order: usize,
    num_sh: usize,
    coeffs: Vec<T>,
    // sh: Vec<fn(&Coordinates<T>) -> T>,
}

impl<T> SphericalHarmonics<T>
where
    T: Float + FromPrimitive + FloatConst,
{
    pub fn eval_vec(&self, p: Vec<&Coordinates<T>>) -> Vec<T> {
        p.iter().map(|&pi| self.eval(pi)).collect()
    }

    pub fn eval(&self, p: &Coordinates<T>) -> T {
        let mut res = self.coeffs[0] * sh00(p);

        if self.order >= 1 {
            res = res
                + self.coeffs[1] * sh1n1(p)
                + self.coeffs[2] * sh10(p)
                + self.coeffs[3] * sh1p1(p);
        }

        if self.order >= 2 {
            res = res
                + self.coeffs[4] * sh2n2(p)
                + self.coeffs[5] * sh2n1(p)
                + self.coeffs[6] * sh20(p)
                + self.coeffs[7] * sh2p1(p)
                + self.coeffs[8] * sh2p2(p);
        }

        if self.order >= 3 {
            res = res
                + self.coeffs[9] * sh3n3(p)
                + self.coeffs[10] * sh3n2(p)
                + self.coeffs[11] * sh3n1(p)
                + self.coeffs[12] * sh30(p)
                + self.coeffs[13] * sh3p1(p)
                + self.coeffs[14] * sh3p2(p)
                + self.coeffs[15] * sh3p3(p);
        }

        let mut j = 16;
        if self.order >= 4 {
            for l in 4..=self.order {
                let l = l as i64;
                for m in (-l)..=l {
                    let m = m as i64;
                    res = res + self.coeffs[j] * real_SH(m, l, p);
                    j += 1;
                }
            }
        }

        res
    }
    pub fn eval_indiv(&self, p: &Coordinates<T>) -> Vec<T> {
        let mut sh = Vec::with_capacity(self.num_sh);

        sh.push(self.coeffs[0] * sh00(p));

        if self.order >= 1 {
            sh.push(self.coeffs[1] * sh1n1(p));
            sh.push(self.coeffs[2] * sh10(p));
            sh.push(self.coeffs[3] * sh1p1(p));
        }

        if self.order >= 2 {
            sh.push(self.coeffs[4] * sh2n2(p));
            sh.push(self.coeffs[5] * sh2n1(p));
            sh.push(self.coeffs[6] * sh20(p));
            sh.push(self.coeffs[7] * sh2p1(p));
            sh.push(self.coeffs[8] * sh2p2(p));
        }

        if self.order >= 3 {
            sh.push(self.coeffs[9] * sh3n3(p));
            sh.push(self.coeffs[10] * sh3n2(p));
            sh.push(self.coeffs[11] * sh3n1(p));
            sh.push(self.coeffs[12] * sh30(p));
            sh.push(self.coeffs[13] * sh3p1(p));
            sh.push(self.coeffs[14] * sh3p2(p));
            sh.push(self.coeffs[15] * sh3p3(p));
        }

        let mut j = 16;
        if self.order >= 4 {
            for l in 4..=self.order {
                let l = l as i64;
                for m in (-l)..=l {
                    let m = m as i64;
                    sh.push(self.coeffs[j] * real_SH(m, l, p));
                    j += 1;
                }
            }
        }

        sh
    }

    pub fn new(order: usize) -> Self {
        let n = (0..=order).map(|o| (o + 1) * (o + 2) / 2).sum();
        SphericalHarmonics {
            order,
            num_sh: n,
            coeffs: vec![T::one(); n],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn it_works() {
        let p = GenCoordinates::spherical(1.0, PI / 2.0, 0.0);
        let v = sh10(&p);
        let bla: SphericalHarmonics<f64> = SphericalHarmonics::new(3);
        // println!("p: {:?} | v: {}", p, v);
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn comp() {
        let p = GenCoordinates::spherical(1.0, PI / 2.0, 0.0);
        // let p = GenCoordinates::cartesian(1.0, 1.0, 0.3);
        assert!((real_SH(2, 1, &p) - sh2p1(&p)) < std::f64::EPSILON);
        assert!((real_SH(3, -2, &p) - sh3n2(&p)) < std::f64::EPSILON);
    }
}
