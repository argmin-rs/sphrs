// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! todo

extern crate openblas_src;

pub mod coords;
pub mod sh;

pub use crate::coords::*;
pub use crate::sh::*;
use ndarray::{s, Array1, Array2};
use num::{Float, FromPrimitive};
// use num_complex::Complex;
use num_traits::float::FloatConst;
use std::fmt::Debug;
use std::ops::AddAssign;

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
        T: Float + FromPrimitive + FloatConst + AddAssign + Debug,
    {
        match self {
            RealSHType::Standard => real_SH_hc(l, m, p),
            RealSHType::RegularSolid => real_regular_solid_SH(l, m, p),
            RealSHType::IrregularSolid => real_irregular_solid_SH(l, m, p),
        }
    }
}

pub struct RealSphericalHarmonics<T>
where
    T: Float + FromPrimitive + FloatConst + AddAssign + std::iter::Sum + Debug,
{
    order: usize,
    num_sh: usize,
    coeffs: Vec<T>,
    sh: RealSHType,
}

impl<'a, T> RealSphericalHarmonics<T>
where
    T: Float + FromPrimitive + FloatConst + AddAssign + std::iter::Sum + Debug,
{
    #[inline]
    pub fn eval_vec(&self, p: &[impl SHCoordinates<T>]) -> Vec<T> {
        p.iter().map(|pi| self.eval(pi)).collect()
    }

    #[inline]
    pub fn eval(&self, p: &dyn SHCoordinates<T>) -> T {
        self.eval_indiv(p).into_iter().sum()
    }

    #[inline]
    pub fn eval_plain(&self, p: &dyn SHCoordinates<T>) -> T {
        self.eval_indiv_plain(p).into_iter().sum()
    }

    #[inline]
    pub fn eval_indiv(&self, p: &dyn SHCoordinates<T>) -> Vec<T> {
        self.eval_indiv_plain(p)
            .iter()
            .zip(self.coeffs.iter())
            .map(|(&a, &b)| a * b)
            .collect()
    }

    #[inline]
    pub fn eval_indiv_plain(&self, p: &dyn SHCoordinates<T>) -> Vec<T> {
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

    pub fn new(order: usize, sh_type: RealSHType) -> RealSphericalHarmonics<T> {
        let n = (0..=order).map(|o| (2 * o + 1)).sum();

        RealSphericalHarmonics {
            order,
            num_sh: n,
            coeffs: vec![T::one(); n],
            sh: sh_type,
        }
    }

    pub fn set_coeffs(&mut self, coeffs: Vec<T>) {
        assert_eq!(coeffs.len(), self.num_sh);
        self.coeffs = coeffs;
    }
}

pub fn sph_mat<
    'a,
    T: 'a + Float + FromPrimitive + FloatConst + AddAssign + std::iter::Sum + Debug,
>(
    order: usize,
    pos: &[impl SHCoordinates<T>],
    sh_type: RealSHType,
) -> Array2<T> {
    let sh = RealSphericalHarmonics::new(order, sh_type);
    let mut mat = unsafe { Array2::uninitialized((pos.len(), sh.num_sh)) };
    for (i, item) in pos.iter().enumerate() {
        mat.slice_mut(s![i, ..])
            .assign(&Array1::from(sh.eval_indiv_plain(item)));
    }
    mat
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    // #[test]
    // fn it_works() {
    //     let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
    //     let v = sh10(&p);
    //     // let bla: SphericalHarmonics<f64> = SphericalHarmonics::new(3, );
    //     // println!("p: {:?} | v: {}", p, v);
    //     assert_eq!(2 + 2, 4);
    // }

    #[test]
    fn comp() {
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        // let p = Coordinates::cartesian(1.0, 1.0, 0.3);
        assert!((real_SH(2, 1, &p) - sh2p1(&p)) < std::f64::EPSILON);
        assert!((real_SH(3, -2, &p) - sh3n2(&p)) < std::f64::EPSILON);
    }

    // #[test]
    // fn sph_mat_test() {
    //     let p1 = Coordinates::spherical(1.0, PI / 2.0, 0.0);
    //     let p1 = p1.finalize();
    //     let p2 = Coordinates::spherical(0.7, PI / 4.0, 0.0);
    //     let p2 = p2.finalize();
    //     let fu = vec![&p1, &p2];
    //     let bla = sph_mat(1, &fu);
    //     println!("{:#?}", bla);
    //
    //     // assert!((real_SH(2, 1, &p) - sh2p1(&p)) < std::f64::EPSILON);
    //     // assert!((real_SH(3, -2, &p) - sh3n2(&p)) < std::f64::EPSILON);
    // }
}
