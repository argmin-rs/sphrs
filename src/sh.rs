// Copyright 2018-2020 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

// Allow comparison chains because benchmarking shows that they are much faster than match
// expressions.
#![allow(clippy::comparison_chain)]

use crate::coords::*;
use crate::SphrsFloat;
use num_complex::Complex;

/// Hardcoded SH (l=0,m=0)
pub fn sh00<T: SphrsFloat>(_p: &dyn SHCoordinates<T>) -> T {
    T::from_f64(0.5).unwrap() * T::FRAC_1_PI().sqrt()
}

/// Hardcoded SH (l=1,m=-1)
pub fn sh1n1<T: SphrsFloat>(p: &dyn SHCoordinates<T>) -> T {
    -(T::from_f64(0.75).unwrap() * T::FRAC_1_PI()).sqrt() * p.y() / p.r()
}

/// Hardcoded SH (l=1,m=0)
pub fn sh10<T: SphrsFloat>(p: &dyn SHCoordinates<T>) -> T {
    (T::from_f64(0.75).unwrap() * T::FRAC_1_PI()).sqrt() * p.z() / p.r()
}

/// Hardcoded SH (l=1,m=1)
pub fn sh1p1<T: SphrsFloat>(p: &dyn SHCoordinates<T>) -> T {
    -(T::from_f64(0.75).unwrap() * T::FRAC_1_PI()).sqrt() * p.x() / p.r()
}

/// Hardcoded SH (l=2,m=-2)
pub fn sh2n2<T: SphrsFloat>(p: &dyn SHCoordinates<T>) -> T {
    T::from_f64(0.5).unwrap()
        * (T::from_f64(15.0).unwrap() * T::FRAC_1_PI()).sqrt()
        * (p.x() * p.y())
        / p.r().powi(2)
}

/// Hardcoded SH (l=2,m=-1)
pub fn sh2n1<T: SphrsFloat>(p: &dyn SHCoordinates<T>) -> T {
    -T::from_f64(0.5).unwrap()
        * (T::from_f64(15.0).unwrap() * T::FRAC_1_PI()).sqrt()
        * (p.y() * p.z())
        / p.r().powi(2)
}

/// Hardcoded SH (l=2,m=0)
pub fn sh20<T: SphrsFloat>(p: &dyn SHCoordinates<T>) -> T {
    T::from_f64(0.25).unwrap()
        * (T::from_f64(5.0).unwrap() * T::FRAC_1_PI()).sqrt()
        * (-p.x().powi(2) - p.y().powi(2) + T::from_f64(2.0).unwrap() * p.z().powi(2))
        / p.r().powi(2)
}

/// Hardcoded SH (l=2,m=1)
pub fn sh2p1<T: SphrsFloat>(p: &dyn SHCoordinates<T>) -> T {
    -T::from_f64(0.5).unwrap()
        * (T::from_f64(15.0).unwrap() * T::FRAC_1_PI()).sqrt()
        * (p.z() * p.x())
        / p.r().powi(2)
}

/// Hardcoded SH (l=2,m=2)
pub fn sh2p2<T: SphrsFloat>(p: &dyn SHCoordinates<T>) -> T {
    T::from_f64(0.25).unwrap()
        * (T::from_f64(15.0).unwrap() * T::FRAC_1_PI()).sqrt()
        * (p.x().powi(2) - p.y().powi(2))
        / p.r().powi(2)
}

/// Hardcoded SH (l=3,m=-3)
pub fn sh3n3<T: SphrsFloat>(p: &dyn SHCoordinates<T>) -> T {
    -T::from_f64(0.25).unwrap()
        * (T::from_f64(35.0 / 2.0).unwrap() * T::FRAC_1_PI()).sqrt()
        * (T::from_f64(3.0).unwrap() * p.x().powi(2) - p.y().powi(2))
        * p.y()
        / p.r().powi(3)
}

/// Hardcoded SH (l=3,m=-2)
pub fn sh3n2<T: SphrsFloat>(p: &dyn SHCoordinates<T>) -> T {
    T::from_f64(0.5).unwrap()
        * (T::from_f64(105.0).unwrap() * T::FRAC_1_PI()).sqrt()
        * (p.x() * p.y() * p.z())
        / p.r().powi(3)
}

