/*
Copyright 2020 ETCDEV GmbH
Copyright 2020 EmeraldPay, Inc

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

///
/// Produces byte-array based struct
///```
///# #[macro_use] extern crate byte_array_struct; use std::str::FromStr; fn main() {
///byte_array_struct!(
///    // 24 is the size of the struct in bytes
///    pub struct Address(24);
///);
///
///let foo = Address::default();
///let foo = Address::from_str("0x00112233445566778899aabbccddeeff0123456789abcdef").unwrap();
///# }
///```
//
#[macro_export]
macro_rules! byte_array_struct {
    (
        $visibility:vis struct $name:ident ($num:expr);
    ) => {

        #[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
        $visibility struct $name([u8; $num]);

        impl ::std::ops::Deref for $name {
            type Target = [u8];

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::str::FromStr for $name {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let s_hex: &str = if s.len() == $num * 2 {
                    s
                } else if s.len() == $num * 2 + 2 && s.starts_with("0x") {
                    &s[2..]
                } else {
                    return Err(());
                };
                let hex = hex::decode(s_hex).map_err(|_| ())?;
                <$name as std::convert::TryFrom<Vec<u8>>>::try_from(hex)
            }
        }

        impl ToString for $name {
            fn to_string(&self) -> String {
                hex::encode(&self.0)
            }
        }

        impl From<[u8; $num]> for $name {
            fn from(bytes: [u8; $num]) -> Self {
                $name(bytes)
            }
        }

        impl std::convert::TryFrom<&[u8]> for $name {
            type Error = ();

            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                if value.len() != $num {
                    return Err(());
                }
                let mut result: [u8; $num] = [0; $num];
                result.copy_from_slice(value);
                Ok(result.into())
            }
        }

        impl std::convert::TryFrom<Vec<u8>> for $name {
            type Error = ();

            fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
                if value.len() != $num {
                    return Err(());
                }
                let mut result: [u8; $num] = [0; $num];
                result.copy_from_slice(&value);
                Ok(result.into())
            }
        }

        impl Into<Vec<u8>> for $name {
            fn into(self) -> Vec<u8> {
                self.0.to_vec()
            }
        }

        impl Into<[u8; $num]> for $name {
            fn into(self) -> [u8; $num] {
                self.0
            }
        }

        #[cfg(feature = "serialize")]
        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<$name, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                use hex::FromHex;
                let v = String::deserialize(deserializer)
                    .and_then(|s| Vec::from_hex(s).map_err(::serde::de::Error::custom))?;

                if v.len() != $num {
                    return Err(::serde::de::Error::custom(&format!(
                        "Byte array invalid length: {}",
                        v.len()
                    )));
                }

                let mut bytes = [0u8; $num];
                bytes.copy_from_slice(&v);

                Ok($name(bytes))
            }
        }

        #[cfg(feature = "serialize")]
        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                use hex;
                serializer.serialize_str(&hex::encode(&self.0))
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::convert::TryFrom;

    byte_array_struct!(
        struct Bytes8(8);
    );
    byte_array_struct!(
        pub struct Bytes20(20);
    );
    byte_array_struct!(
        struct Bytes24 (24);
    );
    byte_array_struct!(
        struct Bytes32(32);
    );

    #[test]
    fn creates_8byte_struct() {
        let act = Bytes8::default();
        assert_eq!(act.0.len(), 8);
    }

    #[test]
    fn creates_20byte_struct() {
        let act = Bytes20::default();
        assert_eq!(act.0.len(), 20);
    }

    #[test]
    fn creates_24byte_struct() {
        let act = Bytes24::default();
        assert_eq!(act.0.len(), 24);
    }

    #[test]
    fn creates_32byte_struct() {
        let act = Bytes32::default();
        assert_eq!(act.0.len(), 32);
    }

    #[test]
    fn create_from_string() {
        let act = Bytes20::from_str("00112233445566778899aabbccddeeff00112233").unwrap();
        assert_eq!(act.to_string(), "00112233445566778899aabbccddeeff00112233");
        assert_eq!(act.0.len(), 20);

        let act = Bytes24::from_str("0x00112233445566778899aabbccddeeff0011223344556677").unwrap();
        assert_eq!(act.to_string(), "00112233445566778899aabbccddeeff0011223344556677");
        assert_eq!(act.0.len(), 24);
    }

    #[test]
    fn fail_on_invalid_size_string() {
        let act = Bytes20::from_str("00112233445566778899aabbccddeeff0011223344");
        assert!(act.is_err());

        let act = Bytes20::from_str("00112233445566778899aabbccddeeff001122");
        assert!(act.is_err());
    }

    #[test]
    fn fail_on_non_hex_string() {
        let act = Bytes20::from_str("00112233445566778899aabbccddeeff1234qwer");
        assert!(act.is_err());

        let act = Bytes20::from_str("0_00112233445566778899aabbccddeeff00112233");
        assert!(act.is_err());
    }

    #[test]
    fn fail_on_empty_string() {
        let act = Bytes20::from_str("");
        assert!(act.is_err());
    }

    #[test]
    fn create_from_bytes() {
        let input = hex::decode("00112233445566778899aabbccddeeff00112233").unwrap();
        let input = input.as_slice();
        let act = Bytes20::try_from(input).unwrap();
        assert_eq!(act.to_string(), "00112233445566778899aabbccddeeff00112233");
        assert_eq!(act.0.len(), 20);
    }

    #[test]
    fn create_from_vec() {
        let input = hex::decode("00112233445566778899aabbccddeeff00112233").unwrap();
        let act = Bytes20::try_from(input).unwrap();
        assert_eq!(act.to_string(), "00112233445566778899aabbccddeeff00112233");
        assert_eq!(act.0.len(), 20);
    }

    #[test]
    fn fail_to_create_incorrect_arr() {
        let input = hex::decode("00112233445566778899aabbccddeeff").unwrap();
        let act = Bytes20::try_from(input.as_slice());
        assert!(act.is_err());

        let input = hex::decode("00112233445566778899aabbccddeeff00112233").unwrap();
        let act = Bytes8::try_from(input.as_slice());
        assert!(act.is_err());
    }

    #[test]
    fn fail_to_create_incorrect_vec() {
        let input = hex::decode("00112233445566778899aabbccddeeff").unwrap();
        let act = Bytes20::try_from(input);
        assert!(act.is_err());

        let input = hex::decode("00112233445566778899aabbccddeeff00112233").unwrap();
        let act = Bytes8::try_from(input);
        assert!(act.is_err());
    }

    #[test]
    fn convert_into_vec() {
        let input = hex::decode("00112233445566778899aabbccddeeff00112233").unwrap();
        let act = Bytes20::try_from(input.clone()).unwrap();
        let output: Vec<u8> = act.into();
        assert_eq!(output, input);
    }

    #[test]
    fn convert_into_arr() {
        let input = hex::decode("00112233445566778899aabbccddeeff00112233").unwrap();
        let act = Bytes20::try_from(input.clone()).unwrap();
        let output: [u8; 20] = act.into();
        assert_eq!(output, input.as_slice());
    }
}

#[cfg(all(test, feature = "serialize"))]
mod tests_serde {

    byte_array_struct!(
        struct Hex8(8);
    );

    #[test]
    fn encode_default_byte_array() {
        assert_eq!(
            serde_json::to_string(&Hex8::default()).unwrap(),
            "\"0000000000000000\""
        );
    }

    #[test]
    fn decode_zero_byte_array() {
        assert_eq!(
            serde_json::from_str::<Hex8>("\"0000000000000000\"").unwrap(),
            Hex8::default()
        );
    }

    #[test]
    fn encode_byte_array() {
        let hex = Hex8::from([0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);

        assert_eq!(serde_json::to_string(&hex).unwrap(), "\"0123456789abcdef\"");
    }

    #[test]
    fn decode_byte_array() {
        let hex = Hex8::from([0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);

        assert_eq!(
            serde_json::from_str::<Hex8>("\"0123456789abcdef\"").unwrap(),
            hex
        );
    }

    #[test]
    fn not_decode_invalid_byte_array() {
        assert!(serde_json::from_str::<Hex8>("\"__23456789abcdef\"").is_err());
    }

    #[test]
    fn not_decode_insufficient_byte_array() {
        assert!(serde_json::from_str::<Hex8>("1234567890").is_err());
    }

    #[test]
    fn not_decode_empty_text() {
        assert!(serde_json::from_str::<Hex8>("\"\"").is_err());
    }

    #[test]
    fn not_decode_absent_text() {
        assert!(serde_json::from_str::<Hex8>("").is_err());
    }
}
