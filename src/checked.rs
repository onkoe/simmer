#![cfg(all(any(feature = "checked", doc), std))]
//! # Checked
//!
//! [Temperature] is useful for storing a real-world temperature value, but it
//! may hurt when you need to check any operations applied to a temperature.
//!
//! [CheckedTemperature] enforces an
//! [absolute zero](https://en.wikipedia.org/wiki/Absolute_zero) boundary for
//! contained temperatures.
//!
//! You can also set your own upper and lower limits for the contained
//! temperature, helping to ensure that any value is within your project's
//! bounds.
//!
//! **Warning**: Due to internal types using floating point numbers, mildly
//! invalid state may be representable. For example, a temperature that's below
//! 0.0° K by "less than" a bit will be represented as 0.0° K.
//!
//! [Fractional values](https://docs.rs/fraction/) would fix these issues,
//! but they aren't (yet) supported.
//!
//! ## Usage
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

use onlyerror::{self, Error};

use crate::{Float, Temperature};

/// A set of bounds for which a [CheckedTemperature] cannot exceed.
/// By default, these are \[Float::NEG_INFINITY, Float::INFINITY\], but users can change them
/// for their uses.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
struct Bounds {
    lower: Float,
    upper: Float,
}

impl Default for Bounds {
    /// The default [Bounds] for some floating point number.
    /// \[Float::NEG_INFINITY, Float::INFINITY\]
    fn default() -> Self {
        Self {
            #[cfg(feature = "f32")]
            lower: f32::NEG_INFINITY,
            #[cfg(feature = "f32")]
            upper: f32::INFINITY,
            #[cfg(not(feature = "f32"))]
            lower: f64::NEG_INFINITY,
            #[cfg(not(feature = "f32"))]
            upper: f64::INFINITY,
        }
    }
}

impl Bounds {
    /// Helper function to get a `Float`'s `MAX`.
    const fn get_float_max() -> Float {
        #[cfg(feature = "f32")]
        return f32::MAX;

        #[cfg(not(feature = "f32"))]
        return f64::MAX;
    }

    /// Helper function to get a `Float`'s `MIN`.
    const fn get_float_min() -> Float {
        #[cfg(feature = "f32")]
        return f32::MIN;

        #[cfg(not(feature = "f32"))]
        return f64::MIN;
    }

    /// Tries to set the lower bound to a given value.
    /// Can fail if larger than the Float's `MAX` or the upper bound.
    pub fn set_lower(&mut self, val: Float) -> Result<(), CheckedTempError> {
        if val > self.upper {
            return Err(CheckedTempError::BoundTooHigh(val));
        } else if val < Bounds::get_float_min() {
            return Err(CheckedTempError::BoundTooLow(val));
        }

        self.lower = val;

        Ok(())
    }

    /// Tries to set the upper bound to some given value.
    /// Fails when the value is under `Float::MIN` or the lower bound.
    pub fn set_upper(&mut self, val: Float) -> Result<(), CheckedTempError> {
        if val < self.lower {
            return Err(CheckedTempError::BoundTooLow(val));
        } else if val > Bounds::get_float_max() {
            return Err(CheckedTempError::BoundTooHigh(val));
        }

        self.upper = val;

        Ok(())
    }
}

