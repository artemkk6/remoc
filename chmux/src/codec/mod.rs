//! Codecs for transforming messages into wire format.

use bytes::Bytes;
use serde::{de::DeserializeOwned, Serialize};
use std::{error::Error, fmt};

use crate::receiver::DataBuf;

/// Boxed error that is send, sync and static.
pub type BoxError = Box<dyn Error + Send + Sync + 'static>;

/// Serialization error.
#[derive(Debug)]
pub struct SerializationError(pub BoxError);

impl SerializationError {
    /// Creates a new serialization error.
    pub fn new<E>(err: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self(Box::new(err))
    }
}

impl fmt::Display for SerializationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl Error for SerializationError {}

/// Deserialization error.
#[derive(Debug)]
pub struct DeserializationError(pub BoxError);

impl DeserializationError {
    /// Creates a new deserialization error.
    pub fn new<E>(err: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self(Box::new(err))
    }
}

impl fmt::Display for DeserializationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl Error for DeserializationError {}

/// Serializes items into bytes.
pub trait Serializer<Item>: Send + fmt::Debug
where
    Item: Serialize,
{
    /// Serializes the specified item into the data format.
    fn serialize(&self, item: &Item) -> Result<Bytes, SerializationError>;
}

/// Deserializes items from bytes.
pub trait Deserializer<Item>: Send + fmt::Debug
where
    Item: DeserializeOwned,
{
    /// Deserializes the specified data into an item.
    fn deserialize(&self, data: DataBuf) -> Result<Item, DeserializationError>;
}

/// Creates [Serializer]s and [Deserializer]s for converting any item type into the
/// multiplex message's content type.
pub trait CodecFactory: Clone + Send + Sync + fmt::Debug + 'static {
    /// Create a [Serializer] for the specified item type.
    fn serializer<Item: Serialize + 'static>(&self) -> Box<dyn Serializer<Item>>;

    /// Create a [Deserializer] for the specified item type.
    fn deserializer<Item: DeserializeOwned + 'static>(&self) -> Box<dyn Deserializer<Item>>;
}

#[cfg(feature = "json-codec")]
pub mod json;

#[cfg(feature = "bincode-codec")]
pub mod bincode;
