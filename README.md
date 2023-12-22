# :sob: sob

[![Crates.io](https://img.shields.io/crates/v/sob.svg)](https://crates.io/crates/sob)
[![Documentation](https://docs.rs/sob/badge.svg)](https://docs.rs/sob)
![License](https://img.shields.io/crates/l/sob.svg)

*Serializable Owned/Borrowed*

`Sob` is a type similar to `Cow` but without the `Clone` trait requirement for the inner type.

In practice, `Sob` should only be used when you [need to serialize and deserialize a `Cow`](https://stackoverflow.com/a/52733564) and the inner type does not implement `Clone`.