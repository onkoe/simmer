#![cfg(feature = "checked")]
#![cfg(std)]
use simmer::{CheckedTemperature, Temperature};
use util::CharArrWriter;

extern crate alloc;

mod util;

#[test]
fn ufmt_display_print() {
    let mut buf = CharArrWriter::default();

    ufmt::uwrite!(
        &mut buf,
        "{}",
        CheckedTemperature::new(Temperature::Celsius(0.0)).unwrap()
    )
    .unwrap();

    assert_eq!(
        "0.00000",
        buf.to_char_iter()
            .copied()
            .collect::<alloc::string::String>()
            .trim()
    );

    buf.clear();
    ufmt::uwrite!(
        &mut buf,
        "{}",
        CheckedTemperature::new(Temperature::Celsius(42.13)).unwrap()
    )
    .unwrap();

    assert_eq!(
        "42.13000",
        buf.to_char_iter()
            .copied()
            .collect::<alloc::string::String>()
            .trim()
    );
}

#[test]
fn ufmt_debug_print() {
    let mut buf = CharArrWriter::default();

    ufmt::uwrite!(
        &mut buf,
        "{:?}",
        CheckedTemperature::new(Temperature::Celsius(0.0)).unwrap()
    )
    .unwrap();

    assert_eq!(
        "Temperature::Celsius(0.00000)",
        buf.to_char_iter()
            .copied()
            .collect::<alloc::string::String>()
            .trim()
    );

    buf.clear();
    ufmt::uwrite!(
        &mut buf,
        "{:?}",
        CheckedTemperature::new(Temperature::Fahrenheit(4.06)).unwrap()
    )
    .unwrap();

    assert_eq!(
        "Temperature::Fahrenheit(4.05999)",
        buf.to_char_iter()
            .copied()
            .collect::<alloc::string::String>()
            .trim()
    );
}
