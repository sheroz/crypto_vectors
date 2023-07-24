# Cryptography Test Vectors

[![crates.io](https://img.shields.io/crates/v/crypto_vectors)](https://crates.io/crates/crypto_vectors)
[![docs](https://img.shields.io/docsrs/crypto_vectors)](https://docs.rs/crypto_vectors)
[![build & test](https://github.com/sheroz/crypto_vectors/actions/workflows/ci.yml/badge.svg)](https://github.com/sheroz/crypto_vectors/actions/workflows/ci.yml)
[![MIT](https://img.shields.io/github/license/sheroz/crypto_vectors)](https://github.com/sheroz/magma/tree/main/crypto_vectors/LICENSE.txt)

## Test Vectors for Block Ciphers

* [RFC8891, a.k.a GOST R 34.12-2015: Block Cipher "Magma"](src/gost/rfc8891.rs)
* [RFC5830, a.k.a GOST 28147-89](src/gost/rfc5831.rs)

## Test Vectors for Block Cipher Modes

[GOST R 34.13-2015](https://www.tc26.ru/standard/gost/GOST_R_3413-2015.pdf)

* [**ECB** - Electronic Codebook Mode](src/gost/r3413_2015.rs)
* [**CTR** - Counter Encryption Mode](src/gost/r3413_2015.rs)
* [**CTR-ACPKM** - Counter Encryption Mode as per RFC8645](src/gost/r1323565_1_017_2018/ctr_acpkm.rs)
* [**OFB** - Output Feedback Mode](src/gost/r3413_2015.rs)
* [**CBC** - Cipher Block Chaining Mode](src/gost/r3413_2015.rs)
* [**CFB** - Cipher Feedback Mode](src/gost/r3413_2015.rs)
* [**MAC** - Message Authentication Code Generation Mode](src/gost/r3413_2015.rs)
