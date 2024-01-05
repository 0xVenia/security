// key_management.rs

use rand::rngs::OsRng;
use rsa::{PublicKey, RSAPrivateKey, PaddingScheme};
use aes::Aes256;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use zeroize::Zeroize;

const AES_KEY_SIZE: usize = 32; // 256 bits for AES
const RSA_KEY_BITS: usize = 2048; // RSA 2048 bits

pub struct KeyManager {
    rsa_private_key: RSAPrivateKey,
    aes_key: [u8; AES_KEY_SIZE],
}

impl KeyManager {
    pub fn new() -> Self {
        // Generate RSA private key
        let mut rng = OsRng;
        let rsa_private_key = RSAPrivateKey::new(&mut rng, RSA_KEY_BITS).expect("Failed to generate RSA key");

        // Generate AES key
        let aes_key = generate_aes_key();

        KeyManager { rsa_private_key, aes_key }
    }

    pub fn encrypt_with_rsa(&self, data: &[u8]) -> Vec<u8> {
        let padding = PaddingScheme::PKCS1v15Encrypt;
        self.rsa_private_key
            .encrypt(&mut OsRng, padding, data)
            .expect("RSA encryption failed")
    }

    pub fn decrypt_with_rsa(&self, encrypted_data: &[u8]) -> Vec<u8> {
        let padding = PaddingScheme::PKCS1v15Encrypt;
        self.rsa_private_key
            .decrypt(padding, encrypted_data)
            .expect("RSA decryption failed")
    }

    pub fn encrypt_with_aes(&self, data: &[u8], iv: &[u8; 16]) -> Vec<u8> {
        let cipher = Cbc::<Aes256, Pkcs7>::new_var(&self.aes_key, iv).expect("AES initialization failed");
        cipher.encrypt_vec(data)
    }

    pub fn decrypt_with_aes(&self, encrypted_data: &[u8], iv: &[u8; 16]) -> Vec<u8> {
        let cipher = Cbc::<Aes256, Pkcs7>::new_var(&self.aes_key, iv).expect("AES initialization failed");
        cipher.decrypt_vec(encrypted_data).expect("AES decryption failed")
    }
}

impl Drop for KeyManager {
    fn drop(&mut self) {
        // Zeroize sensitive data before deallocation
        self.aes_key.zeroize();
    }
}

fn generate_aes_key() -> [u8; AES_KEY_SIZE] {
    let mut key = [0u8; AES_KEY_SIZE];
    let mut rng = OsRng;
    rng.fill_bytes(&mut key);
    key
}

// TODO: Add additional functions for key rotation, secure storage operations, etc.
