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

#[cfg(not(feature = "f32"))]
type Float = f64;

#[cfg(feature = "f32")]
type Float = f32;

/// A value that's one of many common temperature units.
///
/// Wraps a floating point number to give it a unit!
/// You can create a new `Temperature` by putting a float value inside.
#[cfg_attr(feature = "f32", doc = "```ignore")]
#[cfg_attr(not(feature = "f32"), doc = "```")]
/// use simmer::Temperature;
///
/// let my_temp = Temperature::Celsius(0.0);
///```
#[derive(Clone, Debug, PartialEq)]
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
            Self::Fahrenheit(_) => self.clone(),
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
            Temperature::Celsius(_) => self.clone(),
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
            Temperature::Kelvin(_) => self.clone(),
        }
    }

    /// A discovery function that returns the inner type, consuming the outer Temperature type.
    /// Use `my_temp.into()` when possible.
    ///
    /// # Usage
    ///
    #[cfg_attr(feature = "f32", doc = "```ignore")]
    #[cfg_attr(not(feature = "f32"), doc = "```should_fail")]
    /// # use simmer::Temperature;
    /// #
    /// let my_temp = Temperature::Fahrenheit(98.6);
    /// let my_temp_float = my_temp.into_inner(); // moved my_temp. it doesn't exist now!
    ///
    /// println!("{my_temp} doesn't exist so this won't compile!!!");
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
    /// # use assert_approx_eq::assert_approx_eq;
    /// #
    /// let temp = Temperature::Kelvin(0.0);
    ///
    /// let temp_inner = temp.get_inner();
    ///
    /// println!("{temp:?}'s inner is {temp_inner}");
    pub fn get_inner(&self) -> Float {
        match self {
            Temperature::Fahrenheit(t) => *t,
            Temperature::Celsius(t) => *t,
            Temperature::Kelvin(t) => *t,
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
