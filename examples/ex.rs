use simmer::Temperature;

fn main() {
    let ice = Temperature::Fahrenheit(32.0);
    println!("water freezes at {ice} degrees fahrenheit");

    let ice_c = ice.to_celsius();
    println!("water freezes at {ice_c} degrees celsius");

    // i want that cool number
    #[cfg(not(feature = "f32"))]
    let ice_raw_c: f64 = ice_c.into();

    #[cfg(feature = "f32")]
    let ice_raw_c: f32 = ice_c.into();

    println!("here's a number: {ice_raw_c}");
}
