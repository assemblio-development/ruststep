use super::*;
use crate::ast::*;
use inflector::Inflector;
use serde::{
    de::{self, IntoDeserializer},
    forward_to_deserialize_any,
};

impl<'de, 'param> de::Deserializer<'de> for &'param Parameter {
    type Error = crate::error::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Parameter::Typed { keyword, parameter } => {
                visitor.visit_map(RecordDeserializer::new(keyword, parameter))
            }
            Parameter::Integer(val) => visitor.visit_i64(*val),
            Parameter::Real(val) => visitor.visit_f64(*val),
            Parameter::String(val) => visitor.visit_str(val),
            Parameter::List(params) => visitor.visit_seq(SeqDeserializer::new(params)),
            Parameter::Ref(name) => visitor.visit_enum(name),
            Parameter::NotProvided | Parameter::Omitted => visitor.visit_none(),
            Parameter::Enumeration(variant) => {
                visitor.visit_enum(variant.to_class_case().into_deserializer())
            }
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        struct tuple_struct map enum identifier ignored_any
    }
}

#[derive(Debug)]
pub struct SeqDeserializer {
    parameters: Vec<Parameter>,
}

impl SeqDeserializer {
    pub fn new(parameters: &[Parameter]) -> Self {
        SeqDeserializer {
            parameters: parameters.iter().rev().cloned().collect(),
        }
    }
}

impl<'de> de::SeqAccess<'de> for SeqDeserializer {
    type Error = crate::error::Error;

    fn size_hint(&self) -> Option<usize> {
        Some(self.parameters.len())
    }

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if let Some(last) = self.parameters.pop() {
            let value = seed.deserialize(&last)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}