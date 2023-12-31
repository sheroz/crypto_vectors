//! Test Vectors for [RFC8645, CTR-ACPKM](https://www.rfc-editor.org/rfc/rfc8645.html#section-6.2.2) 
//! 
//! Test Vectors taken from [Р 1323565.1.017—2018](https://standartgost.ru/g/%D0%A0_1323565.1.017-2018),
//! Page 11,
//! Section A.1

/// Cipher Key
pub const CIPHER_KEY: [u8;32] = [
    0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
    0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54, 0x32, 0x10, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF
];

/// Plaintext message
pub const PLAINTEXT: [u8;56] = [
    0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x00, 0xFF, 0xEE, 0xDD, 0xCC, 0xBB, 0xAA, 0x99, 0x88,
    0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xEE, 0xFF, 0x0A,
    0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xEE, 0xFF, 0x0A, 0x00,
    0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99            
];

/// Ciphertext
pub const CIPHERTEXT: [u8;56] = [
    0x2A, 0xB8, 0x1D, 0xEE, 0xEB, 0x1E, 0x4C, 0xAB, 0x68, 0xE1, 0x04, 0xC4, 0xBD, 0x6B, 0x94, 0xEA,
    0xC7, 0x2C, 0x67, 0xAF, 0x6C, 0x2E, 0x5B, 0x6B, 0x0E, 0xAF, 0xB6, 0x17, 0x70, 0xF1, 0xB3, 0x2E,
    0xA1, 0xAE, 0x71, 0x14, 0x9E, 0xED, 0x13, 0x82, 0xAB, 0xD4, 0x67, 0x18, 0x06, 0x72, 0xEC, 0x6F,
    0x84, 0xA2, 0xF1, 0x5B, 0x3F, 0xCA, 0x72, 0xC1
];
