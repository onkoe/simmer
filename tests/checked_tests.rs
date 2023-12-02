#![cfg(feature = "checked")]
use assert_approx_eq::assert_approx_eq;
use simmer::{CheckedTemperature, Temperature};

// just like in the lib itself...
#[cfg(not(feature = "f32"))]
type Float = f64;

#[cfg(feature = "f32")]
type Float = f32;

/// This macro expects an argument order of (Fahrenheit, Celsius, Kelvin).
/// If that order isn't correct, you'll find that things don't work properly...
#[allow(unused)]
macro_rules! test_all {
    ($temp_f:expr, $temp_c:expr, $temp_k:expr) => {
        // test temp_f
        assert_approx_eq!(
            $temp_f,
            CheckedTemperature::new(Temperature::Celsius($temp_c))
                .unwrap()
                .to_fahrenheit()
                .unwrap()
                .into_inner()
        );
        assert_approx_eq!(
            $temp_f,
            CheckedTemperature::new(Temperature::Kelvin($temp_k))
                .unwrap()
                .to_fahrenheit()
                .unwrap()
                .into_inner()
        );
        assert_approx_eq!(
            $temp_f,
            CheckedTemperature::new(Temperature::Fahrenheit($temp_f))
                .unwrap()
                .to_fahrenheit()
                .unwrap()
                .into_inner()
        );

        // ok now temp_c
        assert_approx_eq!(
            $temp_c,
            CheckedTemperature::new(Temperature::Fahrenheit($temp_f))
                .unwrap()
                .to_celsius()
                .unwrap()
                .into_inner()
        );
        assert_approx_eq!(
            $temp_c,
            CheckedTemperature::new(Temperature::Kelvin($temp_k))
                .unwrap()
                .to_celsius()
                .unwrap()
                .into_inner()
        );
        assert_approx_eq!(
            $temp_c,
            CheckedTemperature::new(Temperature::Celsius($temp_c))
                .unwrap()
                .to_celsius()
                .unwrap()
                .into_inner()
        );

        // annnnd temp_k
        assert_approx_eq!(
            $temp_k,
            CheckedTemperature::new(Temperature::Fahrenheit($temp_f))
                .unwrap()
                .to_kelvin()
                .unwrap()
                .into_inner()
        );
        assert_approx_eq!(
            $temp_k,
            CheckedTemperature::new(Temperature::Celsius($temp_c))
                .unwrap()
                .to_kelvin()
                .unwrap()
                .into_inner()
        );
        assert_approx_eq!(
            $temp_k,
            CheckedTemperature::new(Temperature::Kelvin($temp_k))
                .unwrap()
                .to_kelvin()
                .unwrap()
                .into_inner()
        );
    };
}

#[allow(unused)]
pub(crate) use test_all;

#[test]
fn surface_of_sun() {
    let sun_f: Float = 9941.0;
    let sun_c: Float = 5505.0;
    let sun_k: Float = 5778.15;

    test_all!(sun_f, sun_c, sun_k);
}

#[test]
fn water_boils() {
    let water_f: Float = 212.0;
    let water_c: Float = 100.0;
    let water_k: Float = 373.15;

    test_all!(water_f, water_c, water_k);
}

#[test]
fn water_freezes() {
    let ice_f: Float = 32.0;
    let ice_c: Float = 0.0;
    let ice_k: Float = 273.15;

    test_all!(ice_f, ice_c, ice_k);
}

#[test]
#[should_panic]
fn zeroes() {
    // zero in different temps aren't the same 🥹
    let (zero_f, zero_c, zero_k): (Float, Float, Float) = (0.0, 0.0, 0.0);

    test_all!(zero_f, zero_c, zero_k);
}
