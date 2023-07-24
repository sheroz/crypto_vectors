//! Test vectors for [RFC5830, a.k.a GOST 28147-89](https://www.rfc-editor.org/rfc/rfc5830)
//! 
//! Test vectors taken from [RFC5831](https://www.rfc-editor.org/rfc/rfc5831#section-7)
//! 
//! **Cipher Keys**
//! 
//! K1 (little-endian)  = 0x733D2C20 0x65686573 0x74746769 0x79676120 0x626E7373 0x20657369 0x326C6568 0x33206D54
//! 
//! K1 (big-endian) = [0x33206D54, 0x326C6568, 0x20657369, 0x626E7373, 0x79676120, 0x74746769, 0x65686573, 0x733D2C20]    
//! 
//! K2 (little-endian)  = 0x110C733D 0x0D166568 0x130E7474 0x06417967 0x1D00626E 0x161A2065 0x090D326C 0x4D393320
//! 
//! K2 (big-endian) = [0x4D393320, 0x090D326C, 0x161A2065, 0x1D00626E, 0x06417967, 0x130E7474, 0x0D166568, 0x110C733D]    
//! 
//! K3 (little-endian)  = 0x80B111F3 0x730DF216 0x850013F1 0xC7E1F941 0x620C1DFF 0x3ABAE91A 0x3FA109F2 0xF513B239
//! 
//! K3 (big-endian) = [0xF513B239, 0x3FA109F2, 0x3ABAE91A, 0x620C1DFF, 0xC7E1F941, 0x850013F1, 0x730DF216, 0x80B111F3]
//! 
//! K4 (little-endian)  = 0xA0E2804E 0xFF1B73F2 0xECE27A00 0xE7B8C7E1 0xEE1D620C 0xAC0CC5BA 0xA804C05E 0xA18B0AEC
//! 
//! K4 (big-endian) = [0xA18B0AEC, 0xA804C05E, 0xAC0CC5BA, 0xEE1D620C, 0xE7B8C7E1, 0xECE27A00, 0xFF1B73F2, 0xA0E2804E]
//! 
//! **Plaintext**
//! 
//! 0x0
//! 
//! 
//! **Outputs**
//! 
//! S1 = 0x42ABBCCE 0x32BC0B1B
//! 
//! S2 = 0x5203EBC8 0x5D9BCFFD
//! 
//! S3 = 0x8D345899 0x00FF0E28
//! 
//! S4 = 0xE7860419 0x0D2A562D

/// Cipher Key K1
pub const CIPHER_KEY1: [u32;8] = [0x33206D54, 0x326C6568, 0x20657369, 0x626E7373, 0x79676120, 0x74746769, 0x65686573, 0x733D2C20];
/// Cipher Key K2
pub const CIPHER_KEY2: [u32;8] = [0x4D393320, 0x090D326C, 0x161A2065, 0x1D00626E, 0x06417967, 0x130E7474, 0x0D166568, 0x110C733D];    
/// Cipher Key K3
pub const CIPHER_KEY3: [u32;8] = [0xF513B239, 0x3FA109F2, 0x3ABAE91A, 0x620C1DFF, 0xC7E1F941, 0x850013F1, 0x730DF216, 0x80B111F3];
/// Cipher Key K4
pub const CIPHER_KEY4: [u32;8] = [0xA18B0AEC, 0xA804C05E, 0xAC0CC5BA, 0xEE1D620C, 0xE7B8C7E1, 0xECE27A00, 0xFF1B73F2, 0xA0E2804E];

/// Plaintext
pub const PLAINTEXT: u64 = 0;

/// Ciphertext1
pub const CIPHERTEXT1: u64 = 0x42ABBCCE32BC0B1B;
/// Ciphertext2
pub const CIPHERTEXT2: u64 = 0x5203EBC85D9BCFFD;
/// Ciphertext3
pub const CIPHERTEXT3: u64 = 0x8D34589900FF0E28;
/// Ciphertext4
pub const CIPHERTEXT4: u64 = 0xE78604190D2A562D;