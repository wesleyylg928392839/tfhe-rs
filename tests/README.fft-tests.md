# FFT128 Test Notes

This note is intended for contributors working on the `fft128` test branch.

## Goals

- Keep test names stable across refactors.
- Prefer descriptive test cases:
  - input size
  - expected property (e.g., "roundtrip", "overflow_guard", "edge_zero")

## Conventions

- Test name format:
  - `fft128_<case>_<expected>`
- Avoid embedding randomness without a fixed seed.
- If a test is slow, mark it clearly and explain why.

## Example

- `fft128_roundtrip_matches_reference`
- `fft128_edge_zero_is_identity`
