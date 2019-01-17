// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! todo

use ndarray::Array1;
use ndarray_linalg::Inverse;
use sphrs::*;
use std::error::Error;
use std::f64::consts::PI;

fn run() -> Result<(), Box<Error>> {
    let mut fu = Vec::with_capacity(8 * 8 * 8);
    // let pos = vec![-1.0f64, -0.75, -0.5, -0.25, 0.25, 0.5, 0.75, 1.0];
    // for i in &pos {
    //     for j in &pos {
    //         for k in &pos {
    //             let p = GenCoordinates::cartesian(*i, *j, *k);
    //             fu.push(p);
    //         }
    //     }
    // }
    let theta = vec![0.0, PI / 4.0, 2.0 * PI / 4.0, 3.0 * PI / 4.0];
    for i in &theta {
        for j in &theta {
            let p = GenCoordinates::spherical(1.0, *i, *j);
            fu.push(p);
        }
    }

    let sh_type = RealSHType::RealRegularSolid;

    let mut target = SphericalHarmonics::new(1, sh_type);
    // target.set_coeffs(vec![0.1, 2.0, 8.9, 3.2]);

    let out = Array1::from_vec(target.eval_vec(&fu));
    // println!("out:\n{:#?}", out);

    let sphm = sph_mat(1, &fu, sh_type);
    // println!("{:#?}", sphm);
    // println!("{:#?}", sphm.t().dot(&sphm));
    println!("{:#?}", sphm.t().dot(&sphm).inv()?);

    let res = (sphm.t().dot(&sphm).inv()?).dot(&(sphm.t())).dot(&out);
    println!("res: {:#?}", res);
    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        println!("{}", e);
        std::process::exit(1);
    }
}
