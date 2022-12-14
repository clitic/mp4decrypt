<h1 align="center">mp4decrypt</h1>

<p align="center">
  <a href="https://crates.io/crates/mp4decrypt">
    <img src="https://img.shields.io/crates/d/mp4decrypt?style=flat-square">
  </a>
  <a href="https://crates.io/crates/mp4decrypt">
    <img src="https://img.shields.io/crates/v/mp4decrypt?style=flat-square">
  </a>
  <a href="https://github.com/clitic/mp4decrypt">
    <img src="https://img.shields.io/github/actions/workflow/status/clitic/mp4decrypt/tests.yml?logo=github&style=flat-square">
  </a>
  <a href="https://docs.rs/mp4decrypt/latest/mp4decrypt">
    <img src="https://img.shields.io/docsrs/mp4decrypt?logo=docsdotrs&style=flat-square">
  </a>
  <a href="https://github.com/clitic/mp4decrypt#license">
    <img src="https://img.shields.io/crates/l/mp4decrypt?style=flat-square">
  </a>
  <a href="https://github.com/clitic/mp4decrypt">
    <img src="https://img.shields.io/github/repo-size/clitic/mp4decrypt?style=flat-square">
  </a>
  <a href="https://github.com/clitic/mp4decrypt">
    <img src="https://img.shields.io/tokei/lines/github/clitic/mp4decrypt?logo=github&style=flat-square">
  </a>
</p>

This library provides a safe function to decrypt encrypted mp4 data stream using [Bento4](https://github.com/axiomatic-systems/Bento4). Also, some basic mp4 splitting can be done.

## Getting Started

Add this to your Cargo.toml file.

```toml
[dependencies]
mp4decrypt = "0.3.1"
```

Or add from command line.

```bash
$ cargo add mp4decrypt
```

See [docs](https://docs.rs/mp4decrypt/latest/mp4decrypt) and [examples](https://github.com/clitic/mp4decrypt/tree/main/examples) to 
know how to use it.

## Example

```rust
use std::collections::HashMap;
use std::io::Write;

fn main() {
    let mut input = include_bytes!("init.mp4").to_vec();
    input.extend(include_bytes!("segment_0.m4s"));

    let mut keys = HashMap::new();
    keys.insert(
        "eb676abbcb345e96bbcf616630f1a3da".to_owned(),
        "100b6c20940f779a4589152b57d2dacb".to_owned(),
    );

    let decrypted_data = mp4decrypt::mp4decrypt(&input, keys, None).unwrap();

    std::fs::File::create("decrypted.mp4")
        .unwrap()
        .write_all(&decrypted_data)
        .unwrap();
}
```

## License

Dual Licensed

- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([LICENSE-APACHE](LICENSE-APACHE))
- [MIT license](https://opensource.org/licenses/MIT) ([LICENSE-MIT](LICENSE-MIT))
