// Copyright 2018-2020 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::SphrsFloat;

pub trait SHCoordinates<T>
where
    T: SphrsFloat,
{
    fn theta(&self) -> T;
    fn phi(&self) -> T;
    fn r(&self) -> T;
    fn x(&self) -> T;
    fn y(&self) -> T;
    fn z(&self) -> T;
    fn theta_cos(&self) -> T;
}

#[derive(Default, Clone, Debug)]
pub struct Coordinates<T>
where
    T: SphrsFloat,
{
    r: T,
    theta: T,
    phi: T,
    x: T,
    y: T,
    z: T,
    theta_cos: T,
}

impl<T> Coordinates<T>
where
    T: SphrsFloat,
{
    pub fn cartesian(x: T, y: T, z: T) -> Self {
        let r = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
        let theta = (z / r).acos();
        let phi = y.atan2(x);
        // let phi = if x.abs() < T::from_f64(10.0.powi(-15)).unwrap() {
        //     if (x.is_sign_negative() && y.is_sign_negative())
        //         || (x.is_sign_positive() && y.is_sign_positive())
        //     {
        //         T::PI() / T::from_f64(2.0).unwrap()
        //     } else {
        //         -T::PI() / T::from_f64(2.0).unwrap()
        //     }
        // } else {
        //     (y / x).atan()
        // };

        let theta_cos = theta.cos();
        Coordinates {
            r,
            theta,
            phi,
            x,
            y,
            z,
            theta_cos,
        }
    }

    pub fn spherical(r: T, theta: T, phi: T) -> Self {
        let x = r * theta.sin() * phi.cos();
        let y = r * theta.sin() * phi.sin();
        let theta_cos = theta.cos();
        let z = r * theta_cos;
        Coordinates {
            r,
            theta,
            phi,
            x,
            y,
            z,
            theta_cos,
        }
    }
}

impl<T> SHCoordinates<T> for Coordinates<T>
where
    T: SphrsFloat,
{
    #[inline]
    fn theta(&self) -> T {
        self.theta
    }

    #[inline]
    fn phi(&self) -> T {
        self.phi
    }

    #[inline]
    fn r(&self) -> T {
        self.r
    }

    #[inline]
    fn x(&self) -> T {
        self.x
    }

    #[inline]
    fn y(&self) -> T {
        self.y
    }

    #[inline]
    fn z(&self) -> T {
        self.z
    }

    #[inline]
    fn theta_cos(&self) -> T {
        self.theta_cos
    }
}
