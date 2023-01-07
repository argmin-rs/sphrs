// Copyright 2018-2020 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::SphrsFloat;

/// SHCoordinates trait
pub trait SHCoordinates<T> {
    /// Return `theta`
    fn theta(&self) -> T;
    /// Return `phi`
    fn phi(&self) -> T;
    /// Return `r`
    fn r(&self) -> T;
    /// Return `x`
    fn x(&self) -> T;
    /// Return `y`
    fn y(&self) -> T;
    /// Return `z`
    fn z(&self) -> T;
    /// Return `cos(theta)`
    fn theta_cos(&self) -> T;
}

/// Coordinates struct
#[derive(Default, Clone, Debug)]
pub struct Coordinates<T> {
    /// radius (spherical coordinates)
    r: T,
    /// theta (spherical coordinates)
    theta: T,
    /// phi (spherical coordinates)
    phi: T,
    /// x (cartesian coordinates)
    x: T,
    /// y (cartesian coordinates)
    y: T,
    /// z (cartesian coordinates)
    z: T,
    /// cos(theta)
    theta_cos: T,
}

impl<T> Coordinates<T>
where
    T: SphrsFloat,
{
    /// Create `Coordinates` struct from cartesian coordinates
    pub fn cartesian(x: T, y: T, z: T) -> Self {
        let r = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
        let theta = (z / r).acos();
        let phi = y.atan2(x);

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

    /// Create `Coordinates` struct from spherical coordinates
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

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use quickcheck_macros::quickcheck;

    #[derive(Debug, Copy, Clone)]
    struct Radius(f64);

    impl quickcheck::Arbitrary for Radius {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            loop {
                let val = f64::arbitrary(g) % 100000.0;
                if !val.is_nan() && val.is_finite() {
                    return Radius(val);
                }
            }
        }
    }

    #[derive(Debug, Copy, Clone)]
    struct Angle(f64);

    impl quickcheck::Arbitrary for Angle {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            loop {
                let val = f64::arbitrary(g) % 2.0 * std::f64::consts::PI;
                if !val.is_nan() && val.is_finite() {
                    return Angle(val);
                }
            }
        }
    }

    #[derive(Debug, Copy, Clone)]
    struct Cartesian(f64);

    impl quickcheck::Arbitrary for Cartesian {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            loop {
                let val = f64::arbitrary(g) % 100000.0;
                if !val.is_nan() && val.is_finite() {
                    return Cartesian(val);
                }
            }
        }
    }

    #[quickcheck]
    fn shcoordinates_spherical_f64(r: Radius, theta: Angle, phi: Angle) {
        let r = r.0;
        let theta = theta.0;
        let phi = phi.0;
        let coords = Coordinates::spherical(r, theta, phi);
        assert_relative_eq!(coords.r(), r);
        assert_relative_eq!(coords.theta(), theta);
        assert_relative_eq!(coords.phi(), phi);
        assert_relative_eq!(coords.x(), r * theta.sin() * phi.cos());
        assert_relative_eq!(coords.y(), r * theta.sin() * phi.sin());
        assert_relative_eq!(coords.z(), r * theta.cos());
        assert_relative_eq!(coords.theta_cos(), theta.cos());
    }

    #[quickcheck]
    fn shcoordinates_spherical_f32(r: Radius, theta: Angle, phi: Angle) {
        let r = r.0 as f32;
        let theta = theta.0 as f32;
        let phi = phi.0 as f32;
        let coords = Coordinates::spherical(r, theta, phi);
        assert_relative_eq!(coords.r(), r);
        assert_relative_eq!(coords.theta(), theta);
        assert_relative_eq!(coords.phi(), phi);
        assert_relative_eq!(coords.x(), r * theta.sin() * phi.cos());
        assert_relative_eq!(coords.y(), r * theta.sin() * phi.sin());
        assert_relative_eq!(coords.z(), r * theta.cos());
        assert_relative_eq!(coords.theta_cos(), theta.cos());
    }

    #[quickcheck]
    fn shcoordinates_cartesian_f64(x: Cartesian, y: Cartesian, z: Cartesian) {
        let x = x.0;
        let y = y.0;
        let z = z.0;
        let r = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
        let theta = (z / r).acos();
        let phi = y.atan2(x);
        let theta_cos = theta.cos();

        let coords = Coordinates::cartesian(x, y, z);
        assert_relative_eq!(coords.x(), x);
        assert_relative_eq!(coords.y(), y);
        assert_relative_eq!(coords.z(), z);
        assert_relative_eq!(coords.r(), r);
        assert_relative_eq!(coords.theta(), theta);
        assert_relative_eq!(coords.phi(), phi);
        assert_relative_eq!(coords.theta_cos(), theta_cos);
    }

    #[quickcheck]
    fn shcoordinates_cartesian_f32(x: Cartesian, y: Cartesian, z: Cartesian) {
        let x = x.0 as f32;
        let y = y.0 as f32;
        let z = z.0 as f32;
        let r = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
        let theta = (z / r).acos();
        let phi = y.atan2(x);
        let theta_cos = theta.cos();

        let coords = Coordinates::cartesian(x, y, z);
        assert_relative_eq!(coords.x(), x);
        assert_relative_eq!(coords.y(), y);
        assert_relative_eq!(coords.z(), z);
        assert_relative_eq!(coords.r(), r);
        assert_relative_eq!(coords.theta(), theta);
        assert_relative_eq!(coords.phi(), phi);
        assert_relative_eq!(coords.theta_cos(), theta_cos);
    }
}
