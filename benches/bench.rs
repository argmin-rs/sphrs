// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! TODO

#![feature(test)]

extern crate sphrs;
extern crate test;

#[cfg(test)]
mod tests {
    use sphrs::*;
    use std::f64::consts::PI;
    use test::{black_box, Bencher};

    #[bench]
    fn sh_1(b: &mut Bencher) {
        let p = GenCoordinates::spherical(1.0, PI / 2.0, 0.0);
        // let p = GenCoordinates::spherical(1.0f32, std::f32::consts::PI / 2.0f32, 0.0f32);
        // black_box(real_SH(2, 1, &p));
        // let p = GenCoordinates::cartesian(1.0, 1.0, 0.3);
        // assert!((real_SH(2, 1, &p) - sh2p1(&p)) < std::f64::EPSILON);
        // assert!((real_SH(3, -2, &p) - sh3n2(&p)) < std::f64::EPSILON);
        b.iter(|| {
            black_box(real_regular_solid_SH(99, 98, &p));
        });
    }

    #[bench]
    fn sh2p1_gen(b: &mut Bencher) {
        let p = GenCoordinates::spherical(1.0, PI / 2.0, 0.0);
        let p = p.finalize();
        b.iter(|| {
            black_box(real_regular_solid_SH(2, 1, &p));
        });
    }

    #[bench]
    fn sh2p1_hc(b: &mut Bencher) {
        let p = GenCoordinates::spherical(1.0, PI / 2.0, 0.0);
        let p = p.finalize();
        b.iter(|| {
            black_box(sh2p1(&p));
        });
    }

    #[bench]
    fn sh3p3_gen(b: &mut Bencher) {
        let p = GenCoordinates::spherical(1.0, PI / 2.0, 0.0);
        let p = p.finalize();
        b.iter(|| {
            black_box(real_regular_solid_SH(3, 3, &p));
        });
    }

    #[bench]
    fn sh3p3_hc(b: &mut Bencher) {
        let p = GenCoordinates::spherical(1.0, PI / 2.0, 0.0);
        let p = p.finalize();
        b.iter(|| {
            black_box(sh3p3(&p));
        });
    }
}
