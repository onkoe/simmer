//! # Simmer
//!
//! A collection of tools to easily represent different temperatures units and
//! convert between them. Mostly for my embedded thermocouple projects. 😄
//!
//! These should work in embedded (`no_std`) environments, too!
//!
//! ## Usage
//!
//! There's nothing complex here. You can wrap your floating point types in a
//! [Temperature] to make sure it keeps its unit. It'll also convert
//! automatically between units, use `Display` and `Debug`, and help you
//! explicitly unwrap the values when you don't need the unit anymore!
//!
//! It's also worth mentioning that *embedded users should enable the `f32`
//! feature flag*. This will help you avoid `f64` problems, particularly on
//! AVR devices where 64-bit floating point values aren't supported.
//!
//! The [ufmt crate's](https://docs.rs/ufmt/latest/ufmt/) `uDisplay` and `uDebug`
//! traits are implemented, so you can use [Temperature] values much like you'd
//! use [ufmt_float](https://docs.rs/ufmt_float/latest/ufmt_float/).
//! Feel free to unwrap the values and manually print it, though! 🥹
//!
//! Anyways, here's an example...
//!
//!```ignore
//! use simmer::Temperature;
//!
//! let ice = Temperature::Fahrenheit(32.0);
//! println!("water freezes at {ice} degrees fahrenheit");
//!
//! let ice_c = ice.to_celsius();
//! println!("water freezes at {ice_c} degrees celsius");
//!
//! // i want that cool number 🥁
//! let ice_raw_c: f64 = ice_c.into(); // it's an f32 if the feature is enabled!
//! println!("here's a number: {ice_raw_c}");
//! ```
//!
//! ### Checked
//!
//! There's also a [CheckedTemperature] type so you can safely store and use
//! temperatures. It works on embedded and implements many of the same
//! functions that [Temperature] does!
//!
//! See the [checked] module for more!
//!
//! Here's an example showing how to use it:
//!
//! ```ignore
//! use simmer::{CheckedTemperature, Temperature};
//!
//! fn main() -> anyhow::Result<()> {
//!     let ice = CheckedTemperature::new(Temperature::Fahrenheit(32.0))?;
//!     println!("water freezes at {ice} degrees fahrenheit");
//!
//!     let ice_c = ice.to_celsius()?;
//!     let ice_raw_c: f64 = ice_c.into(); // can also use `f32` 😄
//!     println!("here's a number: {ice_raw_c}");
//!
//!     Ok(())
//! }
//!
//! ```

#[cfg(any(feature = "checked", doc))]
pub mod checked;

#[cfg(any(feature = "checked", doc))]
pub use self::checked::CheckedTemperature;

#[cfg(not(feature = "f32"))]
type Float = f64;

#[cfg(feature = "f32")]
type Float = f32;

/// A value that's one of many common temperature units.
///
/// Wraps a floating point number to give it a unit!
/// You can create a new `Temperature` by putting a float value inside.
///
/// **Important**: `Temperature` is *not* checked, so invalid states are
/// completely allowed.
///
#[cfg_attr(feature = "f32", doc = "```ignore")]
#[cfg_attr(not(feature = "f32"), doc = "```")]
/// use simmer::Temperature;
///
/// let my_temp = Temperature::Celsius(0.0);
///```
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum Temperature {
    Fahrenheit(self::Float),
    Celsius(self::Float),
    Kelvin(self::Float),
}

impl Temperature {
    /// Return a Temperature in Fahrenheit based off of Self.
    ///
    /// # Usage
    #[cfg_attr(feature = "f32", doc = "```ignore")]
    #[cfg_attr(not(feature = "f32"), doc = "```")]
    /// # use simmer::Temperature;
    /// # use assert_approx_eq::assert_approx_eq;
    /// #
    /// let body_temp_c = Temperature::Celsius(37.0);
    ///
    /// let body_temp_f = body_temp_c.to_fahrenheit();
    /// assert_approx_eq!(body_temp_f.into_inner(), 98.6);
    /// ```
    pub fn to_fahrenheit(&self) -> Temperature {
        match self {
            Self::Fahrenheit(_) => *self,
            Self::Celsius(c) => Self::Fahrenheit((c * 1.8) + 32.0),
            Self::Kelvin(k) => Self::Fahrenheit(((k - 273.15) * 1.8) + 32.0),
        }
    }

