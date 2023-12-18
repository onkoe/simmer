#[allow(unused_imports)]
use simmer::Temperature;

extern crate alloc;

// create a String newtype to be a `ufmt` buffer 🤤
struct StringBuf(alloc::string::String);

impl ufmt_write::uWrite for StringBuf {
    type Error = ();

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.0.push_str(s);
        Ok(())
    }
}

#[cfg(feature = "f32")]
fn main() {
    let mut buf = StringBuf(alloc::string::String::new());

    let ice = Temperature::Fahrenheit(32_f32);
    ufmt::uwriteln!(buf, "ice is {} degrees fahrenheit!", ice).unwrap();

    let ice_c = ice.to_celsius();
    ufmt::uwriteln!(buf, "ice is {} degrees celsius!", ice_c).unwrap();

    // i want that cool number
    let ice_raw_c: f32 = ice_c.into();
    ufmt::uwriteln!(
        buf,
        "the best number is {}!",
        ufmt_float::uFmt_f32::Five(ice_raw_c)
    )
    .unwrap();

    assert_eq!(
        buf.0,
        "ice is 32.00000 degrees fahrenheit!\nice is 0.00000 degrees celsius!\nthe best number is 0.00000!\n"
    );
}

// shhhh
// i know this is dumb. compile with `--features f32`
#[cfg(not(feature = "f32"))]
fn main() {}
