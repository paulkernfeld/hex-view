# hex-view

## `HexView`

Easily format a `&[u8]` as hex.

```rust
use hex_view::HexView;

let ff00_slice: &[u8] = &[255, 0];
assert_eq!(format!("{:x}", HexView::from(&ff00_slice)), "ff00")

```

`HexView::from` also works on anything that implements `AsRef<[u8]>`, such as a `&Vec<u8>`:

```rust
use hex_view::HexView;

let ff00_vec: Vec<u8> = vec![255, 0];
assert_eq!(format!("{:X}", HexView::from(&ff00_vec)), "FF00")
```

License: MIT/Apache-2.0