    /// Return a Temperature in Celsius based off of Self.
    ///
    /// # Usage
    ///
    #[cfg_attr(feature = "f32", doc = "```ignore")]
    #[cfg_attr(not(feature = "f32"), doc = "```")]
    /// # use simmer::Temperature;
    /// # use assert_approx_eq::assert_approx_eq;
    /// #
    /// let body_temp_f = Temperature::Fahrenheit(98.6);
    ///
    /// let body_temp_c = body_temp_f.to_celsius();
    /// assert_approx_eq!(body_temp_c.into_inner(), 37.0);
    /// ```
    pub fn to_celsius(&self) -> Temperature {
        match self {
            Temperature::Fahrenheit(f) => Self::Celsius((f - 32.0) / 1.8),
            Temperature::Celsius(_) => *self,
            Temperature::Kelvin(k) => Self::Celsius(k - 273.15),
        }
    }

    /// Return a Temperature in Kelvin based off of Self.
    ///
    /// # Usage
    ///
    #[cfg_attr(feature = "f32", doc = "```ignore")]
    #[cfg_attr(not(feature = "f32"), doc = "```")]
    /// # use simmer::Temperature;
    /// # use assert_approx_eq::assert_approx_eq;
    /// #
    /// let abs_zero_k = Temperature::Kelvin(0.0);
    ///
    /// let abs_zero_c = abs_zero_k.to_celsius();
    /// assert_approx_eq!(abs_zero_c.into_inner(), -273.15);
    /// ```
    pub fn to_kelvin(&self) -> Temperature {
        match self {
            Temperature::Fahrenheit(f) => Self::Kelvin(((f - 32.0) / 1.8) + 273.15),
            Temperature::Celsius(c) => Self::Kelvin(c + 273.15),
            Temperature::Kelvin(_) => *self,
        }
    }

    /// A discovery function that returns the inner type, consuming the outer Temperature type.
    /// Use `my_temp.into()` when possible.
    ///
    /// # Usage
    ///
    #[cfg_attr(feature = "f32", doc = "```ignore")]
    #[cfg_attr(not(feature = "f32"), doc = "```")]
    /// # use simmer::Temperature;
    /// #
    /// let my_temp = Temperature::Fahrenheit(98.6);
    /// let my_temp_float = my_temp.into_inner();
    /// ```
    pub fn into_inner(self) -> Float {
        Into::<Float>::into(self)
    }

    /// Gets the inner floating point value.
    ///
    /// # Usage
    ///
    #[cfg_attr(feature = "f32", doc = "```ignore")]
    #[cfg_attr(not(feature = "f32"), doc = "```")]
    /// # use simmer::Temperature;
    /// #
    /// let temp = Temperature::Kelvin(0.0);
    /// let temp_inner = temp.get_inner();
    ///
    /// println!("{temp:?}'s inner is {temp_inner}");
    /// ```
    pub const fn get_inner(&self) -> Float {
        match self {
            Temperature::Fahrenheit(t) => *t,
            Temperature::Celsius(t) => *t,
            Temperature::Kelvin(t) => *t,
        }
    }

    /// Tells you if a [Temperature] is below absolute zero - an invalid state
    /// for temperature.
    ///
    /// So... returns:
    /// - `true` if `t` >= abs zero
    /// - `false` if `t` < abs zero
    ///
    /// # Usage
    ///
    #[cfg_attr(feature = "f32", doc = "```ignore")]
    #[cfg_attr(not(feature = "f32"), doc = "```")]
    /// # use simmer::Temperature;
    /// #
    /// let temp = Temperature::Kelvin(0.0);
    /// assert!(!temp.is_below_abs_zero());
    ///
    /// let temp2 = Temperature::Kelvin(-0.1);
    /// assert!(temp2.is_below_abs_zero());
    /// ```
    pub fn is_below_abs_zero(&self) -> bool {
        match self {
            Temperature::Fahrenheit(f) => *f < -459.67,
            Temperature::Celsius(c) => *c < -273.15,
            Temperature::Kelvin(k) => *k < 0.0,
        }
    }

