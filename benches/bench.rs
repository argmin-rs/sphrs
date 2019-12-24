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
    fn sh_1(b: &mut Bencher) {
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(real_regular_solid_SH(99, 98, &p));
        });
    }

    #[bench]
    fn k_1(b: &mut Bencher) {
        b.iter(|| {
            black_box(K::<f64>(99, 98));
        });
    }

    #[bench]
    fn sh_1_static(b: &mut Bencher) {
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh99p98(&p));
        });
    }

    #[bench]
    fn sh2p1_gen(b: &mut Bencher) {
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(real_regular_solid_SH(2, 1, &p));
        });
    }

    #[bench]
    fn sh2p1_hc(b: &mut Bencher) {
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh2p1(&p));
        });
    }

    #[bench]
    fn sh3p3_gen(b: &mut Bencher) {
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(real_regular_solid_SH(3, 3, &p));
        });
    }

    #[bench]
    fn sh3p3_hc(b: &mut Bencher) {
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh3p3(&p));
        });
    }

    #[bench]
    fn eval(b: &mut Bencher) {
        let sh_type = RealSHType::Standard;
        let order = 5;
        let sh = RealSphericalHarmonics::new(order, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_f32(b: &mut Bencher) {
        let sh_type = RealSHType::Standard;
        let order = 5;
        let sh = RealSphericalHarmonics::new(order, sh_type);
        let p = Coordinates::spherical(1.0f32, PI32 / 2.0f32, 0.0f32);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_order_01(b: &mut Bencher) {
        let sh_type = RealSHType::Standard;
        let order = 1;
        let sh = RealSphericalHarmonics::new(order, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_order_02(b: &mut Bencher) {
        let sh_type = RealSHType::Standard;
        let order = 2;
        let sh = RealSphericalHarmonics::new(order, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_order_03(b: &mut Bencher) {
        let sh_type = RealSHType::Standard;
        let order = 3;
        let sh = RealSphericalHarmonics::new(order, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_order_04(b: &mut Bencher) {
        let sh_type = RealSHType::Standard;
        let order = 4;
        let sh = RealSphericalHarmonics::new(order, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_order_05(b: &mut Bencher) {
        let sh_type = RealSHType::Standard;
        let order = 5;
        let sh = RealSphericalHarmonics::new(order, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_order_06(b: &mut Bencher) {
        let sh_type = RealSHType::Standard;
        let order = 6;
        let sh = RealSphericalHarmonics::new(order, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_order_07(b: &mut Bencher) {
        let sh_type = RealSHType::Standard;
        let order = 7;
        let sh = RealSphericalHarmonics::new(order, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_order_08(b: &mut Bencher) {
        let sh_type = RealSHType::Standard;
        let order = 8;
        let sh = RealSphericalHarmonics::new(order, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_order_09(b: &mut Bencher) {
        let sh_type = RealSHType::Standard;
        let order = 9;
        let sh = RealSphericalHarmonics::new(order, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_order_10(b: &mut Bencher) {
        let sh_type = RealSHType::Standard;
        let order = 10;
        let sh = RealSphericalHarmonics::new(order, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_order_11(b: &mut Bencher) {
        let sh_type = RealSHType::Standard;
        let order = 11;
        let sh = RealSphericalHarmonics::new(order, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_order_12(b: &mut Bencher) {
        let sh_type = RealSHType::Standard;
        let order = 12;
        let sh = RealSphericalHarmonics::new(order, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_order_13(b: &mut Bencher) {
        let sh_type = RealSHType::Standard;
        let order = 13;
        let sh = RealSphericalHarmonics::new(order, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_order_14(b: &mut Bencher) {
        let sh_type = RealSHType::Standard;
        let order = 14;
        let sh = RealSphericalHarmonics::new(order, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_order_15(b: &mut Bencher) {
        let sh_type = RealSHType::Standard;
        let order = 15;
        let sh = RealSphericalHarmonics::new(order, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_order_16(b: &mut Bencher) {
        let sh_type = RealSHType::Standard;
        let order = 16;
        let sh = RealSphericalHarmonics::new(order, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_order_17(b: &mut Bencher) {
        let sh_type = RealSHType::Standard;
        let order = 17;
        let sh = RealSphericalHarmonics::new(order, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_order_18(b: &mut Bencher) {
        let sh_type = RealSHType::Standard;
        let order = 18;
        let sh = RealSphericalHarmonics::new(order, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_order_19(b: &mut Bencher) {
        let sh_type = RealSHType::Standard;
        let order = 19;
        let sh = RealSphericalHarmonics::new(order, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }

    #[bench]
    fn eval_order_20(b: &mut Bencher) {
        let sh_type = RealSHType::Standard;
        let order = 20;
        let sh = RealSphericalHarmonics::new(order, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }
}