/// An error regarding [CheckedTemperature].
#[derive(Debug, Error)]
pub enum CheckedTempError {
    #[error("Given bound, {0}, was too low.")]
    BoundTooLow(Float),
    #[error("Given bound, {0}, was too high.")]
    BoundTooHigh(Float),
    #[error("The given temperature, {0}, was below absolute zero.")]
    BelowAbsoluteZero(Float),
    #[error("The given temperature, {0}, was out of bounds. ({1})")]
    TempOutOfBounds(Float, &'static str),
    #[error("Division by zero is not allowed.")]
    DivisionByZero,
    #[error("NaN values are not allowed for CheckedTemperature construction.")]
    GivenValueIsNan,
}

/// A [Temperature] that cannot be invalid.
///
/// It also stores bounds which require a temperature to be within some range.
///
/// # Usage
///
#[cfg_attr(not(feature = "checked"), doc = "```ignore")]
#[cfg_attr(feature = "checked", doc = "```")]
/// use simmer::{CheckedTemperature, Temperature};
///
/// # fn main() -> anyhow::Result<()> {
/// let checked_temp = CheckedTemperature::new(Temperature::Kelvin(0.2))?;
/// println!("oh baby it's barely not absolute zero: {checked_temp}");
/// #
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct CheckedTemperature {
    temp: Temperature,
    bounds: Bounds,
}

impl CheckedTemperature {
    /// Checks a temperature for problems, such as being below abs. zero or
    /// being out of bounds!
    fn check(&self, temp: Temperature) -> Result<(), CheckedTempError> {
        if temp.is_below_abs_zero() {
            return Err(CheckedTempError::BelowAbsoluteZero(temp.get_inner()));
        }

        if temp.is_nan() {
            return Err(CheckedTempError::GivenValueIsNan);
        }

        // over user-set upper bound
        if temp.get_inner() > self.bounds.upper {
            return Err(CheckedTempError::TempOutOfBounds(
                temp.get_inner(),
                "Too high!",
            ));
        }

        // under user-set lower bound
        if temp.get_inner() < self.bounds.lower {
            return Err(CheckedTempError::TempOutOfBounds(
                temp.get_inner(),
                "Too low!",
            ));
        }

        Ok(())
    }

    /// Tries to create a new [CheckedTemperature] from a given [Temperature].
    /// Fails if temperature is invalid (below absolute zero).
    ///
    /// # Usage
    ///
    #[cfg_attr(not(feature = "checked"), doc = "```ignore")]
    #[cfg_attr(feature = "checked", doc = "```")]
    /// use simmer::{CheckedTemperature, Temperature};
    /// #
    /// # fn main() -> anyhow::Result<()> {
    ///     let my_temp = CheckedTemperature::new(Temperature::Fahrenheit(32.0))?;
    ///     println!("water freezes at {my_temp} degrees f!");
    /// #   Ok(())
    /// # }
    /// ```
    pub fn new(temp: Temperature) -> Result<CheckedTemperature, CheckedTempError> {
        if temp.is_below_abs_zero() {
            return Err(CheckedTempError::BelowAbsoluteZero(temp.get_inner()));
        }

        if temp.is_nan() {
            return Err(CheckedTempError::GivenValueIsNan);
        }

        // over upper bound
        if temp.get_inner() > Bounds::get_float_max() {
            return Err(CheckedTempError::TempOutOfBounds(
                temp.get_inner(),
                "Too high!",
            ));
        }

        // under lower bound
        if temp.get_inner() < Bounds::get_float_min() {
            return Err(CheckedTempError::TempOutOfBounds(
                temp.get_inner(),
                "Too low!",
            ));
        }

        Ok(CheckedTemperature {
            temp,
            bounds: Bounds::default(),
        })
    }

    /// Tries to change the current value of `Self` to a new [Temperature].
    ///
    /// # Usage
    ///
    #[cfg_attr(not(feature = "checked"), doc = "```ignore")]
    #[cfg_attr(feature = "checked", doc = "```")]
    /// # use simmer::{checked::CheckedTemperature, Temperature};
    /// # use assert_approx_eq::assert_approx_eq;
    /// #
    /// # fn main() -> anyhow::Result<()> {
    ///     let mut my_temp = CheckedTemperature::new(Temperature::Celsius(24.0))?;
    ///     my_temp.set_temperature(Temperature::Fahrenheit(72.0));
    ///     
    ///     assert_approx_eq!(my_temp.get_inner(), 72.0);
    /// #   Ok(())
    /// # }
    /// ```
    pub fn set_temperature(&mut self, new: Temperature) -> Result<(), CheckedTempError> {
        self.check(new)?;

        self.temp = new;
        Ok(())
    }

    /// Returns the internal unchecked [Temperature].
    ///
    /// # Usage
    ///
    #[cfg_attr(not(feature = "checked"), doc = "```ignore")]
    #[cfg_attr(feature = "checked", doc = "```")]
    /// # use simmer::{checked::CheckedTemperature, Temperature};
    /// #
    /// # fn main() -> anyhow::Result<()> {
    ///     let checked = CheckedTemperature::new(Temperature::Fahrenheit(32.0))?;
    ///     let unchecked = checked.get_unchecked();
    ///
    ///     assert_eq!(unchecked.get_inner(), checked.get_inner());
    ///     # Ok(())
    /// # }
    /// ```
    pub fn get_unchecked(&self) -> Temperature {
        self.temp
    }

