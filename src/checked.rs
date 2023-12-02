#![cfg(any(feature = "checked", doc))]
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

use onlyerror::{self, Error};

use crate::{Float, Temperature};

// TODO: usage examples
// TODO: tests !!!!!!!!!!!!!

/// A set of bounds for which a [CheckedTemperature] cannot exceed.
/// By default, these are \[Float::MIN, Float::MAX\], but you can change them
/// for your uses.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Bounds {
    lower: Float,
    upper: Float,
}

impl Default for Bounds {
    /// The default [Bounds] for some floating point number.
    /// \[MIN, MAX\]
    fn default() -> Self {
        #[cfg(feature = "f32")]
        return Self {
            lower: f32::MIN,
            upper: f32::MAX,
        };

        #[cfg(not(feature = "f32"))]
        return Self {
            lower: f64::MIN,
            upper: f64::MAX,
        };
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
}

/// A [Temperature] that cannot be invalid. 
/// 
/// It also stores bounds which 
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CheckedTemperature {
    temp: Temperature,
    bounds: Bounds,
}

impl CheckedTemperature {
    /// Checks a temperature for problems, such as being below abs. zero or
    /// being out of bounds!
    fn check(&self, temp: Temperature) -> Result<(), CheckedTempError> {
        if temp.check_abs_zero() {
            return Err(CheckedTempError::BelowAbsoluteZero(temp.get_inner()));
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
    pub fn new(temp: Temperature) -> Result<CheckedTemperature, CheckedTempError> {
        if temp.check_abs_zero() {
            return Err(CheckedTempError::BelowAbsoluteZero(temp.get_inner()));
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
    pub fn change(&mut self, new: Temperature) -> Result<(), CheckedTempError> {
        self.check(new)?;

        self.temp = new;
        Ok(())
    }

    /// Tries to add two temperatures together.
    pub fn add(&mut self, temp: Temperature) -> Result<(), CheckedTempError> {
        let result = self.temp + temp;
        self.check(result)?;

        self.temp = result;
        Ok(())
    }

    /// Tries to subtract using two temperatures.
    pub fn sub(&mut self, temp: Temperature) -> Result<(), CheckedTempError> {
        let result = self.temp - temp;
        self.check(result)?;

        self.temp = result;
        Ok(())
    }

    /// Tries to multiply a temperature by another number.
    pub fn mul(&mut self, num: Float) -> Result<(), CheckedTempError> {
        let result = self.temp * num;
        self.check(result)?;

        self.temp = result;
        Ok(())
    }

    /// Tries to divide a temperature by another number.
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
    pub fn set_upper_bound(&mut self, bound: Float) -> Result<(), CheckedTempError> {
        self.bounds.set_upper(bound)?;
        Ok(())
    }

    /// Tries to set the lower allowed bound to a given value.
    pub fn set_lower_bound(&mut self, bound: Float) -> Result<(), CheckedTempError> {
        self.bounds.set_lower(bound)?;
        Ok(())
    }
}
