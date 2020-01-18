# FxTypeMap

It's simple typemap based on crate [fxhash](https://crates.io/crates/fxhash).

## Usage

```rust
let mut map = TypeMap::new();

map.insert::<String>(String::from("hello"));

assert!(map.get::<String>().is_some());
assert!(map.contains::<String>());
```