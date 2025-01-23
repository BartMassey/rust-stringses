use thiserror::Error;

#[derive(Debug, Error)]
pub enum StringerError {
    #[error("failed conversion")]
    FailedConversion,
}

pub trait Stringses {
    type Letter;
}

pub struct Stringer<T: Stringses> {
    letters: Vec<T::Letter>,
}

pub struct Asciies;

impl Stringses for Asciies {
    type Letter = u8;
}

pub struct Utf8ses;

impl Stringses for Utf8ses {
    type Letter = u8;
}

impl<T: Stringses> Default for Stringer<T> {
    fn default() -> Self {
        Self { letters: Vec::new() }
    }
}

impl<T: Stringses> Stringer<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TryFrom<Stringer<Utf8ses>> for Stringer<Asciies> {
    type Error = StringerError;

    fn try_from(value: Stringer<Utf8ses>) -> Result<Self, Self::Error> {
        for &b in &value.letters {
            if b > 127 {
                return Err(StringerError::FailedConversion);
            }
        }
        // XXX This could just be a transmute.
        Ok(Self { letters: value.letters })
    }
}
