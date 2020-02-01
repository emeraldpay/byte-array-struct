Provides a macro to allows creation of a simple byte-array backed structs. Such struct has a predefined size and
allocated on stack.

## Usage

### Dependency

```
[dependencies]
byte-array-struct = "0.1"
```

### Example

```
// create struct named Address backed by [u8; 24]
// basically a shortcut to `pub struct Address([u8; 24]);`
byte_array_struct!(
    pub struct Address(24);
);

impl Address {
    // any additional functionality for Address type
}

// passed as a value on stack
fn send(to: Address) {
   // ...
}

fn main() {
  //accepts hex, which can also be prefixed with 0x
  let foo = Address::from_str("0123456789abcdef0123456789abcdef0123456789abcdef").unwrap();

  send(foo);
}
```

### Features

Macro provides implementation for following traits:

- `.deref()`
- `.from_str(s)`, which accepts a hex string with the length of target array; may be optionally prefixed with `0x`
- `.to_string()`
- `.from([u8; ...])` and `.from(&[u8; ...])`, where `...` is the defined size
- `.try_from(Vec<u8>)` and `.try_from(&[u8])`
- `.into(Vec<u8>)` and `.into([u8; ...])`
- `.serialize` and `.deserialize` for Serde, with `serialize` feature enabled (default)