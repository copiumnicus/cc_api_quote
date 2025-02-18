use derivative::Derivative;
use revm::primitives::{Address, U256};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QuoteIntent {
    pub from: Address,
    pub to: Address,
    #[serde_as(as = "HexOrDecimalU256")]
    pub input: U256,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QuoteIntentRes {
    #[serde_as(as = "HexOrDecimalU256")]
    pub executed_input: U256,
    #[serde_as(as = "HexOrDecimalU256")]
    pub executed_output: U256,
    pub call_from: Address,
    pub interactions: Vec<InteractionData>,
    pub block: u64,
}

#[serde_as]
#[derive(Clone, Deserialize, Eq, PartialEq, Serialize, Default, Derivative)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase")]
pub struct InteractionData {
    pub target: Address,
    #[serde_as(as = "HexOrDecimalU256")]
    pub value: U256,
    #[derivative(Debug(format_with = "debug_bytes"))]
    #[serde(with = "bytes_hex")]
    pub call_data: Vec<u8>,
}
fn debug_bytes(
    bytes: impl AsRef<[u8]>,
    formatter: &mut std::fmt::Formatter,
) -> Result<(), std::fmt::Error> {
    formatter.write_fmt(format_args!("{}", hex::to_str(bytes.as_ref().to_vec())))
}

mod hex {
    /// convert vector of bytes to lowercase hex string prefixed with `0x`
    pub fn to_str(v: Vec<u8>) -> String {
        format!("0x{}", to_str_no_pre(v))
    }

    pub fn to_str_no_pre(v: Vec<u8>) -> String {
        v.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<String>>()
            .join("")
            .to_string()
    }

    /// convert hex str to a vec of bytes
    pub fn to_vec(mut s: &str) -> Option<Vec<u8>> {
        if s.starts_with("0x") {
            s = &s[2..]
        }
        if s.len() % 2 != 0 {
            // not div by 2
            return None;
        }
        Some(
            (0..s.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
                .collect(),
        )
    }
}
use u256_hex_or_decimal::HexOrDecimalU256;
mod u256_hex_or_decimal {
    use revm::primitives::U256;
    use serde::{de, Deserializer, Serializer};
    use serde_with::{DeserializeAs, SerializeAs};
    use std::fmt;

    pub struct HexOrDecimalU256;

    impl<'de> DeserializeAs<'de, U256> for HexOrDecimalU256 {
        fn deserialize_as<D>(deserializer: D) -> Result<U256, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserialize(deserializer)
        }
    }

    impl SerializeAs<U256> for HexOrDecimalU256 {
        fn serialize_as<S>(source: &U256, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serialize(source, serializer)
        }
    }

    pub fn serialize<S>(value: &U256, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<U256, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor {}
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = U256;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    formatter,
                    "a u256 encoded either as 0x hex prefixed or decimal encoded string"
                )
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if s.starts_with("0x") {
                    U256::from_str_radix(s.trim_start_matches("0x"), 16).map_err(|err| {
                        de::Error::custom(format!("failed to decode {s:?} as hex u256: {err}"))
                    })
                } else {
                    U256::from_str_radix(s, 10).map_err(|err| {
                        de::Error::custom(format!("failed to decode {s:?} as decimal u256: {err}"))
                    })
                }
            }
        }

        deserializer.deserialize_str(Visitor {})
    }

    #[cfg(test)]
    mod tests {
        use serde::de::{
            value::{Error as ValueError, StrDeserializer},
            IntoDeserializer,
        };

        use super::*;

        #[test]
        fn test_deserialization() {
            let deserializer: StrDeserializer<ValueError> = "0x10".into_deserializer();
            assert_eq!(deserialize(deserializer), Ok(U256::from(16)));

            let deserializer: StrDeserializer<ValueError> = "10".into_deserializer();
            assert_eq!(deserialize(deserializer), Ok(U256::from(10)));
        }
    }
}

mod bytes_hex {
    //! Serialization of Vec<u8> to 0x prefixed hex string
    use super::hex;
    use serde::{de::Error, Deserialize, Deserializer, Serializer};
    use serde_with::{DeserializeAs, SerializeAs};
    use std::borrow::Cow;

    pub fn serialize<S, T>(bytes: T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: AsRef<[u8]>,
    {
        serializer.serialize_str(&hex::to_str(bytes.as_ref().to_vec()))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let prefixed_hex_str = Cow::<str>::deserialize(deserializer)?;
        hex::to_vec(&prefixed_hex_str).ok_or(D::Error::custom("invalid hex"))
    }

    pub struct BytesHex;

    impl<'de> DeserializeAs<'de, Vec<u8>> for BytesHex {
        fn deserialize_as<D>(deserializer: D) -> Result<Vec<u8>, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserialize(deserializer)
        }
    }

    impl SerializeAs<Vec<u8>> for BytesHex {
        fn serialize_as<S>(source: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serialize(source, serializer)
        }
    }

    #[cfg(test)]
    mod tests {

        #[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
        struct S {
            #[serde(with = "super")]
            b: Vec<u8>,
        }

        #[test]
        fn json() {
            let orig = S { b: vec![0, 1] };
            let serialized = serde_json::to_value(&orig).unwrap();
            let expected = serde_json::json!({
                "b": "0x0001"
            });
            assert_eq!(serialized, expected);
            let deserialized: S = serde_json::from_value(expected).unwrap();
            assert_eq!(orig, deserialized);
        }
    }
}