    /// Transforms a `CheckedTemperature` into a `Temperature`.
    ///
    /// # Usage
    ///
    #[cfg_attr(not(feature = "checked"), doc = "```ignore")]
    #[cfg_attr(feature = "checked", doc = "```")]
    /// # use simmer::{checked::CheckedTemperature, Temperature};
    /// #
    /// # fn main() -> anyhow::Result<()> {
    ///     let checked = CheckedTemperature::new(Temperature::Fahrenheit(32.0))?;
    ///     let unchecked = checked.into_unchecked();
    ///     
    ///     // checked doesn't exist anymore
    ///     println!("my unchecked temp is: {unchecked}!");
    ///     # Ok(())
    /// # }
    /// ```
    pub fn into_unchecked(self) -> Temperature {
        self.temp
    }

    // some delegate methods from `Temperature`

    /// Gets the inner floating point value.
    ///
    /// # Usage
    ///
    #[cfg_attr(not(feature = "checked"), doc = "```ignore")]
    #[cfg_attr(feature = "checked", doc = "```")]
    /// # use simmer::{checked::CheckedTemperature, Temperature};
    /// #
    /// # fn main() -> anyhow::Result<()> {
    ///     let temp = CheckedTemperature::new(Temperature::Kelvin(0.0))?;
    ///     let temp_inner = temp.get_inner();
    ///
    ///     println!("{temp:?}'s inner is {temp_inner}");
    /// #   Ok(())
    /// # }
    /// ```
    pub fn get_inner(&self) -> Float {
        self.temp.get_inner()
    }

    /// A discovery function that returns the inner type, consuming the outer Temperature type.
    /// Use `my_temp.into()` when possible.
    ///
    /// # Usage
    ///
    #[cfg_attr(not(feature = "checked"), doc = "```ignore")]
    #[cfg_attr(feature = "checked", doc = "```")]
    /// # use simmer::{checked::CheckedTemperature, Temperature};
    /// #
    /// # fn main() -> anyhow::Result<()> {
    ///     let my_temp = CheckedTemperature::new(Temperature::Fahrenheit(98.6))?;
    ///     let my_temp_float = my_temp.into_inner(); // moved my_temp. it doesn't exist now!
    ///
    ///     println!("{my_temp} doesn't exist so this won't compile!!!");
    ///     # Ok(())
    /// # }
    /// ```
    pub fn into_inner(self) -> Float {
        self.temp.into_inner()
    }

    /// helper function to adjust the bounds.
    fn adjust_bounds(
        &mut self,
        new_unit: fn(Float) -> Temperature,
    ) -> Result<(), CheckedTempError> {
        let current_unit = match self.temp {
            Temperature::Fahrenheit(_) => Temperature::Fahrenheit,
            Temperature::Celsius(_) => Temperature::Celsius,
            Temperature::Kelvin(_) => Temperature::Kelvin,
        };

        // don't bother converting if we're converting to the same type
        if new_unit == current_unit {
            return Ok(());
        }

        // don't try to convert infinities
        if self.bounds.lower == Float::NEG_INFINITY && self.bounds.upper == Float::INFINITY {
            return Ok(());
        }

        let set_with_bounds = |b: Float| -> Result<Float, CheckedTempError> {
            let current_bound = current_unit(b);

            Ok(match new_unit(0.0) {
                Temperature::Fahrenheit(_) => current_bound.to_fahrenheit().into_inner(),
                Temperature::Celsius(_) => current_bound.to_celsius().into_inner(),
                Temperature::Kelvin(_) => current_bound.to_kelvin().into_inner(),
            })
        };

        if self.bounds.lower != Float::NEG_INFINITY {
            self.bounds.lower = set_with_bounds(self.bounds.lower)?;
        }

        if self.bounds.upper != Float::INFINITY {
            self.bounds.upper = set_with_bounds(self.bounds.upper)?;
        }

        Ok(())
    }

