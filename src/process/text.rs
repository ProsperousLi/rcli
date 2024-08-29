use rand::rngs::OsRng;
use std::{collections::HashMap, io::Read};

use crate::TextSignFormat;
use anyhow::{Ok, Result};
use ed25519::Signature;
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};

use super::process_genpass;

pub trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerify {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool>;
}

// pub trait KeyLoader {
//     fn load(path: impl AsRef<Path>) -> Result<Self>
//     where
//         Self: Sized;
// }

// pub trait KeyGenerator {
//     fn generate() -> Result<Vec<Vec<u8>>> {
//         let key = process_genpass(32, true, true, true, true)?;
//         let key = key.as_bytes().to_vec();
//         Ok(vec![key])
//     }
// }

pub struct Blake3 {
    key: [u8; 32],
}

pub struct Ed25519Signer {
    key: SigningKey,
}

pub struct Ed25519Verifyer {
    key: VerifyingKey,
}

// impl KeyGenerator for Blake3 {
//     fn generate() -> Result<Vec<Vec<u8>>> {
//         let mut csprng = OsRng;
//         let key = SigningKey::generate(&mut csprng);
//         let pk = key.verifying_key().to_bytes().to_vec();
//         let sk = key.to_bytes().to_vec();

//         Ok(vec![sk, pk])
//     }
// }

// impl KeyGenerator for Ed25519Signer {
//     fn generate() -> Result<Vec<Vec<u8>>> {
//         let mut csprng = OsRng;
//         let key = SigningKey::generate(&mut csprng);
//         let pk = key.verifying_key().to_bytes().to_vec();
//         let sk = key.to_bytes().to_vec();

//         Ok(vec![sk, pk])
//     }
// }

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        let _ = reader.read_to_end(&mut buf);
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

// impl KeyLoader for Blake3 {
//     fn load(path: impl AsRef<Path>) -> Result<Self> {
//         let key = fs::read(path)?;
//         Self::try_new(&key)
//     }
// }

// impl KeyLoader for Ed25519Signer {
//     fn load(path: impl AsRef<Path>) -> Result<Self> {
//         let key = fs::read(path)?;
//         Self::try_new(&key)
//     }
// }

// impl KeyLoader for Ed25519Verifyer {
//     fn load(path: impl AsRef<Path>) -> Result<Self> {
//         let key = fs::read(path)?;
//         Self::try_new(&key)
//     }
// }

impl Blake3 {
    // pub fn new(key: [u8; 32]) -> Self {
    //     Self { key }
    // }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into().unwrap();
        let signer = Blake3 { key };
        Ok(signer)
    }

    fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {
        let key = process_genpass(32, true, true, true, true)?;
        let mut map = HashMap::new();
        map.insert("blake3.txt", key.as_bytes().to_vec());
        Ok(map)
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        let hash = hash.as_bytes();
        Ok(hash == sig)
    }
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = SigningKey::from_bytes(key.try_into()?);

        let signer = Ed25519Signer::new(key);

        Ok(signer)
    }

    fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {
        let mut csprng = OsRng;
        let sk: SigningKey = SigningKey::generate(&mut csprng);
        let pk: VerifyingKey = (&sk).into();
        let mut map = HashMap::new();
        map.insert("ed25519.sk", sk.to_bytes().to_vec());
        map.insert("ed25519.pk", pk.to_bytes().to_vec());

        Ok(map)
    }
}

impl Ed25519Verifyer {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;

        let verify = Ed25519Verifyer::new(key);

        Ok(verify)
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = self.key.sign(&buf);
        Ok(sig.to_bytes().to_vec())
    }
}

impl TextVerify for Ed25519Verifyer {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = Signature::from_bytes(sig.try_into()?);
        let ret = self.key.verify(&buf, &sig).is_ok();
        Ok(ret)
    }
}
pub fn process_text_sign(
    reader: &mut dyn Read,
    key: &[u8], // (ptr, length)
    format: TextSignFormat,
) -> Result<Vec<u8>> {
    let signer: Box<dyn TextSign> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Signer::try_new(key)?),
    };

    signer.sign(reader)
}

pub fn process_text_verify(
    reader: &mut dyn Read,
    key: &[u8],
    sig: &[u8],
    format: TextSignFormat,
) -> Result<bool> {
    let verifier: Box<dyn TextVerify> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Verifyer::try_new(key)?),
    };
    verifier.verify(reader, sig)
}

pub fn process_text_generate(format: TextSignFormat) -> Result<HashMap<&'static str, Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

#[cfg(test)]
mod tests {
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

    use super::*;
    const KEY: &[u8] = include_bytes!("../../fixtures/blake3.txt");

    #[test]

    fn test_process_text_sign() -> Result<()> {
        let mut reader = "hello".as_bytes();
        let mut reader1 = "hello".as_bytes();
        let format = TextSignFormat::Blake3;
        let sig = process_text_sign(&mut reader, KEY, format)?;
        let ret = process_text_verify(&mut reader1, KEY, &sig, format)?;
        assert!(ret);
        Ok(())
    }

    #[test]
    fn test_process_text_verify() -> Result<()> {
        let mut reader = "hello".as_bytes();
        let format = TextSignFormat::Blake3;
        let sig = "33Ypo4rveYpWmJKAiGnnse-wHQhMVujjmcVkV4Tl43k";
        let sig: Vec<u8> = URL_SAFE_NO_PAD.decode(sig)?;
        let ret = process_text_verify(&mut reader, KEY, &sig, format)?;
        assert!(ret);
        Ok(())
    }
}
