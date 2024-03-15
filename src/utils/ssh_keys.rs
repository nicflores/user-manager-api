use anyhow::{anyhow, Result};
use rand::rngs::OsRng;
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};
//use rsa::traits::PaddingScheme;
use rsa::{RsaPrivateKey, RsaPublicKey};

pub struct SSHKeyPair {
    pub private_key: String,
    pub public_key: String,
}

/// Generates an RSA private and public key pair.
pub fn generate_key_pair() -> Result<SSHKeyPair> {
    let mut rng = OsRng;
    let bits = 2048; // Key size - can be adjusted based on security needs
    let private_key = RsaPrivateKey::new(&mut rng, bits)
        .map_err(|e| anyhow!("Failed to generate private key: {}", e))?;
    let public_key = RsaPublicKey::from(&private_key);

    // Encoding keys to PEM format
    let private_key_pem = private_key
        .to_pkcs8_pem(Default::default())
        .map_err(|e| anyhow!("Failed to encode private key to PEM: {}", e))?
        .to_string();
    let public_key_pem = public_key
        .to_public_key_pem(Default::default())
        .map_err(|e| anyhow!("Failed to encode public key to PEM: {}", e))?
        .to_string();

    Ok(SSHKeyPair {
        private_key: private_key_pem,
        public_key: public_key_pem,
    })
}
