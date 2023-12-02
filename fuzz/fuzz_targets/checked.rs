#![no_main]

use libfuzzer_sys::fuzz_target;
use simmer::{CheckedTemperature, Temperature};

fuzz_target!(|input: Temperature| {
    let temp = CheckedTemperature::new(input);

    if let Ok(mut t) = temp {
        assert!(t.to_kelvin().unwrap().get_inner() >= 0.0);
    }
});
