use serde::{Deserialize, Serialize};

use crate::Sob;

impl<'a, T> Serialize for Sob<'a, T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Sob::Owned(t) => t.serialize(serializer),
            Sob::Borrowed(t) => t.serialize(serializer),
        }
    }
}

impl<'a, 'de, T> Deserialize<'de> for Sob<'a, T>
where
    T: Serialize + for<'de_> Deserialize<'de_>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        T::deserialize(deserializer).map(Sob::Owned)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestData {
        data_1: u32,
        data_2: String,
        data_3: Vec<u8>,
    }

    #[test]
    fn basic() {
        let data = TestData {
            data_1: 35498,
            data_2: "data test".to_owned(),
            data_3: vec![255, 64, 8, 0],
        };
        let sob = Sob::Borrowed(&data);
        let serialized =
            serde_json::to_string(&sob).expect("sob with test data should be serializable");
        let deserialized: Sob<'_, TestData> =
            serde_json::from_str(&serialized).expect("sob with test data should be deserializable");

        assert_eq!(deserialized, Sob::Owned(data));
    }
}
