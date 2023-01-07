// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

// use ndarray::Array1;
// use ndarray_linalg::Inverse;
// use sphrs::*;
// use std::error::Error;
// // use std::f64::consts::PI;
//
// fn run() -> Result<(), Box<dyn Error>> {
//     let mut fu = Vec::with_capacity(8 * 8 * 8);
//     let pos = vec![
//         -2.0f64, -1.75, -1.5, -1.25, -1.0, -0.75, -0.5, -0.25, 0.25, 0.5, 0.75, 1.0, 1.25, 1.50,
//         1.75, 2.0,
//     ];
//     // let pos = vec![-0.75, -0.5, -0.25, 0.25, 0.5, 0.75];
//     for i in &pos {
//         for j in &pos {
//             for k in &pos {
//                 let p = Coordinates::cartesian(*i, *j, *k);
//                 let p = p.finalize();
//                 fu.push(p);
//             }
//         }
//     }
//     // let theta = vec![0.0, PI / 4.0, 2.0 * PI / 4.0, 3.0 * PI / 4.0];
//     // for i in &theta {
//     //     for j in &theta {
//     //         let p = Coordinates::spherical(1.0, *i, *j).finalize();
//     //         fu.push(p);
//     //     }
//     // }
//
//     let sh_type = RealSH::RegularSolid;
//
//     let target = RealSphericalHarmonics::new(1, sh_type);
//     // target.set_coeffs(vec![0.1, 2.0, 8.9, 3.2]);
//
//     let out = Array1::from(target.eval_vec(&fu));
//     // println!("out:\n{:#?}", out);
//
//     let sphm = sph_mat(1, &fu, sh_type);
//     // println!("{:#?}", sphm);
//     // println!("{:#?}", sphm.t().dot(&sphm));
//     println!("{:#?}", sphm.t().dot(&sphm).inv()?);
//
//     let res = (sphm.t().dot(&sphm).inv()?).dot(&(sphm.t())).dot(&out);
//     println!("res: {:#?}", res);
//     Ok(())
// }
//
fn main() {
    // if let Err(ref e) = run() {
    //     println!("{}", e);
    //     std::process::exit(1);
    // }
}

// use ndarray::{s, Array1, Array2};
// pub fn sph_mat<
//     'a,
//     T: 'a + Float + FromPrimitive + FloatConst + AddAssign + std::iter::Sum + Debug,
// >(
//     order: usize,
//     pos: &[impl SHCoordinates<T>],
//     sh_type: RealSH,
// ) -> Array2<T> {
//     let sh = RealSphericalHarmonics::new(order, sh_type);
//     let mut mat = unsafe { Array2::uninitialized((pos.len(), sh.num_sh)) };
//     for (i, item) in pos.iter().enumerate() {
//         mat.slice_mut(s![i, ..])
//             .assign(&Array1::from(sh.eval_indiv_plain(item)));
//     }
//     mat
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::f64::consts::PI;
//
//     #[test]
//     fn sph_mat_test() {
//         let p1 = Coordinates::spherical(1.0, PI / 2.0, 0.0);
//         let p1 = p1.finalize();
//         let p2 = Coordinates::spherical(0.7, PI / 4.0, 0.0);
//         let p2 = p2.finalize();
//         let fu = vec![&p1, &p2];
//         let bla = sph_mat(1, &fu);
//         println!("{:#?}", bla);
//
//         // assert!((real_SH(2, 1, &p) - sh2p1(&p)) < std::f64::EPSILON);
//         // assert!((real_SH(3, -2, &p) - sh3n2(&p)) < std::f64::EPSILON);
//     }
// }
