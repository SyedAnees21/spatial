/// A generic trait which represents rust primitive numbers type. This
/// trait acts as the basic bounds requirement by main `Float` trait.
/// 
/// It offers the blanket implementation over rust primitive numbers.
pub trait Primitive: Copy
    + 'static
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + PartialOrd
    + Sized
{}

/// This is the blanket implementation of `primitive` trait on types which
/// implements basic aithematic and logical tratis.
impl<T> Primitive for T where
    T: Copy
        + 'static
        + std::ops::Add<Output = Self>
        + std::ops::Sub<Output = Self>
        + std::ops::Mul<Output = Self>
        + std::ops::Div<Output = Self>
        + std::ops::Neg<Output = Self>
        + PartialOrd
        + Sized
{
}

/// A generic trait implementation for converting to the underlying type
/// in our case `f32`/`f64` types from rust's primitive number types.
///
/// Note: Do not use these methods, if there is a risk of lossy conversions.
pub trait FromPrimitive {
    /// Associated type resulting after the conversion.
    type Output;

    /// Casts the `u8` value type to underlying float
    /// type `f32`/`f64`
    fn from_u8(value: u8) -> Self::Output;

    /// Casts the `u16` value type to underlying float
    /// type `f32`/`f64`
    fn from_u16(value: u16) -> Self::Output;

    /// Casts the `u32` value type to underlying float
    /// type `f32`/`f64`
    fn from_u32(value: u32) -> Self::Output;

    /// Casts the `u64` value type to underlying float
    /// type `f32`/`f64`
    fn from_u64(value: u64) -> Self::Output;

    /// Casts the `u128` value type to underlying float
    /// type `f32`/`f64`
    fn from_u128(value: u128) -> Self::Output;

    /// Casts the `i8` value type to underlying float
    /// type `f32`/`f64`
    fn from_i8(value: i8) -> Self::Output;

    /// Casts the `i16` value type to underlying float
    /// type `f32`/`f64`
    fn from_i16(value: i16) -> Self::Output;

    /// Casts the `i32` value type to underlying float
    /// type `f32`/`f64`
    fn from_i32(value: i32) -> Self::Output;

    /// Casts the `i64` value type to underlying float
    /// type `f32`/`f64`
    fn from_i64(value: i64) -> Self::Output;

    /// Casts the `i128` value type to underlying float
    /// type `f32`/`f64`
    fn from_i128(value: i128) -> Self::Output;
}

macro_rules! impl_from_primitive {
    ($($type: ty),*) => {
        $(
            impl FromPrimitive for $type {
                type Output = Self;

                #[inline(always)]
                fn from_u8(value: u8) -> Self::Output {
                    value as $type
                }
                #[inline(always)]
                fn from_u16(value: u16) -> Self::Output {
                    value as $type
                }
                #[inline(always)]
                fn from_u32(value: u32) -> Self::Output {
                    value as $type
                }
                #[inline(always)]
                fn from_u64(value: u64) -> Self::Output {
                    value as $type
                }
                #[inline(always)]
                fn from_u128(value: u128) -> Self::Output {
                    value as $type
                }

                #[inline(always)]
                fn from_i8(value: i8) -> Self::Output {
                    value as $type
                }
                #[inline(always)]
                fn from_i16(value: i16) -> Self::Output {
                    value as $type
                }
                #[inline(always)]
                fn from_i32(value: i32) -> Self::Output {
                    value as $type
                }
                #[inline(always)]
                fn from_i64(value: i64) -> Self::Output {
                    value as $type
                }
                #[inline(always)]
                fn from_i128(value: i128) -> Self::Output {
                    value as $type
                }
            }
        )*
    };
}

impl_from_primitive!(f32, f64);

/// A generic trait implementation for converting to the rust's primitive
/// ints types from underlysing float types `f32`/`f64`.
///
/// Note: Do not use these methods, if there is a risk of lossy conversions.
pub trait ToPrimitive {
    /// Associative input type to be converted.
    type Input;

    /// Casts the `Float` value type to `u8`.
    fn to_u8(value: Self::Input) -> u8;

    /// Casts the `Float` value type to `u16`.
    fn to_u16(value: Self::Input) -> u16;

