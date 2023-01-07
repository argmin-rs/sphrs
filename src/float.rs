// Copyright 2018-2020 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use num::{Float, FromPrimitive};
use num_traits::float::FloatConst;
use std::fmt::Debug;

/// Trait alias to simplify common trait bounds
pub trait SphrsFloat: Float + FloatConst + FromPrimitive + Debug {}
impl<I> SphrsFloat for I where I: Float + FloatConst + FromPrimitive + Debug {}
