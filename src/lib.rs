// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! todo

pub mod coords;

use crate::coords::*;
use std::f64::consts::FRAC_1_PI;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn it_works() {
        let p = GenCoordinates::spherical(1.0, PI / 2.0, 0.0);
        let v = sh10(&p);
        println!("p: {:?} | v: {}", p, v);
        assert_eq!(2 + 2, 4);
    }
}
