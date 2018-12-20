// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::cell::Cell;

pub trait Coordinates {
    fn theta(&self) -> f64;
    fn phi(&self) -> f64;
    fn r(&self) -> f64;
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    // fn x2(&self) -> f64;
    // fn y2(&self) -> f64;
    // fn z2(&self) -> f64;
}

#[derive(Default, Clone, Debug)]
pub struct SphericalCoordinates {
    theta: f64,
    phi: f64,
    r: f64,
}

impl SphericalCoordinates {
    pub fn new(theta: f64, phi: f64, r: f64) -> Self {
        SphericalCoordinates { theta, phi, r }
    }
}

impl Coordinates for SphericalCoordinates {
    #[inline(always)]
    fn theta(&self) -> f64 {
        self.theta
    }

    #[inline(always)]
    fn phi(&self) -> f64 {
        self.phi
    }

    #[inline(always)]
    fn r(&self) -> f64 {
        self.r
    }

    #[inline(always)]
    fn x(&self) -> f64 {
        self.r * self.theta.sin() * self.phi.cos()
    }

    #[inline(always)]
    fn y(&self) -> f64 {
        self.r * self.theta.sin() * self.phi.sin()
    }

    #[inline(always)]
    fn z(&self) -> f64 {
        self.r * self.theta.cos()
    }
}

#[derive(Default, Clone, Debug)]
pub struct CartesianCoordinates {
    x: f64,
    y: f64,
    z: f64,
}

impl CartesianCoordinates {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        CartesianCoordinates { x, y, z }
    }
}

impl Coordinates for CartesianCoordinates {
    #[inline(always)]
    fn theta(&self) -> f64 {
        (self.z / (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()).acos()
    }

    #[inline(always)]
    fn phi(&self) -> f64 {
        (self.y / self.x).atan()
    }

    #[inline(always)]
    fn r(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    #[inline(always)]
    fn x(&self) -> f64 {
        self.x
    }

    #[inline(always)]
    fn y(&self) -> f64 {
        self.y
    }

    #[inline(always)]
    fn z(&self) -> f64 {
        self.z
    }
}

#[derive(Default, Clone, Debug)]
pub struct GenCoordinates {
    r: Cell<Option<f64>>,
    theta: Cell<Option<f64>>,
    phi: Cell<Option<f64>>,
    x: Cell<Option<f64>>,
    y: Cell<Option<f64>>,
    z: Cell<Option<f64>>,
}

impl GenCoordinates {
    pub fn new() -> Self {
        GenCoordinates::default()
    }

    pub fn cartesian(x: f64, y: f64, z: f64) -> Self {
        GenCoordinates {
            r: Cell::new(None),
            theta: Cell::new(None),
            phi: Cell::new(None),
            x: Cell::new(Some(x)),
            y: Cell::new(Some(y)),
            z: Cell::new(Some(z)),
        }
    }

    pub fn spherical(r: f64, theta: f64, phi: f64) -> Self {
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
    fn get_x(&self) -> f64 {
        self.x.get().unwrap()
    }

    #[inline(always)]
    fn get_y(&self) -> f64 {
        self.y.get().unwrap()
    }

    #[inline(always)]
    fn get_z(&self) -> f64 {
        self.z.get().unwrap()
    }

    #[inline(always)]
    fn get_r(&self) -> f64 {
        self.r.get().unwrap()
    }

    #[inline(always)]
    fn get_theta(&self) -> f64 {
        self.theta.get().unwrap()
    }

    #[inline(always)]
    fn get_phi(&self) -> f64 {
        self.phi.get().unwrap()
    }

    #[inline(always)]
    fn set_x(&self, x: f64) {
        self.x.set(Some(x));
    }

    #[inline(always)]
    fn set_y(&self, y: f64) {
        self.y.set(Some(y));
    }

    #[inline(always)]
    fn set_z(&self, z: f64) {
        self.z.set(Some(z));
    }

    #[inline(always)]
    fn set_r(&self, r: f64) {
        self.z.set(Some(r));
    }

    #[inline(always)]
    fn set_theta(&self, theta: f64) {
        self.theta.set(Some(theta));
    }

    #[inline(always)]
    fn set_phi(&self, phi: f64) {
        self.phi.set(Some(phi));
    }
}

impl Coordinates for GenCoordinates {
    #[inline(always)]
    fn theta(&self) -> f64 {
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
    fn phi(&self) -> f64 {
        if let Some(phi) = self.phi.get() {
            phi
        } else {
            let phi = (self.get_y() / self.get_x()).atan();
            self.set_phi(phi);
            phi
        }
    }

    #[inline(always)]
    fn r(&self) -> f64 {
        if let Some(r) = self.r.get() {
            r
        } else {
            let r = (self.get_x().powi(2) + self.get_y().powi(2) + self.get_z().powi(2)).sqrt();
            self.set_r(r);
            r
        }
    }

    #[inline(always)]
    fn x(&self) -> f64 {
        if let Some(x) = self.x.get() {
            x
        } else {
            let x = self.get_r() * self.get_theta().sin() * self.get_phi().cos();
            self.set_x(x);
            x
        }
    }

    #[inline(always)]
    fn y(&self) -> f64 {
        if let Some(y) = self.y.get() {
            y
        } else {
            let y = self.get_r() * self.get_theta().sin() * self.get_phi().sin();
            self.set_y(y);
            y
        }
    }

    #[inline(always)]
    fn z(&self) -> f64 {
        if let Some(z) = self.z.get() {
            z
        } else {
            let z = self.get_r() * self.get_theta().cos();
            self.set_z(z);
            z
        }
    }
}
