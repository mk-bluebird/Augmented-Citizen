// crates/brain-identity-kernel/src/fixed.rs
// SPDX-License-Identifier: MIT OR Apache-2.0

#![forbid(unsafe_code)]

//! Fixed-point arithmetic for deterministic neural computations.
//!
//! Uses Q16.16 format (16 integer bits, 16 fractional bits) for
//! predictable, overflow-checked arithmetic without floating-point.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fx(pub i32);

impl Fx {
    pub const ONE: Self = Fx(65536);
    pub const ZERO: Self = Fx(0);

    pub fn from_f32(v: f32) -> Self {
        Fx((v * 65536.0).round() as i32)
    }

    pub fn to_f32(self) -> f32 {
        self.0 as f32 / 65536.0
    }

    pub fn from_raw(raw: i32) -> Self {
        Fx(raw)
    }

    pub fn to_raw(self) -> i32 {
        self.0
    }

    pub fn zero() -> Self {
        Fx::ZERO
    }

    pub fn one() -> Self {
        Fx::ONE
    }

    pub fn checked_add(self, other: Fx) -> Option<Fx> {
        self.0.checked_add(other.0).map(Fx)
    }

    pub fn checked_sub(self, other: Fx) -> Option<Fx> {
        self.0.checked_sub(other.0).map(Fx)
    }

    pub fn checked_mul(self, other: Fx) -> Option<Fx> {
        // Q16.16 * Q16.16 = Q32.32, then shift right by 16 to get Q16.16
        let prod = (self.0 as i64) * (other.0 as i64);
        let shifted = prod >> 16;
        if shifted > i32::MAX as i64 || shifted < i32::MIN as i64 {
            None
        } else {
            Some(Fx(shifted as i32))
        }
    }

    pub fn checked_div(self, other: Fx) -> Option<Fx> {
        if other.0 == 0 {
            return None;
        }
        // Q16.16 / Q16.16: shift numerator left by 16 first
        let num = (self.0 as i64) << 16;
        let result = num / (other.0 as i64);
        if result > i32::MAX as i64 || result < i32::MIN as i64 {
            None
        } else {
            Some(Fx(result as i32))
        }
    }

    pub fn saturating_add(self, other: Fx) -> Fx {
        Fx(self.0.saturating_add(other.0))
    }

    pub fn saturating_sub(self, other: Fx) -> Fx {
        Fx(self.0.saturating_sub(other.0))
    }

    pub fn saturating_mul(self, other: Fx) -> Fx {
        let prod = (self.0 as i64) * (other.0 as i64);
        let shifted = prod >> 16;
        Fx(shifted.clamp(i32::MIN as i64, i32::MAX as i64) as i32)
    }
}

impl Default for Fx {
    fn default() -> Self {
        Fx::ZERO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_to_f32() {
        let v = 0.5f32;
        let fx = Fx::from_f32(v);
        assert!((fx.to_f32() - v).abs() < 0.0001);
    }

    #[test]
    fn test_addition() {
        let a = Fx::from_f32(0.25);
        let b = Fx::from_f32(0.75);
        let sum = a.checked_add(b).unwrap();
        assert_eq!(sum.to_f32(), 1.0);
    }

    #[test]
    fn test_multiplication() {
        let a = Fx::from_f32(2.0);
        let b = Fx::from_f32(3.0);
        let prod = a.checked_mul(b).unwrap();
        assert_eq!(prod.to_f32(), 6.0);
    }
}