    /// Converts the internal [Temperature] to Fahrenheit and rewraps it.
    ///
    /// Warning: Adjusts bounds by converting them!
    ///
    /// # Usage
    ///
    #[cfg_attr(not(feature = "checked"), doc = "```ignore")]
    #[cfg_attr(feature = "checked", doc = "```")]
    /// # use simmer::{checked::CheckedTemperature, Temperature};
    /// # use assert_approx_eq::assert_approx_eq;
    /// #
    /// # fn main() -> anyhow::Result<()> {
    /// let mut body_temp_c = CheckedTemperature::new(Temperature::Celsius(37.0))?;
    ///
    /// let body_temp_f = body_temp_c.to_fahrenheit()?;
    /// assert_approx_eq!(body_temp_f.into_inner(), 98.6);
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_fahrenheit(&self) -> Result<CheckedTemperature, CheckedTempError> {
        let mut new = *self;

        // adjust bounds
        new.adjust_bounds(Temperature::Fahrenheit)?;

        new.temp = new.temp.to_fahrenheit();
        Ok(new)
    }

    /// Converts the internal [Temperature] to Celsius and rewraps it.
    ///
    /// Warning: Adjusts bounds by converting them!
    ///
    /// # Usage
    ///
    #[cfg_attr(not(feature = "checked"), doc = "```ignore")]
    #[cfg_attr(feature = "checked", doc = "```")]
    /// # use simmer::{checked::CheckedTemperature, Temperature};
    /// # use assert_approx_eq::assert_approx_eq;
    /// #
    /// # fn main() -> anyhow::Result<()> {
    /// let mut body_temp_f = CheckedTemperature::new(Temperature::Fahrenheit(98.6))?;
    ///
    /// let body_temp_c = body_temp_f.to_celsius()?;
    /// assert_approx_eq!(body_temp_c.into_inner(), 37.0);
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_celsius(&mut self) -> Result<CheckedTemperature, CheckedTempError> {
        // adjust bounds
        self.adjust_bounds(Temperature::Celsius)?;

        self.temp = self.temp.to_celsius();
        Ok(self.to_owned())
    }

    /// Converts the internal [Temperature] to Kelvin and rewraps it.
    ///
    /// Warning: Adjusts bounds by converting them!
    ///
    /// # Usage
    ///
    #[cfg_attr(not(feature = "checked"), doc = "```ignore")]
    #[cfg_attr(feature = "checked", doc = "```")]
    /// # use simmer::{checked::CheckedTemperature, Temperature};
    /// # use assert_approx_eq::assert_approx_eq;
    /// #
    /// # fn main() -> anyhow::Result<()> {
    /// let mut abs_zero_k = CheckedTemperature::new(Temperature::Kelvin(0.0))?;
    ///
    /// let abs_zero_c = abs_zero_k.to_celsius()?;
    /// assert_approx_eq!(abs_zero_c.into_inner(), -273.15);
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_kelvin(&mut self) -> Result<CheckedTemperature, CheckedTempError> {
        // adjust bounds
        self.adjust_bounds(Temperature::Kelvin)?;

        self.temp = self.temp.to_kelvin();
        Ok(self.to_owned())
    }

    // a little math...
    // can't operator overload with `Result`, so these will have to do

    /// Tries to add two temperatures together.
    ///
    /// # Usage
    ///
    #[cfg_attr(not(feature = "checked"), doc = "```ignore")]
    #[cfg_attr(feature = "checked", doc = "```")]
    /// # use simmer::{checked::CheckedTemperature, Temperature};
    /// # use assert_approx_eq::assert_approx_eq;
    /// #
    /// # fn main() -> anyhow::Result<()> {
    ///     let mut my_temp = CheckedTemperature::new(Temperature::Celsius(32.0))?;
    ///     my_temp.add(Temperature::Celsius(32.0))?;
    ///
    ///     assert_approx_eq!(my_temp.get_inner(), 64.0);
    /// #
    /// #   Ok(())
    /// # }
    /// ```
    pub fn add(&mut self, temp: Temperature) -> Result<(), CheckedTempError> {
        let result = self.temp + temp;
        self.check(result)?;

        self.temp = result;
        Ok(())
    }

