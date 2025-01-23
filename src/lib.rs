use std::marker::PhantomData;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum StringerError {
    #[error("failed conversion")]
    FailedConversion,
}

pub struct Stringer<G, T> {
    letters: Vec<G>,
    p: PhantomData<T>,
}

impl<G, T> Default for Stringer<G, T> {
    fn default() -> Self {
        Self { letters: Vec::new(), p: PhantomData }
    }
}

pub struct Asciies;
//pub struct Byteses;
pub struct Utf8ses;
//pub struct Charses;

impl<G: Default, T> Stringer<G, T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TryFrom<Stringer<u8, Utf8ses>> for Stringer<u8, Asciies> {
    type Error = StringerError;

    fn try_from(value: Stringer<u8, Utf8ses>) -> Result<Self, Self::Error> {
        for &b in &value.letters {
            if b > 127 {
                return Err(StringerError::FailedConversion);
            }
        }
        // XXX This could just be a transmute.
        Ok(Self { letters: value.letters, p: PhantomData })
    }
}
