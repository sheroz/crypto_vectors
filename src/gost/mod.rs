//! Test vectors, related to **GOST** ciphers
//! * [RFC 8891](https://datatracker.ietf.org/doc/html/rfc8891.html) a.k.a GOST R 34.12-2015: Block Cipher "Magma"
//! * [RFC 5830](https://datatracker.ietf.org/doc/html/rfc5830) a.k.a GOST 28147-89
//! * Block Cipher Modes:
//!   * [GOST R 34.13-2015](https://www.tc26.ru/standard/gost/GOST_R_3413-2015.pdf)
//!   * CTR-ACPKM: [RFC8645](https://www.rfc-editor.org/rfc/rfc8645.html#section-6.2.2), [Р 1323565.1.017—2018](https://standartgost.ru/g/%D0%A0_1323565.1.017-2018)

pub mod rfc8891;
pub mod rfc5831;
pub mod r3413_2015;
pub mod r1323565_1_017_2018;