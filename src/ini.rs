pub mod error;
pub mod parser;

use parser::parse_from_str;
use serde::{de, Deserialize, forward_to_deserialize_any};
use serde::de::{DeserializeSeed, Visitor};
use std::iter::Peekable;

pub type Result<T, E = error::IniError> = std::result::Result<T, E>;

pub struct Deserializer<'input> {
    events: Peekable<Box<dyn Iterator<Item = &'input str> + 'input>>,
}

impl<'input> Deserializer<'input> {
    fn new(events: impl Iterator<Item = &'input str> + 'input) -> Self {
        let events: Box<dyn Iterator<Item = &'input str>> = Box::new(events);
        Self {
            events: events.peekable(),
        }
    }
}

struct MapAccess<'a, 'de>(&'a mut Deserializer<'de>);

impl<'de, 'a> de::MapAccess<'de> for MapAccess<'a, 'de> {
    type Error = error::IniError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if self.0.events.peek().is_none() {
            return Ok(None);
        }
        seed.deserialize(&mut *self.0).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.0)
    }
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let events = parse_from_str(s);
    let mut deserializer = Deserializer::new(events);
    T::deserialize(&mut deserializer)
}

impl<'de> de::Deserializer<'de> for &mut Deserializer<'de> {
    type Error = error::IniError;
    
    // The format we're dealing with ONLY supports strings, so... Yeah.
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.events.next().ok_or(error::IniError::ExpectedString)?)
    }

    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(MapAccess(&mut self))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct enum ignored_any identifier
    }
}
