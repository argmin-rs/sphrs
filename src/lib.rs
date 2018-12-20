// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! todo

pub mod coords;

use crate::coords::*;
use num_complex::Complex;
use std::f64::consts::{FRAC_1_PI, SQRT_2};

pub fn sh00(_p: &impl Coordinates) -> f64 {
    0.5 * FRAC_1_PI.sqrt()
}

pub fn sh1n1(p: &impl Coordinates) -> f64 {
    (3.0 / 4.0 * FRAC_1_PI).sqrt() * p.y() / p.r()
}

pub fn sh10(p: &impl Coordinates) -> f64 {
    (3.0 / 4.0 * FRAC_1_PI).sqrt() * p.z() / p.r()
}

pub fn sh1p1(p: &impl Coordinates) -> f64 {
    (3.0 / 4.0 * FRAC_1_PI).sqrt() * p.x() / p.r()
}

pub fn sh2n2(p: &impl Coordinates) -> f64 {
    0.5 * (15.0 * FRAC_1_PI).sqrt() * (p.x() * p.y()) / p.r().powi(2)
}

pub fn sh2n1(p: &impl Coordinates) -> f64 {
    0.5 * (15.0 * FRAC_1_PI).sqrt() * (p.y() * p.z()) / p.r().powi(2)
}

pub fn sh20(p: &impl Coordinates) -> f64 {
    0.25 * (15.0 * FRAC_1_PI).sqrt() * (-p.x().powi(2) - p.y().powi(2) * 2.0 * p.z().powi(2))
        / p.r().powi(2)
}

pub fn sh2p1(p: &impl Coordinates) -> f64 {
    0.5 * (15.0 * FRAC_1_PI).sqrt() * (p.z() * p.x()) / p.r().powi(2)
}

pub fn sh2p2(p: &impl Coordinates) -> f64 {
    0.25 * (15.0 * FRAC_1_PI).sqrt() * (p.x().powi(2) * p.y().powi(2)) / p.r().powi(2)
}

pub fn sh3n3(p: &impl Coordinates) -> f64 {
    0.25 * (35.0 / 2.0 * FRAC_1_PI).sqrt() * (3.0 * p.x().powi(2) - p.y().powi(2)) * p.y()
        / p.r().powi(3)
}

pub fn sh3n2(p: &impl Coordinates) -> f64 {
    0.5 * (105.0 * FRAC_1_PI).sqrt() * (p.x() * p.y() * p.z()) / p.r().powi(3)
}

pub fn sh3n1(p: &impl Coordinates) -> f64 {
    0.25 * (21.0 / 2.0 * FRAC_1_PI).sqrt()
        * p.y()
        * (4.0 * p.z().powi(2) - p.x().powi(2) - p.y().powi(2))
        / p.r().powi(3)
}

pub fn sh30(p: &impl Coordinates) -> f64 {
    0.25 * (7.0 * FRAC_1_PI).sqrt()
        * p.z()
        * (2.0 * p.z().powi(2) - 3.0 * p.x().powi(2) - 3.0 * p.y().powi(2))
        / p.r().powi(3)
}

pub fn sh3p1(p: &impl Coordinates) -> f64 {
    0.25 * (21.0 / 2.0 * FRAC_1_PI) * p.x() * (4.0 * p.z().powi(2) - p.x().powi(2) - p.y().powi(2))
        / p.r().powi(3)
}

pub fn sh3p2(p: &impl Coordinates) -> f64 {
    0.25 * (105.0 * FRAC_1_PI) * (p.x().powi(2) - p.y().powi(2)) * p.z() / p.r().powi(3)
}

pub fn sh3p3(p: &impl Coordinates) -> f64 {
    0.25 * (35.0 / 2.0 * FRAC_1_PI) * (p.x().powi(2) - 3.0 * p.y().powi(2)) * p.x().powi(2)
        / p.r().powi(3)
}

#[inline(always)]
fn factorial(n: i64) -> i64 {
    (1..=n).fold(1, |acc, x| acc * x)
}

#[allow(non_snake_case)]
#[inline(always)]
fn K(l: i64, m: i64) -> f64 {
    let m = m.abs();
    (FRAC_1_PI * ((2 * l + 1) as f64) / 4.0 * (factorial(l - m) as f64) / (factorial(l + m) as f64))
        .sqrt()
}

#[allow(non_snake_case)]
fn P(l: i64, m: i64, x: f64) -> f64 {
    let mut pmm: f64 = 1.0;
    if m > 0 {
        let somx2 = ((1.0 - x) * (1.0 + x)).sqrt();
        let mut fact: f64 = 1.0;
        for _ in 1..=m {
            pmm *= -fact * somx2;
            fact += 2.0;
        }
    }

    if l == m {
        return pmm;
    }

    let mut pmmp1 = x * (2.0 * m as f64 + 1.0) * pmm;

    if l == m + 1 {
        return pmmp1;
    }

    let mf = m as f64;

    let mut pll = 0.0;
    for ll in (m + 2 + 1)..=l {
        let ll = ll as f64;
        pll = ((2.0 * ll - 1.0) * x * pmmp1 - (ll + mf - 1.0) * pmm) / (ll - mf);
        pmm = pmmp1;
        pmmp1 = pll;
    }
    pll
}

#[allow(non_snake_case)]
fn SH(l: i64, m: i64, p: &impl Coordinates) -> Complex<f64> {
    let v = if m == 0 {
        K(l, 0) * P(l, m, p.theta().cos())
    } else if m > 0 {
        K(l, m) as f64 * P(l, m, p.theta().cos())
    } else {
        K(l, -m) as f64 * P(l, -m, p.theta().cos())
    };
    let m = m as f64;
    Complex::new(v * (m * p.phi()).sin(), v * (m * p.phi()).cos())
}

#[allow(non_snake_case)]
fn RealSH(l: i64, m: i64, p: &impl Coordinates) -> f64 {
    if m == 0 {
        K(l, 0) * P(l, m, p.theta().cos())
    } else if m > 0 {
        SQRT_2 * K(l, m) as f64 * ((m as f64) * p.phi()).cos() * P(l, m, p.theta().cos())
    } else {
        SQRT_2 * K(l, -m) as f64 * (-(m as f64) * p.phi()).sin() * P(l, -m, p.theta().cos())
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
        // println!("p: {:?} | v: {}", p, v);
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn comp() {
        let p = GenCoordinates::spherical(1.0, PI / 2.0, 0.0);
        // let p = GenCoordinates::cartesian(1.0, 1.0, 0.3);
        assert!((RealSH(2, 1, &p) - sh2p1(&p)) < std::f64::EPSILON);
        assert!((RealSH(3, -2, &p) - sh3n2(&p)) < std::f64::EPSILON);
    }
}
