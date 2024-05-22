# float-dbg

> Debug floating point numbers easily in Rust.

[![crates.io](https://img.shields.io/crates/v/float-dbg?logo=rust)](https://crates.io/crates/float-dbg)
[![docs.rs](https://img.shields.io/docsrs/float-dbg?logo=docsdotrs)](https://docs.rs/float-dbg)
[![GitHub](https://img.shields.io/static/v1?label=github&message=FedericoStra/float-dbg&color=brightgreen&logo=github)](https://github.com/FedericoStra/float-dbg)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/FedericoStra/float-dbg/rust.yml?logo=githubactions&logoColor=white)](https://github.com/FedericoStra/float-dbg/actions/workflows/rust.yml)
[![Dependencies status](https://deps.rs/repo/github/FedericoStra/float-dbg/status.svg)](https://deps.rs/repo/github/FedericoStra/float-dbg)
[![MIT license](https://img.shields.io/crates/l/float-dbg)](https://choosealicense.com/licenses/mit/)

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
