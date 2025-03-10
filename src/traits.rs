pub trait Primitive {}
impl<T> Primitive for T where
    T: Copy
        + 'static
        + std::ops::Add<Output = Self>
        + std::ops::Sub<Output = Self>
        + std::ops::Mul<Output = Self>
        + std::ops::Div<Output = Self>
        + std::ops::Neg<Output = Self>
        + PartialOrd
{
}

pub trait FromPrimitive {
    type Output;

    fn from_u8(value: u8) -> Self::Output;
    fn from_u16(value: u16) -> Self::Output;
    fn from_u32(value: u32) -> Self::Output;
    fn from_u64(value: u64) -> Self::Output;
    fn from_u128(value: u128) -> Self::Output;

    fn from_i8(value: i8) -> Self::Output;
    fn from_i16(value: i16) -> Self::Output;
    fn from_i32(value: i32) -> Self::Output;
    fn from_i64(value: i64) -> Self::Output;
    fn from_i128(value: i128) -> Self::Output;
}

macro_rules! impl_from_primitive {
    ($($type: ty),*) => {
        $(
            impl FromPrimitive for $type {
                type Output = Self;

                fn from_u8(value: u8) -> Self::Output {
                    value as $type
                }
                fn from_u16(value: u16) -> Self::Output {
                    value as $type
                }
                fn from_u32(value: u32) -> Self::Output {
                    value as $type
                }
                fn from_u64(value: u64) -> Self::Output {
                    value as $type
                }
                fn from_u128(value: u128) -> Self::Output {
                    value as $type
                }

                fn from_i8(value: i8) -> Self::Output {
                    value as $type
                }
                fn from_i16(value: i16) -> Self::Output {
                    value as $type
                }
                fn from_i32(value: i32) -> Self::Output {
                    value as $type
                }
                fn from_i64(value: i64) -> Self::Output {
                    value as $type
                }
                fn from_i128(value: i128) -> Self::Output {
                    value as $type
                }
            }
        )*
    };
}

impl_from_primitive!(f32, f64);

pub trait ToPrimitive {
    type Input;

    fn to_u8(value: Self::Input) -> u8;
    fn to_u16(value: Self::Input) -> u16;
    fn to_u32(value: Self::Input) -> u32;
    fn to_u64(value: Self::Input) -> u64;
    fn to_u128(value: Self::Input) -> u128;

    fn to_i8(value: Self::Input) -> i8;
    fn to_i16(value: Self::Input) -> i16;
    fn to_i32(value: Self::Input) -> i32;
    fn to_i64(value: Self::Input) -> i64;
    fn to_i128(value: Self::Input) -> i128;
}

macro_rules! impl_to_primitive {
    ($($type: ty),*) => {
        $(
            impl ToPrimitive for $type {
                type Input = Self;

                fn to_u8(value: Self::Input) -> u8 {
                    value as u8
                }
                fn to_u16(value: Self::Input) -> u16 {
                    value as u16
                }
                fn to_u32(value: Self::Input) -> u32 {
                    value as u32
                }
                fn to_u64(value: Self::Input) -> u64 {
                    value as u64
                }
                fn to_u128(value: Self::Input) -> u128 {
                    value as u128
                }

                fn to_i8(value: Self::Input) -> i8 {
                    value as i8
                }
                fn to_i16(value: Self::Input) -> i16 {
                    value as i16
                }
                fn to_i32(value: Self::Input) -> i32 {
                    value as i32
                }
                fn to_i64(value: Self::Input) -> i64 {
                    value as i64
                }
                fn to_i128(value: Self::Input) -> i128 {
                    value as i128
                }
            }
        )*
    };
}

impl_to_primitive!(f32, f64);

pub trait Float: Primitive + ToPrimitive<Input = Self> + FromPrimitive<Output = Self> {
    // Constants
    const PI: Self;
    const E: Self;
    const EPSILON: Self;
    const INFINITY: Self;
    const NEG_INFINITY: Self;
    const NAN: Self;

    // Basic checks and operations
    fn is_nan(self) -> bool;
    fn is_infinite(self) -> bool;
    fn is_finite(self) -> bool;
    fn is_sign_positive(self) -> bool;
    fn is_sign_negative(self) -> bool;
    fn abs(self) -> Self;
    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;

    // Mathematical operations
    fn sqrt(self) -> Self;
    fn exp(self) -> Self;
    fn ln(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn powi(self, n: i32) -> Self;
    fn powf(self, n: Self) -> Self;

    // Conversion from any primitive
    fn from_u8(value: u8) -> Self;

    // Conversion to f32/f64
    fn to_f32(self) -> f32;
    fn to_f64(self) -> f64;
}

macro_rules! impl_float {
    ($type:ty, $pi:expr, $e:expr, $epsilon:expr, $infinity:expr, $neg_infinity:expr, $nan:expr) => {
        impl Float for $type {
            // Constants
            const PI: Self = $pi;
            const E: Self = $e;
            const EPSILON: Self = $epsilon;
            const INFINITY: Self = $infinity;
            const NEG_INFINITY: Self = $neg_infinity;
            const NAN: Self = $nan;

            // Basic checks and operations
            fn is_nan(self) -> bool {
                self.is_nan()
            }
            fn is_infinite(self) -> bool {
                self.is_infinite()
            }
            fn is_finite(self) -> bool {
                self.is_finite()
            }
            fn is_sign_positive(self) -> bool {
                self.is_sign_positive()
            }
            fn is_sign_negative(self) -> bool {
                self.is_sign_negative()
            }
            fn abs(self) -> Self {
                self.abs()
            }
            fn max(self, other: Self) -> Self {
                self.max(other)
            }
            fn min(self, other: Self) -> Self {
                self.min(other)
            }

            // Mathematical operations
            fn sqrt(self) -> Self {
                self.sqrt()
            }
            fn exp(self) -> Self {
                self.exp()
            }
            fn ln(self) -> Self {
                self.ln()
            }
            fn sin(self) -> Self {
                self.sin()
            }
            fn cos(self) -> Self {
                self.cos()
            }
            fn tan(self) -> Self {
                self.tan()
            }
            fn powi(self, n: i32) -> Self {
                self.powi(n)
            }
            fn powf(self, n: Self) -> Self {
                self.powf(n)
            }

            // Conversions
            fn from_u8(value: u8) -> Self {
                value as $type
            }

            fn to_f32(self) -> f32 {
                self as f32
            }
            fn to_f64(self) -> f64 {
                self as f64
            }
        }
    };
}

// Implement for f32
impl_float!(
    f32,
    std::f32::consts::PI,
    std::f32::consts::E,
    f32::EPSILON,
    f32::INFINITY,
    f32::NEG_INFINITY,
    f32::NAN
);

// Implement for f64
impl_float!(
    f64,
    std::f64::consts::PI,
    std::f64::consts::E,
    f64::EPSILON,
    f64::INFINITY,
    f64::NEG_INFINITY,
    f64::NAN
);