    /// Tries to subtract using two temperatures.
    ///
    /// # Usage
    ///
    #[cfg_attr(not(feature = "checked"), doc = "```ignore")]
    #[cfg_attr(feature = "checked", doc = "```")]
    /// # use simmer::{checked::CheckedTemperature, Temperature};
    /// # use assert_approx_eq::assert_approx_eq;
    /// #
    /// # fn main() -> anyhow::Result<()> {
    ///     let mut my_temp = CheckedTemperature::new(Temperature::Celsius(64.0))?;
    ///     my_temp.sub(Temperature::Celsius(32.0))?;
    ///
    ///     assert_approx_eq!(my_temp.get_inner(), 32.0);
    /// #
    /// #   Ok(())
    /// # }
    /// ```
    pub fn sub(&mut self, temp: Temperature) -> Result<(), CheckedTempError> {
        let result = self.temp - temp;
        self.check(result)?;

        self.temp = result;
        Ok(())
    }

    /// Tries to multiply a temperature by another number.
    ///
    /// # Usage
    ///
    #[cfg_attr(not(feature = "checked"), doc = "```ignore")]
    #[cfg_attr(feature = "checked", doc = "```")]
    /// # use simmer::{checked::CheckedTemperature, Temperature};
    /// # use assert_approx_eq::assert_approx_eq;
    /// #
    /// # fn main() -> anyhow::Result<()> {
    ///     let mut my_temp = CheckedTemperature::new(Temperature::Celsius(32.0))?;
    ///     my_temp.mul(2.0)?;
    ///
    ///     assert_approx_eq!(my_temp.get_inner(), 64.0);
    /// #
    /// #   Ok(())
    /// # }
    /// ```
    pub fn mul(&mut self, num: Float) -> Result<(), CheckedTempError> {
        let result = self.temp * num;
        self.check(result)?;

        self.temp = result;
        Ok(())
    }

    /// Tries to divide a temperature by another number.
    ///
    /// # Usage
    ///
    #[cfg_attr(not(feature = "checked"), doc = "```ignore")]
    #[cfg_attr(feature = "checked", doc = "```")]
    /// # use simmer::{checked::CheckedTemperature, Temperature};
    /// # use assert_approx_eq::assert_approx_eq;
    /// #
    /// # fn main() -> anyhow::Result<()> {
    ///     let mut my_temp = CheckedTemperature::new(Temperature::Celsius(32.0))?;
    ///     my_temp.div(2.0)?;
    ///
    ///     assert_approx_eq!(my_temp.get_inner(), 16.0);
    /// #
    /// #   Ok(())
    /// # }
    /// ```
    ///
    /// ## Note: Fails on Zero
    ///
    /// Division by zero isn't allowed...
    ///
    #[cfg_attr(not(feature = "checked"), doc = "```ignore")]
    #[cfg_attr(feature = "checked", doc = "```should_panic")]
    /// # use simmer::{checked::CheckedTemperature, Temperature};
    /// # use assert_approx_eq::assert_approx_eq;
    /// #
    /// # fn main() -> anyhow::Result<()> {
    ///     let mut my_temp = CheckedTemperature::new(Temperature::Celsius(32.0))?;
    ///     my_temp.div(0.0)?;
    /// #
    /// #   Ok(())
    /// # }
    /// ```
    pub fn div(&mut self, num: Float) -> Result<(), CheckedTempError> {
        if num == 0.0 {
            return Err(CheckedTempError::DivisionByZero);
        }

        let result = self.temp / num;
        self.check(result)?;

        self.temp = result;
        Ok(())
    }

    /// Tries to set the upper allowed bound to a given value.
    ///
    /// # Usage
    ///
    #[cfg_attr(not(feature = "checked"), doc = "```ignore")]
    #[cfg_attr(feature = "checked", doc = "```should_panic")]
    /// # use simmer::{checked::CheckedTemperature, Temperature};
    /// #
    /// # fn main() -> anyhow::Result<()> {
    ///     let mut my_temp = CheckedTemperature::new(Temperature::Celsius(42.3))?;
    ///     my_temp.set_upper_bound(0.0)?; // no going above water's freezing temp
    ///
    ///     my_temp.set_temperature(Temperature::Celsius(24.0))?; // that's an error :o
    /// #
    /// #   Ok(())
    /// # }
    ///
    /// ```
    pub fn set_upper_bound(&mut self, bound: Float) -> Result<(), CheckedTempError> {
        self.bounds.set_upper(bound)?;
        Ok(())
    }

