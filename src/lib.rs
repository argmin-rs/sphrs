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
use ndarray::{s, Array1, Array2};
use num::{Float, FromPrimitive};
use num_traits::float::FloatConst;
use std::ops::AddAssign;
// use num_complex::Complex;

pub enum SHType {
    Standard,
    Real,
    RegularSolid,
    IrregularSolid,
    RealRegularSolid,
    RealIrregularSolid,
}

pub struct SphericalHarmonics<T>
where
    T: Float + FromPrimitive + FloatConst,
{
    order: usize,
    num_sh: usize,
    coeffs: Vec<T>,
}

impl<T> SphericalHarmonics<T>
where
    T: Float + FromPrimitive + FloatConst + AddAssign,
{
    pub fn eval_vec(&self, p: &Vec<impl Coordinates<T>>) -> Vec<T> {
        p.iter().map(|pi| self.eval(pi)).collect()
    }

    pub fn eval(&self, p: &Coordinates<T>) -> T {
        let mut res = T::zero();
        let mut j = 0;
        for l in 0..=self.order {
            let l = l as i64;
            for m in -l..=l {
                res += self.coeffs[j] * real_SH_hc(m, l, p);
                j += 1;
            }
        }

        res
    }

    // pub fn eval(&self, p: &Coordinates<T>) -> T {
    //     let mut res = self.coeffs[0] * sh00(p);
    //
    //     if self.order >= 1 {
    //         res = res
    //             + self.coeffs[1] * sh1n1(p)
    //             + self.coeffs[2] * sh10(p)
    //             + self.coeffs[3] * sh1p1(p);
    //     }
    //
    //     if self.order >= 2 {
    //         res = res
    //             + self.coeffs[4] * sh2n2(p)
    //             + self.coeffs[5] * sh2n1(p)
    //             + self.coeffs[6] * sh20(p)
    //             + self.coeffs[7] * sh2p1(p)
    //             + self.coeffs[8] * sh2p2(p);
    //     }
    //
    //     if self.order >= 3 {
    //         res = res
    //             + self.coeffs[9] * sh3n3(p)
    //             + self.coeffs[10] * sh3n2(p)
    //             + self.coeffs[11] * sh3n1(p)
    //             + self.coeffs[12] * sh30(p)
    //             + self.coeffs[13] * sh3p1(p)
    //             + self.coeffs[14] * sh3p2(p)
    //             + self.coeffs[15] * sh3p3(p);
    //     }
    //
    //     let mut j = 16;
    //     if self.order >= 4 {
    //         for l in 4..=self.order {
    //             let l = l as i64;
    //             for m in (-l)..=l {
    //                 let m = m as i64;
    //                 res = res + self.coeffs[j] * real_SH(m, l, p);
    //                 j += 1;
    //             }
    //         }
    //     }
    //
    //     res
    // }

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

    pub fn eval_plain(&self, p: &impl Coordinates<T>) -> Vec<T> {
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
        let n = (0..=order).map(|o| (2 * o + 1)).sum();
        SphericalHarmonics {
            order,
            num_sh: n,
            coeffs: vec![T::one(); n],
        }
    }

    pub fn set_coeffs(&mut self, coeffs: Vec<T>) {
        assert_eq!(coeffs.len(), self.num_sh);
        self.coeffs = coeffs;
    }
}

pub fn sph_mat<T: Float + FromPrimitive + FloatConst + AddAssign>(
    order: usize,
    pos: &Vec<impl Coordinates<T>>,
) -> Array2<T> {
    let sh = SphericalHarmonics::new(order);
    let mut mat = unsafe { Array2::uninitialized((pos.len(), sh.num_sh)) };
    for i in 0..pos.len() {
        let bla = &pos[i];
        mat.slice_mut(s![i, ..])
            .assign(&Array1::from_vec(sh.eval_plain(bla)));
    }
    mat
}

pub struct RealRegularSolidSphericalHarmonics<T>
where
    T: Float + FromPrimitive + FloatConst + AddAssign,
{
    order: usize,
    num_sh: usize,
    coeffs: Vec<T>,
    // sh: Vec<fn(&Coordinates<T>) -> T>,
}

