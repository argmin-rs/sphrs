// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use num::{Float, FromPrimitive};
use num_traits::FloatConst;
use std::cell::Cell;

pub trait Coordinates<T>
where
    T: Float + FloatConst + FromPrimitive,
{
    fn theta(&self) -> T;
    fn phi(&self) -> T;
    fn r(&self) -> T;
    fn x(&self) -> T;
    fn y(&self) -> T;
    fn z(&self) -> T;
    fn theta_cos(&self) -> T;
    // fn x2(&self) -> T;
    // fn y2(&self) -> T;
    // fn z2(&self) -> T;
}

#[derive(Default, Clone, Debug)]
pub struct GenCoordinates<T>
where
    T: Float + FloatConst + FromPrimitive,
{
    r: Cell<Option<T>>,
    theta: Cell<Option<T>>,
    phi: Cell<Option<T>>,
    x: Cell<Option<T>>,
    y: Cell<Option<T>>,
    z: Cell<Option<T>>,
    theta_cos: Cell<Option<T>>,
}

#[derive(Default, Clone, Debug)]
pub struct GenCoordinatesFinal<T>
where
    T: Float + FloatConst + FromPrimitive,
{
    r: T,
    theta: T,
    phi: T,
    x: T,
    y: T,
    z: T,
    theta_cos: T,
}

impl<T> GenCoordinates<T>
where
    T: Float + FloatConst + FromPrimitive,
{
    pub fn cartesian(x: T, y: T, z: T) -> Self {
        GenCoordinates {
            r: Cell::new(None),
            theta: Cell::new(None),
            phi: Cell::new(None),
            x: Cell::new(Some(x)),
            y: Cell::new(Some(y)),
            z: Cell::new(Some(z)),
            theta_cos: Cell::new(None),
        }
    }

    pub fn spherical(r: T, theta: T, phi: T) -> Self {
        GenCoordinates {
            r: Cell::new(Some(r)),
            theta: Cell::new(Some(theta)),
            phi: Cell::new(Some(phi)),
            x: Cell::new(None),
            y: Cell::new(None),
            z: Cell::new(None),
            theta_cos: Cell::new(None),
        }
    }

    pub fn finalize(&self) -> GenCoordinatesFinal<T> {
        if self.r.get().is_some() {
            self.x();
            self.y();
            self.z();
        } else {
            self.r();
            self.theta();
            self.phi();
        }

        self.theta_cos();

        GenCoordinatesFinal {
            r: self.get_r(),
            theta: self.get_theta(),
            phi: self.get_phi(),
            x: self.get_x(),
            y: self.get_y(),
            z: self.get_z(),
            theta_cos: self.get_theta_cos(),
        }
    }

    #[inline]
    fn get_x(&self) -> T {
        self.x.get().unwrap()
    }

    #[inline]
    fn get_y(&self) -> T {
        self.y.get().unwrap()
    }

    #[inline]
    fn get_z(&self) -> T {
        self.z.get().unwrap()
    }

    #[inline]
    fn get_r(&self) -> T {
        self.r.get().unwrap()
    }

    #[inline]
    fn get_theta(&self) -> T {
        self.theta.get().unwrap()
    }

    #[inline]
    fn get_phi(&self) -> T {
        self.phi.get().unwrap()
    }

    #[inline]
    fn get_theta_cos(&self) -> T {
        self.theta_cos.get().unwrap()
    }

    #[inline]
    fn set_x(&self, x: T) {
        self.x.set(Some(x));
    }

    #[inline]
    fn set_y(&self, y: T) {
        self.y.set(Some(y));
    }

    #[inline]
    fn set_z(&self, z: T) {
        self.z.set(Some(z));
    }

    #[inline]
    fn set_r(&self, r: T) {
        self.r.set(Some(r));
    }

    #[inline]
    fn set_theta(&self, theta: T) {
        self.theta.set(Some(theta));
    }

    #[inline]
    fn set_phi(&self, phi: T) {
        self.phi.set(Some(phi));
    }

    #[inline]
    fn set_theta_cos(&self, theta_cos: T) {
        self.theta_cos.set(Some(theta_cos));
    }
}

impl<T> Coordinates<T> for GenCoordinates<T>
where
    T: Float + FloatConst + FromPrimitive,
{
    #[inline]
    fn theta(&self) -> T {
        if let Some(theta) = self.theta.get() {
            theta
        } else {
            let theta = (self.get_z()
                / (self.get_x().powi(2) + self.get_y().powi(2) + self.get_z().powi(2)).sqrt())
            .acos();
            self.set_theta(theta);
            theta
        }
    }

    #[inline]
    fn phi(&self) -> T {
        if let Some(phi) = self.phi.get() {
            phi
        } else {
            let phi = (self.get_y() / self.get_x()).atan();
            self.set_phi(phi);
            phi
        }
    }

    #[inline]
    fn r(&self) -> T {
        if let Some(r) = self.r.get() {
            r
        } else {
            let r = (self.get_x().powi(2) + self.get_y().powi(2) + self.get_z().powi(2)).sqrt();
            self.set_r(r);
            r
        }
    }

    #[inline]
    fn x(&self) -> T {
        if let Some(x) = self.x.get() {
            x
        } else {
            let x = self.get_r() * self.get_theta().sin() * self.get_phi().cos();
            self.set_x(x);
            x
        }
    }

    #[inline]
    fn y(&self) -> T {
        if let Some(y) = self.y.get() {
            y
        } else {
            let y = self.get_r() * self.get_theta().sin() * self.get_phi().sin();
            self.set_y(y);
            y
        }
    }

    #[inline]
    fn z(&self) -> T {
        if let Some(z) = self.z.get() {
            z
        } else {
            let z = self.get_r() * self.get_theta().cos();
            self.set_z(z);
            z
        }
    }

    #[inline]
    fn theta_cos(&self) -> T {
        if let Some(theta_cos) = self.theta_cos.get() {
            theta_cos
        } else {
            let theta_cos = self.get_theta().cos();
            self.set_theta_cos(theta_cos);
            theta_cos
        }
    }
}

impl<T> Coordinates<T> for GenCoordinatesFinal<T>
where
    T: Float + FloatConst + FromPrimitive,
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
