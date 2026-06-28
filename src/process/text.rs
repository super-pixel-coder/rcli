use crate::TextSignFormat;
use crate::get_reader;
use crate::process::gen_pass;
use base64::prelude::*;
use ed25519_dalek::Signature;
use ed25519_dalek::Signer;
use ed25519_dalek::SigningKey;
use ed25519_dalek::Verifier;
use ed25519_dalek::VerifyingKey;

struct Blake3 {
    key: [u8; 32],
}

struct Ed25519Signer {
    key: SigningKey,
}

struct Ed25519Verifier {
    key: VerifyingKey,
}

trait TextSign {
    fn sign(&self, reader: &mut dyn std::io::Read) -> anyhow::Result<Vec<u8>>;
}

trait TextVerify {
    fn verify(&self, reader: &mut dyn std::io::Read, sig: &[u8]) -> anyhow::Result<bool>;
}

pub trait KeyLoader {
    fn load(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self>
    where
        Self: Sized;
}

pub trait KeyGenerator {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>>;
}

pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> anyhow::Result<String> {
    let mut reader = get_reader(input)?;

    let signer: Box<dyn TextSign> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::load(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Signer::load(key)?),
    };
    let signed = signer.sign(&mut reader)?;
    Ok(BASE64_URL_SAFE_NO_PAD.encode(&signed))
}

pub fn process_text_verify(
    input: &str,
    key: &str,
    format: TextSignFormat,
    sig: &str,
) -> anyhow::Result<bool> {
    let mut reader = get_reader(input)?;
    let sig = BASE64_URL_SAFE_NO_PAD.decode(sig)?;
    let verifier: Box<dyn TextVerify> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::load(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Verifier::load(key)?),
    };
    verifier.verify(&mut reader, &sig)
}

pub fn process_text_generate(format: TextSignFormat) -> anyhow::Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn std::io::Read) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, reader: &mut dyn std::io::Read, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        let hash = blake3::keyed_hash(&self.key, &buf);
        let hash = hash.as_bytes();
        Ok(hash == sig)
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn std::io::Read) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = self.key.sign(&buf);
        Ok(sig.to_bytes().to_vec())
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, reader: &mut dyn std::io::Read, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = Signature::from_bytes(sig.try_into()?);
        Ok(self.key.verify(&buf, &sig).is_ok())
    }
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let signer = Blake3::new(key);
        Ok(signer)
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let key = std::fs::read(path)?;
        Blake3::try_new(&key)
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>> {
        let key = gen_pass::process_genpass(32, true, true, true, true)?;
        Ok(vec![key.into()])
    }
}

impl Ed25519Signer {
    pub fn new(key: [u8; 32]) -> Self {
        Ed25519Signer {
            key: SigningKey::from_bytes(&key),
        }
    }

    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        Ok(Ed25519Signer::new(key))
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let key = std::fs::read(path)?;
        Ed25519Signer::try_new(&key)
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>> {
        let mut csprng = rand_core::OsRng;
        let sk: SigningKey = SigningKey::generate(&mut csprng);
        let pk = sk.verifying_key().to_bytes().to_vec();
        let sk = sk.to_bytes().to_vec();
        Ok(vec![sk, pk])
    }
}

impl Ed25519Verifier {
    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = (&key[..32]).try_into()?;
        let key = VerifyingKey::from_bytes(key)?;
        Ok(Self { key })
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let key = std::fs::read(path)?;
        Ed25519Verifier::try_new(&key)
    }
}

#[cfg(test)]
mod tests {
    use crate::process::text::{Ed25519Signer, Ed25519Verifier, KeyLoader, TextSign, TextVerify};

    #[test]
    fn test_ed25519_sign_verify() -> anyhow::Result<()> {
        let sk = Ed25519Signer::load("fixtures/ed25519.sk")?;
        let pk = Ed25519Verifier::load("fixtures/ed25519.pk")?;

        let data = b"hello world";
        let sig = sk.sign(&mut &data[..])?;
        assert!(pk.verify(&mut &data[..], &sig)?);
        Ok(())
    }
}
