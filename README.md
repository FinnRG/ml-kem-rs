# ml-kem-rs

Python wrapper around the [ml-kem](https://crates.io/crates/ml-kem) library using
PyO3, which implements the Module-Lattice-Based Key-Encapsulation
Mechanism Standard (formerly known as Kyber) as described in FIPS 203
using pure Rust.

## Usage

This library currently provides the three core functions for a
key encapsulation mechanism: `mlk768_generate`, `mlkem768_encapsulate` and
`mlkem768_decapsulate`.

```python

from ml_kem_rs import mlkem768_generate, mlkem768_encapsulate, mlkem768_decapsulate

dk, ek = mlkem768_generate()
ciphertext, ss1 = mlkem768_encapsulate(ek)
ss2 = mlkem768_decapsulate(dk, ciphertext)
```