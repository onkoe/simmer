#![cfg(feature = "f32")]
use simmer::Temperature;

extern crate alloc;

#[test]
fn ufmt_display_print() {
    let mut buf = CharArrWriter::default();

    ufmt::uwrite!(&mut buf, "{}", Temperature::Celsius(0.0)).unwrap();

    assert_eq!(
        "0.00000",
        buf.to_char_iter()
            .copied()
            .collect::<alloc::string::String>()
            .trim()
    );

    buf.clear();
    ufmt::uwrite!(&mut buf, "{}", Temperature::Celsius(42.13)).unwrap();

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

    ufmt::uwrite!(&mut buf, "{:?}", Temperature::Celsius(0.0)).unwrap();

    assert_eq!(
        "Temperature::Celsius(0.00000)",
        buf.to_char_iter()
            .copied()
            .collect::<alloc::string::String>()
            .trim()
    );

    buf = CharArrWriter::default();

    ufmt::uwrite!(&mut buf, "{:?}", Temperature::Fahrenheit(4.06)).unwrap();

    assert_eq!(
        "Temperature::Fahrenheit(4.05999)",
        buf.to_char_iter()
            .copied()
            .collect::<alloc::string::String>()
            .trim()
    );
}

struct CharArrWriter {
    data: [char; 100],
    taken: usize,
}

impl CharArrWriter {
    fn to_char_iter(&self) -> impl Iterator<Item = &char> {
        return self.data.iter();
    }

    fn clear(&mut self) {
        for n in 0..=self.taken {
            self.data[n] = ' ';
        }
    }
}

impl Default for CharArrWriter {
    fn default() -> Self {
        Self {
            data: [' '; 100],
            taken: 0,
        }
    }
}

impl ufmt_write::uWrite for CharArrWriter {
    type Error = core::convert::Infallible;

    fn write_str(&mut self, s: &str) -> Result<(), <CharArrWriter as ufmt_write::uWrite>::Error> {
        for c in s.chars() {
            if self.taken < 100 {
                self.data[self.taken] = c;
                self.taken += 1;
            } else {
                break;
            }
        }

        Ok(())
    }
}
