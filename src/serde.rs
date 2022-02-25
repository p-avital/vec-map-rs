//! An optional implementation of serialization/deserialization. Reference
//! implementations used:
//!
//! - [Serialize][1].
//! - [Deserialize][2].
//!
//! [1]: https://github.com/serde-rs/serde/blob/97856462467db2e90cf368e407c7ebcc726a01a9/serde/src/ser/impls.rs#L601-L611
//! [2]: https://github.com/serde-rs/serde/blob/97856462467db2e90cf368e407c7ebcc726a01a9/serde/src/de/impls.rs#L694-L746

extern crate serde;

use crate::set::VecSet;
use crate::VecMap;

use self::serde::de::{Error, MapAccess, SeqAccess, Visitor};
use self::serde::ser::{SerializeMap, SerializeSeq};
use self::serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::fmt;
use std::marker::PhantomData;

impl<K, V> Serialize for VecMap<K, V>
where
    K: Serialize + Eq,
    V: Serialize,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self {
            state.serialize_entry(k, v)?;
        }
        state.end()
    }
}

#[allow(missing_docs)]
#[derive(Default)]
pub struct VecMapVisitor<K, V> {
    marker: PhantomData<VecMap<K, V>>,
}

impl<K, V> VecMapVisitor<K, V> {
    pub fn new() -> Self {
        VecMapVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de, K, V> Visitor<'de> for VecMapVisitor<K, V>
where
    K: Deserialize<'de> + Eq,
    V: Deserialize<'de>,
{
    type Value = VecMap<K, V>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a VecMap")
    }

    #[inline]
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(VecMap::new())
    }

    #[inline]
    fn visit_map<Visitor>(self, mut visitor: Visitor) -> Result<Self::Value, Visitor::Error>
    where
        Visitor: MapAccess<'de>,
    {
        let mut values = VecMap::with_capacity(visitor.size_hint().unwrap_or(0));

        while let Some((key, value)) = visitor.next_entry()? {
            values.insert(key, value);
        }

        Ok(values)
    }
}

impl<'de, K, V> Deserialize<'de> for VecMap<K, V>
where
    K: Deserialize<'de> + Eq,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<VecMap<K, V>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(VecMapVisitor::new())
    }
}

impl<K> Serialize for VecSet<K>
where
    K: Serialize + Eq,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_seq(Some(self.len()))?;
        for k in self {
            state.serialize_element(k)?;
        }
        state.end()
    }
}

#[allow(missing_docs)]
#[derive(Default)]
pub struct VecSetVisitor<K> {
    marker: PhantomData<VecSet<K>>,
}

impl<K> VecSetVisitor<K> {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        VecSetVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de, K> Visitor<'de> for VecSetVisitor<K>
where
    K: Deserialize<'de> + Eq,
{
    type Value = VecSet<K>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a VecSet")
    }

    #[inline]
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(VecSet::new())
    }

    #[inline]
    fn visit_seq<Visitor>(self, mut visitor: Visitor) -> Result<Self::Value, Visitor::Error>
    where
        Visitor: SeqAccess<'de>,
    {
        let mut values = VecSet::with_capacity(visitor.size_hint().unwrap_or(0));

        while let Some(key) = visitor.next_element()? {
            values.insert(key);
        }

        Ok(values)
    }
}

impl<'de, K> Deserialize<'de> for VecSet<K>
where
    K: Deserialize<'de> + Eq,
{
    fn deserialize<D>(deserializer: D) -> Result<VecSet<K>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(VecSetVisitor::new())
    }
}
