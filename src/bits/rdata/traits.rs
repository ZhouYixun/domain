//! Traits for record data.

use std::fmt;
use super::super::compose::ComposeBytes;
use super::super::error::{ComposeResult, ParseResult};
use super::super::flavor::{Flavor, FlatFlavor};
use super::super::iana::RRType;
use super::super::parse::ParseFlavor;


/// A trait for creating record data.
pub trait RecordData<F: Flavor>: fmt::Display + Sized {
    /// Returns the record type for this record data instance.
    fn rtype(&self) -> RRType;

    /// Appends the record data to the end of a buffer.
    fn compose<C: ComposeBytes>(&self, target: &mut C) -> ComposeResult<()>;
}

pub trait FlatRecordData<'a, F: FlatFlavor<'a>>: RecordData<F> {
    /// Parse the record data from a cursor if the type is right.
    ///
    /// If this record data type does not feel responsible for records of
    /// type `rtype`, it should return `Ok(None)`. Otherwise it should
    /// return something or an error if parsing fails.
    fn parse<P>(rtype: RRType, parser: &mut P) -> ParseResult<Option<Self>>
             where P: ParseFlavor<'a, F>;
}
