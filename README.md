# ACM CALGO Algorithm 488 — Rust/Fortran equivalence

**Selected algorithm:** Richard P. Brent, *Algorithm 488: A Gaussian Pseudo-Random Number Generator*, *Communications of the ACM* 17(12), December 1974, pp. 704–706.

- ACM listing and Fortran archive: https://calgo.acm.org/488.zip
- Published source listing: https://maths-people.anu.edu.au/~brent/pd/rpb023.pdf
- ACM license notice: https://www.acm.org/publications/policies/softwarecrnotice

The code in this project is a newly written, attributed implementation of the **GRAND** method, not the original ACM-distributed Fortran file. It reproduces algorithmic steps and numerical table entries for educational parity testing. The ACM Software License Agreement governs the ACM-published software and may restrict redistribution/commercial use of derivative implementations. Consult the agreement and obtain necessary permissions before redistributing this port or using it commercially. This package is not represented as an ACM release or licensed under MIT.

## Input and output (identical CLI interface)

Both programs take exactly two command-line arguments:

1. `seed`: unsigned 32-bit integer between `0` and `4294967295`.
2. `count`: positive integer between `1` and `1000000`.

Each prints `count` single-precision Gaussian variates, one per line, with seven digits after the decimal point, including the initial `0.0000000` returned by the original Fortran `GRAND` initialization behaviour.

This is a **CLI wrapper**: the original Fortran function `GRAND(N)` accepts a dummy `N` and returns one `REAL` per call; the original Fortran source assumes that a separate `RAND(0)` uniform generator exists. To make reproducible same-input/same-output tests possible, **both** wrapper programs provide the same explicitly specified wrapping 32-bit linear congruential generator:

```
state = (1664525 * state + 1013904223) mod 2^32
u = floor(state / 256) / 2^24
```

The `u` values are exactly representable as IEEE-754 `f32`. This RNG is intended **only for reproducible demonstration**; it is not a high-quality modern random generator and is not cryptographically secure. It does not make successive normal variates truly independent.

The Rust implementation preserves the original 60-element lookup table and numerical comparisons, including the distinction between the original Fortran branches at labels `10`, `20`, `30`, and `40`. The state variable `u` is preserved between samples, and calculations use `f32` like the original Fortran `REAL`.

## Compile and run

### Fortran (gfortran)

```bash
gfortran -std=f2008 -O2 -ffp-contract=off fortran/grand_ref.f90 -o grand_fortran
./grand_fortran 12345 10
```

### Rust (cargo)

```bash
cargo run --release --offline -- 12345 10
```

Or compile without Cargo:

```bash
rustc -O --edition=2021 src/main.rs -o grand_rust
./grand_rust 12345 10
```

## Verify output equivalence

```bash
python3 tests/verify.py
```

The verifier builds both programs and compares 1,000 generated outputs for each of five seeds. It allows an absolute floating-point difference of `1e-6` if platform/compiler rounding yields slightly different final decimal digits. Both implementations should produce the same **numerical sequence**; exact text identity is not guaranteed for all platforms. The Fortran program was compiled and run in the preparation environment, and its results were matched by a separate Python float32 implementation on all 5,000 test samples. **The Rust binary was not compiled here because the Rust toolchain was unavailable**; run `tests/verify.py` with Cargo installed to complete the direct Rust-to-Fortran verification. This is an algorithmic equivalence test, **not** a statistical validation of normality.

## Android / Termux

With packages installed (`pkg install rust clang python` for Rust and test tools, plus a Fortran compiler if available):

```bash
rustc -O --edition=2021 src/main.rs -o grand_rust
./grand_rust 12345 10
```

No third-party Rust crates are required.