/// Hardcoded SH (l=3,m=-1)
pub fn sh3n1<T: SphrsFloat>(p: &dyn SHCoordinates<T>) -> T {
    -T::from_f64(0.25).unwrap()
        * (T::from_f64(21.0 / 2.0).unwrap() * T::FRAC_1_PI()).sqrt()
        * p.y()
        * (T::from_f64(4.0).unwrap() * p.z().powi(2) - p.x().powi(2) - p.y().powi(2))
        / p.r().powi(3)
}

/// Hardcoded SH (l=3,m=0)
pub fn sh30<T: SphrsFloat>(p: &dyn SHCoordinates<T>) -> T {
    T::from_f64(0.25).unwrap()
        * (T::from_f64(7.0).unwrap() * T::FRAC_1_PI()).sqrt()
        * p.z()
        * (T::from_f64(5.0).unwrap() * p.z().powi(2) - T::from_f64(3.0).unwrap() * p.r().powi(2))
        / p.r().powi(3)
}

/// Hardcoded SH (l=3,m=1)
pub fn sh3p1<T: SphrsFloat>(p: &dyn SHCoordinates<T>) -> T {
    -T::from_f64(0.25).unwrap()
        * (T::from_f64(21.0 / 2.0).unwrap() * T::FRAC_1_PI()).sqrt()
        * p.x()
        * (T::from_f64(4.0).unwrap() * p.z().powi(2) - p.x().powi(2) - p.y().powi(2))
        / p.r().powi(3)
}

/// Hardcoded SH (l=3,m=2)
pub fn sh3p2<T: SphrsFloat>(p: &dyn SHCoordinates<T>) -> T {
    T::from_f64(0.25).unwrap()
        * (T::from_f64(105.0).unwrap() * T::FRAC_1_PI()).sqrt()
        * (p.x().powi(2) - p.y().powi(2))
        * p.z()
        / p.r().powi(3)
}

/// Hardcoded SH (l=3,m=3)
pub fn sh3p3<T: SphrsFloat>(p: &dyn SHCoordinates<T>) -> T {
    -T::from_f64(0.25).unwrap()
        * (T::from_f64(35.0 / 2.0).unwrap() * T::FRAC_1_PI()).sqrt()
        * (p.x().powi(2) - T::from_f64(3.0).unwrap() * p.y().powi(2))
        * p.x()
        / p.r().powi(3)
}

/// Factorial
#[inline]
fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

/// Normalization factor
#[allow(non_snake_case)]
#[inline]
fn K<T: SphrsFloat>(l: i64, m: i64) -> T {
    ((T::from_f64(2.0).unwrap() * T::from_i64(l).unwrap() + T::one())
        * T::from_u64(factorial((l - m).abs() as u64)).unwrap()
        / (T::from_f64(4.0).unwrap()
            * T::PI()
            * T::from_u64(factorial((l + m).abs() as u64)).unwrap()))
    .sqrt()

    // let m = m.abs();
    // (T::FRAC_1_PI() * T::from_i64(2 * l + 1).unwrap() / T::from_f64(4.0).unwrap()
    //     * T::from_u64(factorial((l - m).abs() as u64)).unwrap()
    //     / T::from_u64(factorial((l + m).abs() as u64)).unwrap())
    // .sqrt()
}

/// Legendre polynomials
#[allow(non_snake_case)]
#[inline]
fn P<T: SphrsFloat>(l: i64, m: i64, x: T) -> T {
    let mut pmm = T::one();

    if m > 0 {
        let somx2 = ((T::one() - x) * (T::one() + x)).sqrt();
        let mut fact = T::one();
        for _ in 1..=m {
            pmm = pmm * -fact * somx2;
            fact = fact + T::from_f64(2.0).unwrap();
        }
    }

    if l == m {
        return pmm;
    }

    let mut pmmp1 = x * T::from_i64(2 * m + 1).unwrap() * pmm;

    if l == m + 1 {
        return pmmp1;
    }

    let mut pll = T::zero();
    for ll in (m + 2)..=l {
        pll = (T::from_i64(2 * ll - 1).unwrap() * x * pmmp1
            - (T::from_i64(ll + m - 1)).unwrap() * pmm)
            / T::from_i64(ll - m).unwrap();
        pmm = pmmp1;
        pmmp1 = pll;
    }
    pll
}