    /// Tries to set the lower allowed bound to a given value.
    ///
    /// # Usage
    ///
    #[cfg_attr(not(feature = "checked"), doc = "```ignore")]
    #[cfg_attr(feature = "checked", doc = "```should_panic")]
    /// # use simmer::{checked::CheckedTemperature, Temperature};
    /// #
    /// # fn main() -> anyhow::Result<()> {
    ///     let mut my_temp = CheckedTemperature::new(Temperature::Celsius(42.3))?;
    ///     my_temp.set_lower_bound(0.0)?; // no going below water's freezing temp
    ///
    ///     my_temp.set_temperature(Temperature::Celsius(-24.0))?; // that's an error :o
    /// #
    /// #   Ok(())
    /// # }
    ///
    /// ```
    pub fn set_lower_bound(&mut self, bound: Float) -> Result<(), CheckedTempError> {
        self.bounds.set_lower(bound)?;
        Ok(())
    }

    /// Tries to set both bounds to the given values.
    ///
    /// # Usage
    ///
    #[cfg_attr(not(feature = "checked"), doc = "```ignore")]
    #[cfg_attr(feature = "checked", doc = "```should_panic")]
    /// # use simmer::{checked::CheckedTemperature, Temperature};
    /// #
    /// # fn main() -> anyhow::Result<()> {
    ///     let mut thermostat = CheckedTemperature::new(Temperature::Fahrenheit(68.5))?;
    ///     thermostat.set_bounds(68.0, 72.0)?; // let's keep a warm house
    ///
    ///     thermostat.set_temperature(Temperature::Fahrenheit(65.0))?; // brrr! that's an error buddy
    /// #
    /// #   Ok(())
    /// # }
    ///
    /// ```
    pub fn set_bounds(
        &mut self,
        lower_bound: Float,
        upper_bound: Float,
    ) -> Result<(), CheckedTempError> {
        self.bounds.set_lower(lower_bound)?;
        self.bounds.set_upper(upper_bound)?;

        Ok(())
    }

    /// Returns the bounds of this `CheckedTemperature` as (unchecked)
    /// [Temperature]s.
    ///
    /// Bounds are a tuple, `(lower, upper)`. For example, you may get back a
    /// tuple which is `(Temp::F(32.0), Temp::F(72.0))`.
    ///
    /// # Usage
    ///
    /// When you have a temperature that you've set bounds on, use this
    /// method to check on them.
    ///
    #[cfg_attr(not(feature = "checked"), doc = "```ignore")]
    #[cfg_attr(feature = "checked", doc = "```")]
    /// # use simmer::{CheckedTemperature, Temperature};
    /// # use assert_approx_eq::assert_approx_eq;
    /// #
    /// # fn main() -> anyhow::Result<()> {
    /// let mut temp = CheckedTemperature::new(Temperature::Fahrenheit(68.5))?;
    /// temp.set_bounds(32.0, 72.0)?;
    ///
    /// let bounds = temp.get_bounds();
    /// assert_approx_eq!(bounds.0.into_inner(), 32.0);
    /// assert_approx_eq!(bounds.1.into_inner(), 72.0);
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_bounds(&self) -> (Temperature, Temperature) {
        let t: fn(Float) -> Temperature = match self.temp {
            Temperature::Fahrenheit(_) => Temperature::Fahrenheit,
            Temperature::Celsius(_) => Temperature::Celsius,
            Temperature::Kelvin(_) => Temperature::Kelvin,
        };

        (t(self.bounds.lower), t(self.bounds.upper))
    }
}

// some display impls... ripped straight from `Temperature` 😖
// various display impls

impl core::fmt::Display for CheckedTemperature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.get_inner())
    }
}

impl ufmt::uDebug for CheckedTemperature {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt_write::uWrite + ?Sized,
    {
        let unit = match self.temp {
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

impl ufmt::uDisplay for CheckedTemperature {
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
