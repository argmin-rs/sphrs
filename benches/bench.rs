// Copyright 2018-2020 Stefan Kroboth
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
    use std::f32::consts::PI as PI32;
    use std::f64::consts::PI;
    use test::{black_box, Bencher};

    #[bench]
    fn eval_single(b: &mut Bencher) {
        let sh = RealSHType::Spherical;
        let l = 4;
        let m = -1;
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(l, m, &p));
        });
    }

    #[bench]
    fn eval(b: &mut Bencher) {
        let sh_type = RealSHType::Spherical;
        let degree = 5;
        let sh: HarmonicsSet<f64, _, _> = HarmonicsSet::new(degree, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_f32(b: &mut Bencher) {
        let sh_type = RealSHType::Spherical;
        let degree = 5;
        let sh: HarmonicsSet<f32, _, _> = HarmonicsSet::new(degree, sh_type);
        let p = Coordinates::spherical(1.0f32, PI32 / 2.0f32, 0.0f32);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_real_degree_01(b: &mut Bencher) {
        let sh_type = RealSHType::Spherical;
        let degree = 1;
        let sh: HarmonicsSet<f64, _, _> = HarmonicsSet::new(degree, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_real_degree_02(b: &mut Bencher) {
        let sh_type = RealSHType::Spherical;
        let degree = 2;
        let sh: HarmonicsSet<f64, _, _> = HarmonicsSet::new(degree, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_real_degree_03(b: &mut Bencher) {
        let sh_type = RealSHType::Spherical;
        let degree = 3;
        let sh: HarmonicsSet<f64, _, _> = HarmonicsSet::new(degree, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_real_degree_04(b: &mut Bencher) {
        let sh_type = RealSHType::Spherical;
        let degree = 4;
        let sh: HarmonicsSet<f64, _, _> = HarmonicsSet::new(degree, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_real_degree_05(b: &mut Bencher) {
        let sh_type = RealSHType::Spherical;
        let degree = 5;
        let sh: HarmonicsSet<f64, _, _> = HarmonicsSet::new(degree, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_real_degree_06(b: &mut Bencher) {
        let sh_type = RealSHType::Spherical;
        let degree = 6;
        let sh: HarmonicsSet<f64, _, _> = HarmonicsSet::new(degree, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_real_degree_07(b: &mut Bencher) {
        let sh_type = RealSHType::Spherical;
        let degree = 7;
        let sh: HarmonicsSet<f64, _, _> = HarmonicsSet::new(degree, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_real_degree_08(b: &mut Bencher) {
        let sh_type = RealSHType::Spherical;
        let degree = 8;
        let sh: HarmonicsSet<f64, _, _> = HarmonicsSet::new(degree, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_real_degree_09(b: &mut Bencher) {
        let sh_type = RealSHType::Spherical;
        let degree = 9;
        let sh: HarmonicsSet<f64, _, _> = HarmonicsSet::new(degree, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_real_degree_10(b: &mut Bencher) {
        let sh_type = RealSHType::Spherical;
        let degree = 10;
        let sh: HarmonicsSet<f64, _, _> = HarmonicsSet::new(degree, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_real_degree_11(b: &mut Bencher) {
        let sh_type = RealSHType::Spherical;
        let degree = 11;
        let sh: HarmonicsSet<f64, _, _> = HarmonicsSet::new(degree, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_real_degree_12(b: &mut Bencher) {
        let sh_type = RealSHType::Spherical;
        let degree = 12;
        let sh: HarmonicsSet<f64, _, _> = HarmonicsSet::new(degree, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_real_degree_13(b: &mut Bencher) {
        let sh_type = RealSHType::Spherical;
        let degree = 13;
        let sh: HarmonicsSet<f64, _, _> = HarmonicsSet::new(degree, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_real_degree_14(b: &mut Bencher) {
        let sh_type = RealSHType::Spherical;
        let degree = 14;
        let sh: HarmonicsSet<f64, _, _> = HarmonicsSet::new(degree, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_real_degree_15(b: &mut Bencher) {
        let sh_type = RealSHType::Spherical;
        let degree = 15;
        let sh: HarmonicsSet<f64, _, _> = HarmonicsSet::new(degree, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_real_degree_16(b: &mut Bencher) {
        let sh_type = RealSHType::Spherical;
        let degree = 16;
        let sh: HarmonicsSet<f64, _, _> = HarmonicsSet::new(degree, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_real_degree_17(b: &mut Bencher) {
        let sh_type = RealSHType::Spherical;
        let degree = 17;
        let sh: HarmonicsSet<f64, _, _> = HarmonicsSet::new(degree, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_real_degree_18(b: &mut Bencher) {
        let sh_type = RealSHType::Spherical;
        let degree = 18;
        let sh: HarmonicsSet<f64, _, _> = HarmonicsSet::new(degree, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_real_degree_19(b: &mut Bencher) {
        let sh_type = RealSHType::Spherical;
        let degree = 19;
        let sh: HarmonicsSet<f64, _, _> = HarmonicsSet::new(degree, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_real_degree_20(b: &mut Bencher) {
        let sh_type = RealSHType::Spherical;
        let degree = 20;
        let sh: HarmonicsSet<f64, _, _> = HarmonicsSet::new(degree, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }
}