/// Complex spherical harmonics
#[allow(non_snake_case)]
#[inline]
pub fn SH<T: SphrsFloat>(l: i64, m: i64, p: &dyn SHCoordinates<T>) -> Complex<T> {
    assert!(l >= 0);
    assert!(m.abs() <= l);
    let v: T = if m == 0 {
        K::<T>(l, 0) * P(l, m, p.theta_cos())
    } else if m > 0 {
        K::<T>(l, m) * P(l, m, p.theta_cos())
    } else {
        K::<T>(l, -m) * P(l, -m, p.theta_cos())
    };
    let sign = if m < 0 {
        T::from_f64((-1f64).powi(m.abs() as i32)).unwrap()
    } else {
        T::from_f64(1.0).unwrap()
    };
    let tmp = T::from_i64(m).unwrap() * p.phi();
    Complex::new(sign * v * tmp.cos(), sign * v * tmp.sin())
}

/// Real spherical harmonics (recursive implementation)
#[allow(non_snake_case)]
#[inline(always)]
pub fn real_SH<T: SphrsFloat>(l: i64, m: i64, p: &dyn SHCoordinates<T>) -> T {
    if m == 0 {
        K::<T>(l, 0) * P(l, m, p.theta_cos())
    } else if m > 0 {
        T::SQRT_2()
            * K::<T>(l, m)
            * (T::from_i64(m).unwrap() * p.phi()).cos()
            * P(l, m, p.theta_cos())
    } else {
        T::SQRT_2()
            * K::<T>(l, -m)
            * (T::from_i64(-m).unwrap() * p.phi()).sin()
            * P(l, -m, p.theta_cos())
    }
}

/// Spherical harmonics. This will use the hardcoded functions if available and the recursive
/// implementation otherwise.
#[allow(non_snake_case)]
#[inline]
pub fn real_SH_hardcoded<T: SphrsFloat>(l: i64, m: i64, p: &dyn SHCoordinates<T>) -> T {
    match (l, m) {
        // 0th degree
        (0, 0) => sh00(p),
        // 1st degree
        (1, -1) => sh1n1(p),
        (1, 0) => sh10(p),
        (1, 1) => sh1p1(p),
        // 2nd degree
        (2, -2) => sh2n2(p),
        (2, -1) => sh2n1(p),
        (2, 0) => sh20(p),
        (2, 1) => sh2p1(p),
        (2, 2) => sh2p2(p),
        // 3rd degree
        (3, -3) => sh3n3(p),
        (3, -2) => sh3n2(p),
        (3, -1) => sh3n1(p),
        (3, 0) => sh30(p),
        (3, 1) => sh3p1(p),
        (3, 2) => sh3p2(p),
        (3, 3) => sh3p3(p),
        // the rest
        _ => real_SH(l, m, p),
    }
}

/// Complex regular solid harmonics
#[allow(non_snake_case)]
#[inline]
pub fn regular_solid_SH<T: SphrsFloat>(l: i64, m: i64, p: &dyn SHCoordinates<T>) -> Complex<T> {
    let scaling = ((T::from_f64(4.0).unwrap() * T::PI()) / T::from_i64(2 * l + 1).unwrap()).sqrt()
        * p.r().powi(l as i32);
    let sh = SH(l, m, p);
    Complex::new(sh.re * scaling, sh.im * scaling)
}

/// Complex irregular solid harmonics
#[allow(non_snake_case)]
#[inline]
pub fn irregular_solid_SH<T: SphrsFloat>(l: i64, m: i64, p: &dyn SHCoordinates<T>) -> Complex<T> {
    let scaling = ((T::from_f64(4.0).unwrap() * T::PI()) / T::from_i64(2 * l + 1).unwrap()).sqrt()
        / p.r().powi((l + 1) as i32);
    let sh = SH(l, m, p);
    Complex::new(sh.re * scaling, sh.im * scaling)
}

/// Real regular solid harmonics
#[allow(non_snake_case)]
#[inline]
pub fn real_regular_solid_SH<T: SphrsFloat>(l: i64, m: i64, p: &dyn SHCoordinates<T>) -> T {
    ((T::from_f64(4.0).unwrap() * T::PI()) / T::from_i64(2 * l + 1).unwrap()).sqrt()
        * p.r().powi(l as i32)
        * real_SH_hardcoded(l, m, p)
}

