use super::{Pow, Prec, Rt, SinhCosh, SpecialValuesDeci};
use crate::macros::impls::{dec_c_impl, impl_cneg, impl_self_c_ops};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CF64(pub f64, pub f64);

impl Prec for CF64 {
    fn prec(&self) -> u32 {
        64
    }
    fn set_prec(&mut self, _: u32) {}
}

impl From<f64> for CF64 {
    fn from(value: f64) -> Self {
        Self(value, 0.0)
    }
}

impl From<(f64, f64)> for CF64 {
    fn from((a, b): (f64, f64)) -> Self {
        Self(a, b)
    }
}

impl Display for CF64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}+{}i", self.0, self.1)
    }
}

dec_c_impl!(CF64, f64, |_, x| x as f64);
impl_cneg!(CF64, CF64);
impl_self_c_ops!(CF64);
