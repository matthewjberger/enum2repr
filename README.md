# enum2repr

[<img alt="github" src="https://img.shields.io/badge/github-matthewjberger/enum2repr-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/matthewjberger/enum2repr)
[<img alt="crates.io" src="https://img.shields.io/crates/v/enum2repr.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/enum2repr)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-enum2repr-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/enum2repr)

enum2repr is a rust derive macro that creates conversion methods to map between a value and an enum. 
Numeric types supported by `#[repr(T)]` are supported by enum2repr.

## Usage

Add this to your `Cargo.toml`:

```toml
enum2repr = "0.1.14"
```

Example:

```rust
use enum2repr::EnumRepr;

#[derive(EnumRepr, Debug, PartialEq, Copy, Clone)]
#[repr(u16)]
enum Color {
    Red = 0x04,
    Green = 0x15,
    Blue = 0x34,
}

#[test]
fn convert_variants() {
    assert_eq!(Ok(Color::Red), Color::try_from(0x04));
    assert_eq!(Ok(Color::Green), Color::try_from(0x15));
    assert_eq!(Ok(Color::Blue), Color::try_from(0x34));
}

#[test]
fn convert_variants_back() {
    assert_eq!(u16::from(Color::Red), 0x04);
    assert_eq!(u16::from(Color::Green), 0x15);
    assert_eq!(u16::from(Color::Blue), 0x34);
}
```
