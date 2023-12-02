pub(crate) struct CharArrWriter {
    data: [char; 100],
    taken: usize,
}

impl CharArrWriter {
    #[allow(unused)]
    pub(crate) fn to_char_iter(&self) -> impl Iterator<Item = &char> {
        return self.data.iter();
    }

    #[allow(unused)]
    pub(crate) fn clear(&mut self) {
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
