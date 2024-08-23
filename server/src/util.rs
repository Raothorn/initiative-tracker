use std::marker::PhantomData;
use serde::{Serialize, Serializer};

pub trait Encoded {}
// Encoding
pub trait EncodedAs<U: Serialize> {
    fn encode(&self) -> U;
}

#[derive(Clone)]
pub struct Encoding<T, U> {
    raw_data: T,
    phantom: PhantomData<U>
}

impl<T: Clone, U> Encoding<T, U> {
    pub fn raw(&self) -> T {
        self.raw_data.clone()
    }
}

pub fn encoded<T, U>(value: T) -> Encoding<T, U> {
    Encoding { raw_data: value, phantom: PhantomData}
}

impl<T, U> Serialize for Encoding<T, U>
where
    T: EncodedAs<U>,
    U: Serialize
{

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        let encoding = self.raw_data.encode();
        encoding.serialize(serializer)
    }
}