    /// Checks if the internal floating point number is `NaN`.
    ///
    /// # Usage
    ///
    #[cfg_attr(feature = "f32", doc = "```ignore")]
    #[cfg_attr(not(feature = "f32"), doc = "```")]
    /// # use simmer::Temperature;
    /// #
    /// let temp = Temperature::Fahrenheit(f64::NAN);
    /// assert!(temp.is_nan());
    /// ```
    pub fn is_nan(&self) -> bool {
        match self {
            Temperature::Celsius(t) | Temperature::Fahrenheit(t) | Temperature::Kelvin(t) => {
                t.is_nan()
            }
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<Float> for Temperature {
    fn into(self) -> Float {
        match self {
            Temperature::Fahrenheit(f) => f,
            Temperature::Celsius(c) => c,
            Temperature::Kelvin(k) => k,
        }
    }
}

// various display impls

impl core::fmt::Display for Temperature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.get_inner())
    }
}

impl ufmt::uDebug for Temperature {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt_write::uWrite + ?Sized,
    {
        let unit = match self {
            Temperature::Fahrenheit(_) => "Fahrenheit",
            Temperature::Celsius(_) => "Celsius",
            Temperature::Kelvin(_) => "Kelvin",
        };

        #[cfg(feature = "f32")]
        return ufmt::uwrite!(
            f,
            "Temperature::{}({})",
            unit,
            ufmt_float::uFmt_f32::Five(self.get_inner())
        );

        #[cfg(not(feature = "f32"))]
        return ufmt::uwrite!(
            f,
            "Temperature::{}({})",
            unit,
            ufmt_float::uFmt_f64::Five(self.get_inner())
        );
    }
}

impl ufmt::uDisplay for Temperature {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt_write::uWrite + ?Sized,
    {
        #[cfg(feature = "f32")]
        return ufmt::uwrite!(f, "{}", ufmt_float::uFmt_f32::Five(self.get_inner()));

        #[cfg(not(feature = "f32"))]
        return ufmt::uwrite!(f, "{}", ufmt_float::uFmt_f64::Five(self.get_inner()));
    }
}

// operator overloading impls

impl core::ops::Add for Temperature {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Temperature::Fahrenheit(f) => {
                Temperature::Fahrenheit(f + rhs.to_fahrenheit().into_inner())
            }
            Temperature::Celsius(c) => Temperature::Celsius(c + rhs.to_celsius().into_inner()),
            Temperature::Kelvin(k) => Temperature::Kelvin(k + rhs.to_kelvin().into_inner()),
        }
    }
}

impl core::ops::Sub for Temperature {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Temperature::Fahrenheit(f) => {
                Temperature::Fahrenheit(f - rhs.to_fahrenheit().into_inner())
            }
            Temperature::Celsius(c) => Temperature::Celsius(c - rhs.to_celsius().into_inner()),
            Temperature::Kelvin(k) => Temperature::Kelvin(k - rhs.to_kelvin().into_inner()),
        }
    }
}

// note: you can add and subtract temperatures, but i can't think of any
// possible reason to multiply/divide them.

// as such, i used `Float` on these two - it just makes more sense..!

// please let me know if you have a use-case for multiplying or dividing
// two temperatures together. i want to document it!

impl core::ops::Div<Float> for Temperature {
    type Output = Self;

    fn div(self, rhs: Float) -> Self::Output {
        match self {
            Temperature::Fahrenheit(f) => Temperature::Fahrenheit(f / rhs),
            Temperature::Celsius(c) => Temperature::Celsius(c / rhs),
            Temperature::Kelvin(k) => Temperature::Kelvin(k / rhs),
        }
    }
}

impl core::ops::Mul<Float> for Temperature {
    type Output = Self;

    fn mul(self, rhs: Float) -> Self::Output {
        match self {
            Temperature::Fahrenheit(f) => Temperature::Fahrenheit(f * rhs),
            Temperature::Celsius(c) => Temperature::Celsius(c * rhs),
            Temperature::Kelvin(k) => Temperature::Kelvin(k * rhs),
        }
    }
}