impl<T> RealRegularSolidSphericalHarmonics<T>
where
    T: Float + FromPrimitive + FloatConst + AddAssign,
{
    pub fn eval_vec(&self, p: &Vec<impl Coordinates<T>>) -> Vec<T> {
        p.iter().map(|pi| self.eval(pi)).collect()
    }

    pub fn eval(&self, p: &Coordinates<T>) -> T {
        let mut res = T::zero();
        let mut j = 0;
        for l in 0..=self.order {
            let l = l as i64;
            for m in -l..=l {
                res += self.coeffs[j] * real_regular_solid_SH(m, l, p);
                j += 1;
            }
        }

        res
    }
    // pub fn eval_indiv(&self, p: &Coordinates<T>) -> Vec<T> {
    //     let mut sh = Vec::with_capacity(self.num_sh);
    //
    //     sh.push(self.coeffs[0] * sh00(p));
    //
    //     if self.order >= 1 {
    //         sh.push(self.coeffs[1] * sh1n1(p));
    //         sh.push(self.coeffs[2] * sh10(p));
    //         sh.push(self.coeffs[3] * sh1p1(p));
    //     }
    //
    //     if self.order >= 2 {
    //         sh.push(self.coeffs[4] * sh2n2(p));
    //         sh.push(self.coeffs[5] * sh2n1(p));
    //         sh.push(self.coeffs[6] * sh20(p));
    //         sh.push(self.coeffs[7] * sh2p1(p));
    //         sh.push(self.coeffs[8] * sh2p2(p));
    //     }
    //
    //     if self.order >= 3 {
    //         sh.push(self.coeffs[9] * sh3n3(p));
    //         sh.push(self.coeffs[10] * sh3n2(p));
    //         sh.push(self.coeffs[11] * sh3n1(p));
    //         sh.push(self.coeffs[12] * sh30(p));
    //         sh.push(self.coeffs[13] * sh3p1(p));
    //         sh.push(self.coeffs[14] * sh3p2(p));
    //         sh.push(self.coeffs[15] * sh3p3(p));
    //     }
    //
    //     let mut j = 16;
    //     if self.order >= 4 {
    //         for l in 4..=self.order {
    //             let l = l as i64;
    //             for m in (-l)..=l {
    //                 let m = m as i64;
    //                 sh.push(self.coeffs[j] * real_SH(m, l, p));
    //                 j += 1;
    //             }
    //         }
    //     }
    //
    //     sh
    // }

    // pub fn eval_plain(&self, p: &impl Coordinates<T>) -> Vec<T> {
    //     let mut sh = Vec::with_capacity(self.num_sh);
    //
    //     sh.push(sh00(p));
    //
    //     if self.order >= 1 {
    //         sh.push(sh1n1(p));
    //         sh.push(sh10(p));
    //         sh.push(sh1p1(p));
    //     }
    //
    //     if self.order >= 2 {
    //         sh.push(sh2n2(p));
    //         sh.push(sh2n1(p));
    //         sh.push(sh20(p));
    //         sh.push(sh2p1(p));
    //         sh.push(sh2p2(p));
    //     }
    //
    //     if self.order >= 3 {
    //         sh.push(sh3n3(p));
    //         sh.push(sh3n2(p));
    //         sh.push(sh3n1(p));
    //         sh.push(sh30(p));
    //         sh.push(sh3p1(p));
    //         sh.push(sh3p2(p));
    //         sh.push(sh3p3(p));
    //     }
    //
    //     if self.order >= 4 {
    //         for l in 4..=self.order {
    //             let l = l as i64;
    //             for m in (-l)..=l {
    //                 let m = m as i64;
    //                 sh.push(real_SH(m, l, p));
    //             }
    //         }
    //     }
    //
    //     sh
    // }

    pub fn new(order: usize) -> Self {
        let n = (0..=order).map(|o| (2 * o + 1)).sum();
        RealRegularSolidSphericalHarmonics {
            order,
            num_sh: n,
            coeffs: vec![T::one(); n],
        }
    }

    pub fn set_coeffs(&mut self, coeffs: Vec<T>) {
        assert_eq!(coeffs.len(), self.num_sh);
        self.coeffs = coeffs;
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

    // #[test]
    // fn sph_mat_test() {
    //     let p1 = GenCoordinates::spherical(1.0, PI / 2.0, 0.0);
    //     let p1 = p1.finalize();
    //     let p2 = GenCoordinates::spherical(0.7, PI / 4.0, 0.0);
    //     let p2 = p2.finalize();
    //     let fu = vec![&p1, &p2];
    //     let bla = sph_mat(1, &fu);
    //     println!("{:#?}", bla);
    //
    //     // assert!((real_SH(2, 1, &p) - sh2p1(&p)) < std::f64::EPSILON);
    //     // assert!((real_SH(3, -2, &p) - sh3n2(&p)) < std::f64::EPSILON);
    // }
}
