pub mod angle;
pub mod length;
pub mod time;

pub use angle::*;
pub use length::*;
pub use time::*;

macro_rules! unit {
    ($unit_ty:ident, $unit_tr:ident, $unit_val:ident) => {
		#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
        pub struct $unit_ty {
            val: f64,
        }

        impl $unit_ty {
            pub const fn from<Unit: $unit_tr>(value: f64) -> Self {
                Self {
                    val: value * Unit::$unit_val,
                }
            }

            pub const fn to<Unit: $unit_tr>(&self) -> f64 {
                self.val / Unit::$unit_val
            }
        }

		impl std::ops::Add<$unit_ty> for $unit_ty {
			type Output = $unit_ty;

			fn add(self, other: Self) -> Self {
				Self {
					val: self.val + other.val
				}
			}
		}

		impl std::ops::Sub<$unit_ty> for $unit_ty {
			type Output = $unit_ty;

			fn sub(self, other: Self) -> Self {
				Self {
					val: self.val - other.val
				}
			}
		}

		impl std::ops::Mul<f64> for $unit_ty {
			type Output = Self;

			fn mul(self, other:f64) -> Self {
				Self {
					val: self.val * other
				}
			}
		}

		impl std::ops::Div<f64> for $unit_ty {
			type Output = Self;

			fn div(self, other:f64) -> Self {
				Self {
					val: self.val / other
				}
			}
		}

		impl std::ops::Div<$unit_ty> for $unit_ty {
			type Output = f64;

			fn div(self, other:Self) -> f64 {
				self.val / other.val
			}
		}
    };

	($($unit_ty:ident, $unit_tr:ident, $unit_val:ident);+;) => {
		$(
			unit!($unit_ty, $unit_tr, $unit_val);
		)+
	};
}

macro_rules! compound_unit {
    ($unit_ty:ident, $ty1:ident, $ty2:ident, $tr1:ident, $tr2:ident, $un1:ident, $un2:ident, $to: expr) => {
        pub struct $unit_ty {
            val_a: $ty1,
            val_b: $ty2,
        }

        impl $unit_ty {
            pub const fn from<A: $tr1, B: $tr2>(a: f64, b: f64) -> Self {
                Self {
                    val_a: $ty1::from::<A>(a),
                    val_b: $ty2::from::<B>(b),
                }
            }

			pub const fn from_one<A: $tr1, B: $tr2>(a: f64) -> Self {
				Self::from::<A, B>(a, 1.0)
			}

            pub const fn to<A: $tr1, B: $tr2>(&self) -> f64 {
				$to(self.val_a.to::<A>(), self.val_b.to::<B>())
            }
        }
    };

	($($unit_ty:ident, $ty1:ident, $ty2:ident, $tr1:ident, $tr2:ident, $un1:ident, $un2:ident, $to: expr);+;) => {
		$(
			compound_unit!($unit_ty, $ty1, $ty2, $tr1, $tr2, $un1, $un2, $to);
		)+
	}
}

unit! {
    Length, LengthUnit, MM_PER_UNIT;
    Angle, AngularUnit, RAD_PER_UNIT;
    Time, TimeUnit, SEC_PER_UNIT;
}

compound_unit! {
    Velocity, Length, Time, LengthUnit, TimeUnit, MM_PER_UNIT, SEC_PER_UNIT, |a, b| a / b;
    AngVelocity, Angle, Time, AngularUnit, TimeUnit, RAD_PER_UNIT, SEC_PER_UNIT, |a, b| a / b;
}
