# access-derive

A set of derive macros to automatically generate getters/setters. This is a educational project and is not optimized in any way (for now at least)

<br/>

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
access-derive = "^0.1"
```

<br/>

## Details

`access-derive` comes with a set of field attributes that generates automatically getters and setters.

Consider a simple structure as follow:

```rust
pub struct MyStruct {
    foo: String,
}
```

Attributes edded to the `foo` field will have following effects:
- `#[access(get)]` will produce a method `foo(&self) -> &String { ... }`
- `#[access(get(ref = '&str'))]` will produce a method `foo(&self) -> &str { ... }`
- `#[access(get(ref_mut)]` will produce a method `foo(&self) -> &mut String { ... }`
- `#[access(set)]` will produce a method `set_foo(&mut self, String) { ... }`

<br/>

## Improvements
If I have the time/will, here's a list of things that I am not happy about
- [ ] Errors reported are a bit wierd, due to spans when generating methods
- [ ] `set` attribute should take an argument to specify a custom type
- [ ] `set` attribute should take a `ref`