/// Real irregular solid harmonics
#[allow(non_snake_case)]
#[inline]
pub fn real_irregular_solid_SH<T: SphrsFloat>(l: i64, m: i64, p: &dyn SHCoordinates<T>) -> T {
    ((T::from_f64(4.0).unwrap() * T::PI()) / T::from_i64(2 * l + 1).unwrap()).sqrt()
        / p.r().powi(l as i32)
        * real_SH_hardcoded(l, m, p)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    macro_rules! comp {
        ($l:expr, $m:expr, $p:tt, $hcf:expr, $tol:tt) => {
            let rsh: f64 = real_SH($l, $m, $p);
            let hsh: f64 = $hcf($p);
            assert!((rsh - hsh).abs() < $tol);
        };
    }

    #[test]
    fn compare_hardcoded_and_recursive() {
        let tol = 10.0 * std::f64::EPSILON;
        let c = [
            Coordinates::spherical(1.0, PI / 4.0, PI / 2.0),
            Coordinates::spherical(2.0, PI / 4.0, PI / 2.0),
            Coordinates::spherical(2.0, PI / 2.0, PI / 4.0),
            Coordinates::spherical(0.5, 0.0, PI / 4.0),
            Coordinates::spherical(0.75, PI / 2.0, 0.0),
            Coordinates::cartesian(1.0, 1.0, 0.3),
            Coordinates::cartesian(1.0, 0.0, 0.0),
            Coordinates::cartesian(0.0, 1.0, 0.0),
            Coordinates::cartesian(0.0, 0.0, 1.0),
        ];

        for p in c.iter() {
            // println!("{:?}", p);
            // println!("{:?} | {:?}", real_SH(2, 2, p), sh2p2(p));
            // println!("{:?}", (real_SH(2, 2, p) - sh2p2(p)).abs());
            // 0th degree
            comp!(0, 0, p, sh00, tol);
            // 1st degree
            comp!(1, -1, p, sh1n1, tol);
            comp!(1, 0, p, sh10, tol);
            comp!(1, 1, p, sh1p1, tol);
            // 2nd degree
            comp!(2, -2, p, sh2n2, tol);
            comp!(2, -1, p, sh2n1, tol);
            comp!(2, 0, p, sh20, tol);
            comp!(2, 1, p, sh2p1, tol);
            comp!(2, 2, p, sh2p2, tol);
            // 3rd degree
            comp!(3, -3, p, sh3n3, tol);
            comp!(3, -2, p, sh3n2, tol);
            comp!(3, -1, p, sh3n1, tol);
            comp!(3, 0, p, sh30, tol);
            comp!(3, 1, p, sh3p1, tol);
            comp!(3, 2, p, sh3p2, tol);
            comp!(3, 3, p, sh3p3, tol);
        }
    }

    #[test]
    fn compare_recursive_complex_and_scipy() {
        use csv;
        use std::fs::File;

        let tol = 10.0 * std::f64::EPSILON;
        let file = File::open("test_helpers/scipy.csv").unwrap();
        let mut rdr = csv::Reader::from_reader(file);
        for (_idx, result) in rdr.records().enumerate() {
            let record = result.unwrap();
            let l: i64 = record[0].parse().ok().unwrap();
            let m: i64 = record[1].parse().ok().unwrap();
            let phi: f64 = record[2].parse().ok().unwrap();
            let theta: f64 = record[3].parse().ok().unwrap();
            let scipy_res: Complex<f64> = Complex::new(
                record[4].parse().ok().unwrap(),
                record[5].parse().ok().unwrap(),
            );
            let coords = Coordinates::spherical(1.0, theta, phi);
            let sphrs_res: Complex<f64> = SH(l, m, &coords);
            // println!(
            //     "{:?} | l: {:?}, m: {:?}, phi: {:?}, theta: {:?}, {:?} - {:?}",
            //     idx, l, m, phi, theta, scipy_res, sphrs_res
            // );
            assert!((sphrs_res.re - scipy_res.re).abs() < tol);
            assert!((sphrs_res.im - scipy_res.im).abs() < tol);
        }
    }
}