    /// Casts the `Float` value type to `u32`.
    fn to_u32(value: Self::Input) -> u32;

    /// Casts the `Float` value type to `u64`.
    fn to_u64(value: Self::Input) -> u64;

    /// Casts the `Float` value type to `u128`.
    fn to_u128(value: Self::Input) -> u128;

    /// Casts the `Float` value type to `i8`.
    fn to_i8(value: Self::Input) -> i8;

    /// Casts the `Float` value type to `i16`.
    fn to_i16(value: Self::Input) -> i16;

    /// Casts the `Float` value type to `i32`.
    fn to_i32(value: Self::Input) -> i32;

    /// Casts the `Float` value type to `i64`.
    fn to_i64(value: Self::Input) -> i64;

    /// Casts the `Float` value type to `i128`.
    fn to_i128(value: Self::Input) -> i128;
}

macro_rules! impl_to_primitive {
    ($($type: ty),*) => {
        $(
            impl ToPrimitive for $type {
                type Input = Self;

                #[inline(always)]
                fn to_u8(value: Self::Input) -> u8 {
                    value as u8
                }
                #[inline(always)]
                fn to_u16(value: Self::Input) -> u16 {
                    value as u16
                }
                #[inline(always)]
                fn to_u32(value: Self::Input) -> u32 {
                    value as u32
                }
                #[inline(always)]
                fn to_u64(value: Self::Input) -> u64 {
                    value as u64
                }
                #[inline(always)]
                fn to_u128(value: Self::Input) -> u128 {
                    value as u128
                }

                #[inline(always)]
                fn to_i8(value: Self::Input) -> i8 {
                    value as i8
                }
                #[inline(always)]
                fn to_i16(value: Self::Input) -> i16 {
                    value as i16
                }
                #[inline(always)]
                fn to_i32(value: Self::Input) -> i32 {
                    value as i32
                }
                #[inline(always)]
                fn to_i64(value: Self::Input) -> i64 {
                    value as i64
                }
                #[inline(always)]
                fn to_i128(value: Self::Input) -> i128 {
                    value as i128
                }
            }
        )*
    };
}

impl_to_primitive!(f32, f64);

/// This is the main `Float` trait implementation for rust's single and double precision
/// float types `f32`/`f64`. 
/// 
/// This trait is kept as minimal as it could while it provides enough arithematic and
/// logical functionalities to be used in generics in-place of core types `f32`/`f64`.
/// 
/// This trait requires `Primitive`, `ToPrimitive` and `FromPrimitive` trait bounds. 
pub trait Float: Primitive + ToPrimitive<Input = Self> + FromPrimitive<Output = Self> {
    // Associative constans exposure

    /// Smallest finite value representable by the
    /// underlying float type.
    /// 
    /// - In case of f32, its `-3.40282347e+38`.
    /// - In case of f64, its `-1.7976931348623157e+308`.
    /// 
    /// Alternative to MIN, use -(MAX)
    const MIN: Self;

    /// Largest finite value representable by the
    /// underlying float type.
    /// 
    /// - In case of f32, its `3.40282347e+38`.
    /// - In case of f64, its `1.7976931348623157e+308`.
    /// 
    /// Alternative to MAX, use -(MIN)
    const MAX: Self;

    /// Archimede's constant, the value of `π = 3.141...`.
    /// 
    /// Underlying float type affects the precision of
    /// the value due to different mantissa bits.
    const PI: Self;

    /// Euler's constant, the value of `e = 2.7182...`.
    /// 
    /// Underlying float type affects the precision of
    /// the value due to different mantissa bits.
    const E: Self;

    /// [Machine epsilon] value for underlying float type.
    /// This is the difference between `1.0` and the next
    /// larger representable number.
    ///
    /// It is calculated using the formula: 
    /// 
    /// 2<sup>1&nbsp;&minus;&nbsp;`MANTISSA_DIGITS`</sup>.
    ///
    /// In case of `f32`, its:
    /// 
    /// 2<sup>1&nbsp;&minus;&nbsp;24</sup>
    /// 
    /// In case of `f64`, its:
    /// 
    /// 2<sup>1&nbsp;&minus;&nbsp;53</sup>
    /// 
    /// [Machine epsilon]: https://en.wikipedia.org/wiki/Machine_epsilon
    const EPSILON: Self;

