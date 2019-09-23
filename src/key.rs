use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::convert::TryInto;
use std::hash::Hash;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Serialize, Deserialize)]
pub struct Key {
    value: [u8; 32],
}

impl Key {
    pub fn new<T: AsRef<[u8]>>(data: T) -> Key {
        Self {
            value: Sha256::digest(data.as_ref()).as_slice().try_into().unwrap(),
        }
    }

    pub fn value(&self) -> &[u8; 32] {
        &self.value
    }

    pub(crate) fn as_base64(&self) -> String {
        base64::encode_config(&self.value, base64::URL_SAFE_NO_PAD)
    }

    pub(crate) fn from_base64(encoded: &str) -> Option<Key> {
        let mut value = [0u8; 32];
        if base64::decode_config_slice(encoded, base64::URL_SAFE_NO_PAD, &mut value).is_ok() {
            Some(Self { value: value })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn key_test() {
        let k = Key::new("foo");
        assert_eq!(
            k.value(),
            &[
                0x2cu8, 0x26, 0xb4, 0x6b, 0x68, 0xff, 0xc6, 0x8f, 0xf9, 0x9b, 0x45, 0x3c, 0x1d,
                0x30, 0x41, 0x34, 0x13, 0x42, 0x2d, 0x70, 0x64, 0x83, 0xbf, 0xa0, 0xf9, 0x8a, 0x5e,
                0x88, 0x62, 0x66, 0xe7, 0xae
            ]
        );
        let k64 = k.as_base64();
        assert_eq!(&k64, "LCa0a2j_xo_5m0U8HTBBNBNCLXBkg7-g-YpeiGJm564");
        assert_eq!(Key::from_base64(&k64).unwrap(), k);
    }
}
