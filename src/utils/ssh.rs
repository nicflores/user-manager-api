use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use rand::thread_rng;
use rsa::pkcs8::EncodePrivateKey;
use rsa::traits::PublicKeyParts;
use rsa::RsaPrivateKey;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SSHKeyPair {
    pub private_key: String,
    pub public_key: String,
}

/// Generate 2048-bit RSA keys used to authenticate to SFTP.
impl SSHKeyPair {
    pub fn new() -> Self {
        let bits = 2048;
        let mut rng = thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, bits).unwrap();
        let public_key = private_key.to_public_key();

        let private_key_pem = private_key.to_pkcs8_pem(Default::default()).unwrap();

        let e = public_key.e().to_bytes_be();
        let n = public_key.n().to_bytes_be();

        let mut buffer = Vec::new();
        let key_type = "ssh-rsa";

        // Append Key type
        buffer.extend_from_slice(&(key_type.len() as u32).to_be_bytes());
        buffer.extend_from_slice(key_type.as_bytes());

        // Append exponent
        buffer.extend_from_slice(&(e.len() as u32).to_be_bytes());
        buffer.extend_from_slice(&e);

        // Append modulus
        buffer.extend_from_slice(&(n.len() as u32).to_be_bytes());
        buffer.extend_from_slice(&n);

        let encoded_public_key = STANDARD.encode(&buffer);
        let encoded_private_key = STANDARD.encode(&private_key_pem);
        let public_ssh_key = format!("{} {}", key_type, encoded_public_key);

        Self {
            private_key: encoded_private_key,
            public_key: public_ssh_key,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ssh_key_pair() {
        let key_pair = SSHKeyPair::new();
        //println!("Private Key: {}", key_pair.private_key);
        //println!("Public Key: {}", key_pair.public_key);
        assert!(key_pair.public_key.starts_with("ssh-rsa"));
    }
}
