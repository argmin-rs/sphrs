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
    // fn x2(&self) -> T;
    // fn y2(&self) -> T;
    // fn z2(&self) -> T;
}

#[derive(Default, Clone, Debug)]
pub struct SphericalCoordinates<T>
where
    T: Float + FloatConst + FromPrimitive,
{
    theta: T,
    phi: T,
    r: T,
}

impl<T> SphericalCoordinates<T>
where
    T: Float + FloatConst + FromPrimitive,
{
    pub fn new(theta: T, phi: T, r: T) -> Self {
        SphericalCoordinates { theta, phi, r }
    }
}

impl<T> Coordinates<T> for SphericalCoordinates<T>
where
    T: Float + FloatConst + FromPrimitive,
{
    #[inline(always)]
    fn theta(&self) -> T {
        self.theta
    }

    #[inline(always)]
    fn phi(&self) -> T {
        self.phi
    }

    #[inline(always)]
    fn r(&self) -> T {
        self.r
    }

    #[inline(always)]
    fn x(&self) -> T {
        self.r * self.theta.sin() * self.phi.cos()
    }

    #[inline(always)]
    fn y(&self) -> T {
        self.r * self.theta.sin() * self.phi.sin()
    }

    #[inline(always)]
    fn z(&self) -> T {
        self.r * self.theta.cos()
    }
}

#[derive(Default, Clone, Debug)]
pub struct CartesianCoordinates<T>
where
    T: Float + FloatConst + FromPrimitive,
{
    x: T,
    y: T,
    z: T,
}

impl<T> CartesianCoordinates<T>
where
    T: Float + FloatConst + FromPrimitive,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        CartesianCoordinates { x, y, z }
    }
}

impl<T> Coordinates<T> for CartesianCoordinates<T>
where
    T: Float + FloatConst + FromPrimitive,
{
    #[inline(always)]
    fn theta(&self) -> T {
        (self.z / (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()).acos()
    }

    #[inline(always)]
    fn phi(&self) -> T {
        (self.y / self.x).atan()
    }

    #[inline(always)]
    fn r(&self) -> T {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    #[inline(always)]
    fn x(&self) -> T {
        self.x
    }

    #[inline(always)]
    fn y(&self) -> T {
        self.y
    }

    #[inline(always)]
    fn z(&self) -> T {
        self.z
    }
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
        }
    }

    #[inline(always)]
    fn get_x(&self) -> T {
        self.x.get().unwrap()
    }

    #[inline(always)]
    fn get_y(&self) -> T {
        self.y.get().unwrap()
    }

    #[inline(always)]
    fn get_z(&self) -> T {
        self.z.get().unwrap()
    }

    #[inline(always)]
    fn get_r(&self) -> T {
        self.r.get().unwrap()
    }

    #[inline(always)]
    fn get_theta(&self) -> T {
        self.theta.get().unwrap()
    }

    #[inline(always)]
    fn get_phi(&self) -> T {
        self.phi.get().unwrap()
    }

    #[inline(always)]
    fn set_x(&self, x: T) {
        self.x.set(Some(x));
    }

    #[inline(always)]
    fn set_y(&self, y: T) {
        self.y.set(Some(y));
    }

    #[inline(always)]
    fn set_z(&self, z: T) {
        self.z.set(Some(z));
    }

    #[inline(always)]
    fn set_r(&self, r: T) {
        self.z.set(Some(r));
    }

    #[inline(always)]
    fn set_theta(&self, theta: T) {
        self.theta.set(Some(theta));
    }

    #[inline(always)]
    fn set_phi(&self, phi: T) {
        self.phi.set(Some(phi));
    }
}

impl<T> Coordinates<T> for GenCoordinates<T>
where
    T: Float + FloatConst + FromPrimitive,
{
    #[inline(always)]
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

    #[inline(always)]
    fn phi(&self) -> T {
        if let Some(phi) = self.phi.get() {
            phi
        } else {
            let phi = (self.get_y() / self.get_x()).atan();
            self.set_phi(phi);
            phi
        }
    }

    #[inline(always)]
    fn r(&self) -> T {
        if let Some(r) = self.r.get() {
            r
        } else {
            let r = (self.get_x().powi(2) + self.get_y().powi(2) + self.get_z().powi(2)).sqrt();
            self.set_r(r);
            r
        }
    }

    #[inline(always)]
    fn x(&self) -> T {
        if let Some(x) = self.x.get() {
            x
        } else {
            let x = self.get_r() * self.get_theta().sin() * self.get_phi().cos();
            self.set_x(x);
            x
        }
    }

    #[inline(always)]
    fn y(&self) -> T {
        if let Some(y) = self.y.get() {
            y
        } else {
            let y = self.get_r() * self.get_theta().sin() * self.get_phi().sin();
            self.set_y(y);
            y
        }
    }

    #[inline(always)]
    fn z(&self) -> T {
        if let Some(z) = self.z.get() {
            z
        } else {
            let z = self.get_r() * self.get_theta().cos();
            self.set_z(z);
            z
        }
    }
}