    /// Positive infinity represented by rustc.
    /// 
    /// Its `∞ = 1.0 / 0.0`.
    const INFINITY: Self;

    /// Negative infinity represented by rustc.
    /// 
    /// Its `∞ = - 1.0 / 0.0`.
    const NEG_INFINITY: Self;

    /// Not a Number, represented by rustc for floats.
    /// 
    /// Rust does not guarantee the IEEE NAN standard.
    const NAN: Self;

    // Logical Ops

    /// Returns `true`, if the underlying float value is
    /// `NaN`.
    fn is_nan(self) -> bool;

    /// Returns `true` if the underlying float value is
    /// equal to either positive or negative infinity.
    /// 
    /// Otherwise it returns `False`
    fn is_infinite(self) -> bool;

    /// Returns `ture`, if the value is within +ve and
    /// -ve infinity range.
    /// 
    /// Otherwise returns `false`.
    fn is_finite(self) -> bool;

    /// Returns `true`, if the underlying value is +ve
    /// signed.
    fn is_sign_positive(self) -> bool;

    /// Returns `true`, if the underlying value is -ve
    /// signed.
    fn is_sign_negative(self) -> bool;

    /// Returns the absolute value of self.
    fn abs(self) -> Self;

    /// Returns the maximum of the two numbers in comparison.
    /// 
    /// Including `NaN`s.
    fn max(self, other: Self) -> Self;

    /// Returns the minimum of the two numbers in comparison.
    /// 
    /// Including `NaN`s.
    fn min(self, other: Self) -> Self;

    // Arithematic Ops

    /// Computes the square root of a number.
    /// 
    /// Its ressult is consistent with IEEE 754 specification 
    /// and is guaranteed to be rounded infinite-precision.
    /// 
    /// Returns `NAN`, if the num is negative.
    fn sqrt(self) -> Self;

    /// Eponential function, calculates and return the exponent
    /// raised to the power of self as `e`<sup>`self`</sup>.
    /// 
    /// Its result is not consistent, due to undeterministic
    /// precision in underlying float types.
    fn exp(self) -> Self;

    /// Computes the natural log of the underlying float type.
    /// 
    /// Its result is not consistent, due to undeterministic
    /// precision in underlying float types.
    fn ln(self) -> Self;

    /// Computes the sine of underlying float value in radians.
    /// 
    /// Its result is non-deterministic due to inconsistent
    /// precision.
    fn sin(self) -> Self;

    /// Computes the cosine of underlying float value in radians.
    /// 
    /// Its result is non-deterministic due to inconsistent
    /// precision.
    fn cos(self) -> Self;

    /// Computes the tangent of underlying float value in radians.
    /// 
    /// Its result is non-deterministic due to inconsistent
    /// precision.
    fn tan(self) -> Self;

    /// Computes the power `n` raised to an integer `m`,
    /// such as (`M`)<sup>`N`</sup>.
    /// 
    /// Its result is non-deterministic due to various
    /// reasons.
    fn powi(self, n: i32) -> Self;
    
    /// Computes the power in float type `n` raised to an
    /// integer `m`, such as (`M`)<sup>`N`</sup>.
    /// 
    /// Its result is non-deterministic due to various
    /// reasons.
    fn powf(self, n: Self) -> Self;

    /// Returns the smallest integer greater than or equal to `self`.
    ///
    /// This function always returns the precise result.
    fn ceil(self) -> Self;

    /// Returns the largest integer less than or equal to `self`.
    ///
    /// This function always returns the precise result.
    fn floor(self) -> Self;

    /// Returns the nearest integer to `self`. If a value is 
    /// half-way between two integers, round away from `0.0`.
    /// 
    /// Its result is deterministic and precise.
    fn round(self) -> Self;

    // Conversion to f32/f64

    fn to_f32(self) -> f32;
    fn to_f64(self) -> f64;
}


