float-dbg
=========

Easily debug floating point numbers in Rust.

```rust
0.045_f32.explain()
```

```
value = 0.045
bits: 00111101001110000101000111101100
      ±^^^^^^^^_______________________
sign: +
exponent = 122 - 127 = -5
significand = 2^23 + 3690988 = 12079596
```
