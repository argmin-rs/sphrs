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
    fn full_set(b: &mut Bencher) {
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0).finalize();
        let s: RealSphericalHarmonics<f64> = RealSphericalHarmonics::new(3, RealSHType::Standard);
        b.iter(|| {
            black_box(s.eval(&p));
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
        let p = p.finalize();
        b.iter(|| {
            black_box(real_regular_solid_SH(2, 1, &p));
        });
    }

    #[bench]
    fn sh2p1_hc(b: &mut Bencher) {
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        let p = p.finalize();
        b.iter(|| {
            black_box(sh2p1(&p));
        });
    }

    #[bench]
    fn sh3p3_gen(b: &mut Bencher) {
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        let p = p.finalize();
        b.iter(|| {
            black_box(real_regular_solid_SH(3, 3, &p));
        });
    }

    #[bench]
    fn sh3p3_hc(b: &mut Bencher) {
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        let p = p.finalize();
        b.iter(|| {
            black_box(sh3p3(&p));
        });
    }

    #[bench]
    fn sph_mat_1(b: &mut Bencher) {
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        let p = p.finalize();
        let fu = vec![p; 10000];
        b.iter(|| {
            black_box(sph_mat(3, &fu, RealSHType::Standard));
        });
    }

    #[bench]
    fn sph_mat_2(b: &mut Bencher) {
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        let p = p.finalize();
        let fu = vec![p; 1000];
        b.iter(|| {
            black_box(sph_mat(3, &fu, RealSHType::Standard));
        });
    }

    #[bench]
    fn sph_mat_3(b: &mut Bencher) {
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        let p = p.finalize();
        let fu = vec![p; 10000];
        b.iter(|| {
            black_box(sph_mat(4, &fu, RealSHType::Standard));
        });
    }

    #[bench]
    fn sph_mat_4(b: &mut Bencher) {
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        let p = p.finalize();
        let fu = vec![p; 1000];
        b.iter(|| {
            black_box(sph_mat(4, &fu, RealSHType::Standard));
        });
    }

    // #[bench]
    // fn sph_mat_5(b: &mut Bencher) {
    //     let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
    //     let p = p.finalize();
    //     let fu = vec![p; 10000];
    //     b.iter(|| {
    //         black_box(sph_mat(5, &fu, RealSHType::Standard));
    //     });
    // }
    //
    // #[bench]
    // fn sph_mat_6(b: &mut Bencher) {
    //     let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
    //     let p = p.finalize();
    //     let fu = vec![p; 1000];
    //     b.iter(|| {
    //         black_box(sph_mat(5, &fu, RealSHType::Standard));
    //     });
    // }

    #[bench]
    fn eval_indiv_plain(b: &mut Bencher) {
        let sh_type = RealSHType::Standard;
        let order = 5;
        let sh = RealSphericalHarmonics::new(order, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        let p = p.finalize();
        // let fu = vec![p; 10000];
        b.iter(|| {
            black_box(sh.eval_indiv_plain(&p));
        });
    }

    #[bench]
    fn eval_indiv(b: &mut Bencher) {
        let sh_type = RealSHType::Standard;
        let order = 5;
        let sh = RealSphericalHarmonics::new(order, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        let p = p.finalize();
        // let fu = vec![p; 10000];
        b.iter(|| {
            black_box(sh.eval_indiv(&p));
        });
    }

    #[bench]
    fn eval_plain(b: &mut Bencher) {
        let sh_type = RealSHType::Standard;
        let order = 5;
        let sh = RealSphericalHarmonics::new(order, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        let p = p.finalize();
        // let fu = vec![p; 10000];
        b.iter(|| {
            black_box(sh.eval_plain(&p));
        });
    }

    #[bench]
    fn eval(b: &mut Bencher) {
        let sh_type = RealSHType::Standard;
        let order = 5;
        let sh = RealSphericalHarmonics::new(order, sh_type);
        let p = Coordinates::spherical(1.0, PI / 2.0, 0.0);
        let p = p.finalize();
        // let fu = vec![p; 10000];
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
        let p = p.finalize();
        // let fu = vec![p; 10000];
        b.iter(|| {
            black_box(sh.eval(&p));
        });
    }
}
