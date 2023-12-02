# Simmer

A Rust library crate that expresses some standard units of temperature. It's compatible with embedded systems, happily converts between units, and unwraps internal values when you're ready to leave.

## Features

- `Temperature`, a simple wrapper for a float that stores some temperature
- A `CheckedTemperature` type, which only allows 'correct' temperatures
- `Display`, `Debug`, and the `ufmt` counterparts
- `f32` and `f64` support
- Embedded-aware and tested

## Usage

There's not much detail to this crate. You can get started with the code below!

```rust
use simmer::Temperature;

fn main() {
    let ice = Temperature::Fahrenheit(32.0);
    println!("water freezes at {ice} degrees fahrenheit");

    let ice_c = ice.to_celsius();
    println!("water freezes at {ice_c} degrees celsius");

    let ice_raw_c: f64 = ice_c.into();
    println!("here's a number: {ice_raw_c}");
}
```

### Checked

There's also a CheckedTemperature type to store and use temperatures safely. Here's a quick example of that:

```rust
fn main() -> anyhow::Result<()> {
    let ice = CheckedTemperature::new(Temperature::Fahrenheit(32.0))?;
    println!("water freezes at {ice} degrees fahrenheit");

    let ice_c = ice.to_celsius()?;
    let ice_raw_c: f64 = ice_c.into(); // can also use `f32` 😄
    println!("here's a number: {ice_raw_c}");
    
    Ok(())
}
```

## Contributions

If you feel that something is out of place (or you have a new feature), please feel free to submit a pull request! Particularly for bugs, you don't need to waste any time.

On the other hand, please create an issue before adding any new features. Thanks for your help in making this crate better! 🤩
