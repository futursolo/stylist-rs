use super::ComponentValue;
use std::ops::Deref;
use syn::parse::{ParseBuffer, Result as ParseResult};

// Implements an iterator over parsed component values instead of rust tokens
#[derive(Debug)]
pub struct ComponentValueStream<'a> {
    input: &'a ParseBuffer<'a>,
}

impl<'a> From<&'a ParseBuffer<'a>> for ComponentValueStream<'a> {
    fn from(input: &'a ParseBuffer<'a>) -> Self {
        Self { input }
    }
}

impl<'a> From<ComponentValueStream<'a>> for &'a ParseBuffer<'a> {
    fn from(stream: ComponentValueStream<'a>) -> Self {
        stream.input
    }
}

impl<'a> Deref for ComponentValueStream<'a> {
    type Target = ParseBuffer<'a>;
    fn deref(&self) -> &Self::Target {
        self.input
    }
}

impl<'a> Iterator for ComponentValueStream<'a> {
    type Item = ParseResult<ComponentValue>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() {
            return None;
        }
        Some(self.input.parse())
    }
}
