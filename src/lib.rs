/*!
Rust Strings rethought. Goals:

* Support multiple encodings as transparently as possible.
* Provide reference-counting for string values.
* Provide copy-on-write for string values.

*/

use std::borrow::Cow;
use std::rc::Rc;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum StringerError {
    #[error("failed conversion")]
    FailedConversion,
}

pub trait Stringses {
    type Letter;
}

pub struct Stringer<'a, T: Stringses> {
    letters: Cow<'a, Rc<Vec<T::Letter>>>,
}

pub struct Asciies;

impl Stringses for Asciies {
    type Letter = u8;
}

pub struct Utf8ses;

impl Stringses for Utf8ses {
    type Letter = u8;
}

pub struct Unicodes;

impl Stringses for Unicodes {
    type Letter = char;
}

impl<T: Stringses> Default for Stringer<'_, T> {
    fn default() -> Self {
        Self { letters: Cow::Owned(Rc::new(Vec::new())) }
    }
}

impl<T: Stringses> Stringer<'_, T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a> TryFrom<Stringer<'a, Utf8ses>> for Stringer<'a, Asciies> {
    type Error = StringerError;

    fn try_from(value: Stringer<'a, Utf8ses>) -> Result<Self, Self::Error> {
        let letters: &[u8] = value.letters.as_ref();
        for &b in letters {
            if !b.is_ascii() {
                return Err(StringerError::FailedConversion);
            }
        }
        // XXX This could just be a transmute.
        Ok(Stringer { letters: value.letters })
    }
}

impl<T: Stringses> From<Vec<T::Letter>> for Stringer<'_, T> {
    fn from(value: Vec<T::Letter>) -> Self {
        Stringer { letters: Cow::Owned(Rc::new(value)) }
    }
}

impl<'a> From<Stringer<'a, Utf8ses>> for Stringer<'a, Unicodes> {
    fn from(value: Stringer<'a, Utf8ses>) -> Self {
        // Safety: Typestate proves UTF8-ness of letters.
        // Need: Avoid performance hit.
        let letters: &str = unsafe {
            std::str::from_utf8_unchecked(value.letters.as_ref())
        };
        let letters: Vec<char> = letters.chars().collect();
        Stringer::from(letters)
    }
}
