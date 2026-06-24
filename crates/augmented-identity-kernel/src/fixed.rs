// Minimal Q16.16 fixed-point wrapper reusing the same pattern as brain-identity-kernel.

use core::ops::{Add, Sub, Mul};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Fx(pub i32);

impl Fx {
    pub const ZERO: Fx = Fx(0);
    pub const ONE: Fx = Fx(1 << 16);

    pub fn from_f32(x: f32) -> Fx {
        let v = (x * (1i32 << 16) as f32).round() as i32;
        Fx(v)
    }

    pub fn to_f32(self) -> f32 {
        (self.0 as f32) / ((1i32 << 16) as f32)
    }
}

impl Add for Fx {
    type Output = Fx;
    fn add(self, rhs: Fx) -> Fx {
        Fx(self.0.wrapping_add(rhs.0))
    }
}

impl Sub for Fx {
    type Output = Fx;
    fn sub(self, rhs: Fx) -> Fx {
        Fx(self.0.wrapping_sub(rhs.0))
    }
}

impl Mul for Fx {
    type Output = Fx;
    fn mul(self, rhs: Fx) -> Fx {
        let a = self.0 as i64;
        let b = rhs.0 as i64;
        let prod = (a * b) >> 16;
        Fx(prod as i32)
    }
}

impl Fx {
    pub fn sq(self) -> Fx {
        self * self
    }

    pub fn abs(self) -> Fx {
        if self.0 < 0 { Fx(-self.0) } else { self }
    }

    pub fn max(self, other: Fx) -> Fx {
        if self.0 >= other.0 { self } else { other }
    }

    pub fn min(self, other: Fx) -> Fx {
        if self.0 <= other.0 { self } else { other }
    }
}
