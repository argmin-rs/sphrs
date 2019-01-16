// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! todo

use ndarray_linalg::Inverse;
use sphrs::*;
use std::error::Error;

fn run() -> Result<(), Box<Error>> {
    let mut fu = Vec::with_capacity(10 * 10 * 10);
    for i in -2..0 {
        for j in -2..0 {
            for k in -2..0 {
                let p = GenCoordinates::cartesian(i as f64, j as f64, k as f64);
                fu.push(p);
            }
        }
    }

    let bla = sph_mat(1, &fu);
    println!("{:#?}", bla);
    println!("{:#?}", bla.t().dot(&bla));
    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        println!("{}", e);
        std::process::exit(1);
    }
}