/// Macro for implementing core `Float` trait for f32 and f64 types.
macro_rules! impl_float {
    (
        $type:ty, 
        $min:expr, 
        $max:expr, 
        $pi:expr, 
        $e:expr, 
        $epsilon:expr, 
        $infinity:expr, 
        $neg_infinity:expr, 
        $nan:expr
    ) => {
        impl Float for $type {
            // Associative Consts

            const MIN: Self = $min;
            const MAX: Self = $max;
            const PI: Self = $pi;
            const E: Self = $e;
            const EPSILON: Self = $epsilon;
            const INFINITY: Self = $infinity;
            const NEG_INFINITY: Self = $neg_infinity;
            const NAN: Self = $nan;

            // Logical Ops

            #[must_use]
            #[inline(always)]
            fn is_nan(self) -> bool {
                self.is_nan()
            }

            #[must_use]
            #[inline(always)]
            fn is_infinite(self) -> bool {
                self.is_infinite()
            }

            #[must_use]
            #[inline(always)]
            fn is_finite(self) -> bool {
                self.is_finite()
            }

            #[must_use]
            #[inline(always)]
            fn is_sign_positive(self) -> bool {
                self.is_sign_positive()
            }

            #[must_use]
            #[inline(always)]
            fn is_sign_negative(self) -> bool {
                self.is_sign_negative()
            }

            #[must_use = "Returns a new value as a result and doesn't mutate self"]
            #[inline(always)]
            fn abs(self) -> Self {
                self.abs()
            }

            #[must_use = "Returns a new value as a result and doesn't mutate self"]
            #[inline(always)]
            fn max(self, other: Self) -> Self {
                self.max(other)
            }

            #[must_use = "Returns a new value as a result and doesn't mutate self"]
            #[inline(always)]
            fn min(self, other: Self) -> Self {
                self.min(other)
            }

            // Arithematic Ops

            #[must_use = "Returns a new value as a result and doesn't mutate self"]
            #[inline(always)]
            fn sqrt(self) -> Self {
                self.sqrt()
            }

            #[must_use = "Returns a new value as a result and doesn't mutate self"]
            #[inline(always)]
            fn exp(self) -> Self {
                self.exp()
            }

            #[must_use = "Returns a new value as a result and doesn't mutate self"]
            #[inline(always)]
            fn ln(self) -> Self {
                self.ln()
            }

            #[must_use = "Returns a new value as a result and doesn't mutate self"]
            #[inline(always)]
            fn sin(self) -> Self {
                self.sin()
            }

            #[must_use = "Returns a new value as a result and doesn't mutate self"]
            #[inline(always)]
            fn cos(self) -> Self {
                self.cos()
            }

            #[must_use = "Returns a new value as a result and doesn't mutate self"]
            #[inline(always)]
            fn tan(self) -> Self {
                self.tan()
            }

            #[must_use = "Returns a new value as a result and doesn't mutate self"]
            #[inline(always)]
            fn powi(self, n: i32) -> Self {
                self.powi(n)
            }

            #[must_use = "Returns a new value as a result and doesn't mutate self"]
            #[inline(always)]
            fn powf(self, n: Self) -> Self {
                self.powf(n)
            }

            #[must_use = "Returns a new value as a result and doesn't mutate self"]
            #[inline(always)]
            fn ceil(self) -> Self {
                self.ceil()
            }

            #[must_use = "Returns a new value as a result and doesn't mutate self"]
            #[inline(always)]
            fn floor(self) -> Self {
                self.floor()
            }

            #[must_use = "Returns a new value as a result and doesn't mutate self"]
            #[inline(always)]
            fn round(self) -> Self {
                self.round()
            }

            // Conversions

            #[must_use = "Returns a new value as a result and doesn't mutate self"]
            #[inline(always)]
            fn to_f32(self) -> f32 {
                self as f32
            }

            #[must_use = "Returns a new value as a result and doesn't mutate self"]
            #[inline(always)]
            fn to_f64(self) -> f64 {
                self as f64
            }
        }
    };
}

impl_float!(
    f32,
    f32::MIN,
    f32::MAX,
    std::f32::consts::PI,
    std::f32::consts::E,
    f32::EPSILON,
    f32::INFINITY,
    f32::NEG_INFINITY,
    f32::NAN
);

impl_float!(
    f64,
    f64::MIN,
    f64::MAX,
    std::f64::consts::PI,
    std::f64::consts::E,
    f64::EPSILON,
    f64::INFINITY,
    f64::NEG_INFINITY,
    f64::NAN
);
