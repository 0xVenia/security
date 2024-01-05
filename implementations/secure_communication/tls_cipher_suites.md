# TLS Cipher Suites

## Overview

This document outlines the supported TLS cipher suites within the secure communication module of VENIA's blockchain implementation. TLS (Transport Layer Security) cipher suites define the algorithms used for secure communication, including key exchange, encryption, and message authentication.

## Supported Cipher Suites

The VENIA blockchain platform supports a range of secure TLS cipher suites to ensure robust and encrypted communication. The following is a comprehensive list of supported cipher suites:

### 1. TLS_RSA_WITH_AES_128_CBC_SHA256

- **Key Exchange:** RSA
- **Bulk Encryption Algorithm:** AES with a 128-bit key
- **Message Authentication Code (MAC):** SHA-256

### 2. TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384

- **Key Exchange:** Ephemeral Elliptic Curve Diffie-Hellman (ECDHE) with ECDSA
- **Bulk Encryption Algorithm:** AES with a 256-bit key in Galois/Counter Mode (GCM)
- **Message Authentication Code (MAC):** SHA-384

### 3. TLS_DHE_RSA_WITH_CHACHA20_POLY1305_SHA256

- **Key Exchange:** Ephemeral Diffie-Hellman (DHE) with RSA
- **Bulk Encryption Algorithm:** ChaCha20 with a 256-bit key in Poly1305 mode
- **Message Authentication Code (MAC):** SHA-256

## Configuring Cipher Suites

The configuration of TLS cipher suites in VENIA's blockchain implementation is performed in the `secure_communication` module. The cipher suites are specified in the configuration file or programmatically through the provided APIs.

### Example Configuration:

```rust
use venia_secure_communication::TlsConfig;

fn main() {
    // Initialize TLS configuration
    let mut tls_config = TlsConfig::new();

    // Set supported cipher suites
    tls_config.set_cipher_suites(&[
        "TLS_RSA_WITH_AES_128_CBC_SHA256",
        "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384",
        "TLS_DHE_RSA_WITH_CHACHA20_POLY1305_SHA256",
    ]);

    // ... Additional TLS configuration ...

    // Apply the TLS configuration to the secure communication module
    venia_secure_communication::apply_tls_config(tls_config);
}
