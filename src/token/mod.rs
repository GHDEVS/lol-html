mod capture;
mod impls;

use encoding_rs::Encoding;
use failure::Error;

pub use self::capture::{TokenCapture, TokenCaptureFlags, TokenCaptureResult};
pub use self::impls::*;

#[derive(Debug)]
pub enum Token<'i> {
    Text(Text<'i>),
    Comment(Comment<'i>),
    StartTag(StartTag<'i>),
    EndTag(EndTag<'i>),
    Doctype(Doctype<'i>),
    Eof,
}

pub struct TokenFactory {
    encoding: &'static Encoding,
}

impl TokenFactory {
    pub fn new(encoding: &'static Encoding) -> Self {
        TokenFactory { encoding }
    }

    #[inline]
    pub fn try_start_tag_from(
        &self,
        name: &str,
        attributes: &[(&str, &str)],
        self_closing: bool,
    ) -> Result<StartTag<'static>, Error> {
        StartTag::try_from(name, attributes, self_closing, self.encoding)
    }
}