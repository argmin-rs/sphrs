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
    pub fn eval(&self, p: &Coordinates<T>) -> Vec<T> {
        let mut sh = Vec::with_capacity(self.num_sh);

        sh.push(sh00(p));

        if self.order >= 1 {
            sh.push(sh1n1(p));
            sh.push(sh10(p));
            sh.push(sh1p1(p));
        }

        if self.order >= 2 {
            sh.push(sh2n2(p));
            sh.push(sh2n1(p));
            sh.push(sh20(p));
            sh.push(sh2p1(p));
            sh.push(sh2p2(p));
        }

        if self.order >= 3 {
            sh.push(sh3n3(p));
            sh.push(sh3n2(p));
            sh.push(sh3n1(p));
            sh.push(sh30(p));
            sh.push(sh3p1(p));
            sh.push(sh3p2(p));
            sh.push(sh3p3(p));
        }

        if self.order >= 4 {
            for l in 4..=self.order {
                let l = l as i64;
                for m in (-l)..=l {
                    let m = m as i64;
                    sh.push(real_SH(m, l, p));
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
