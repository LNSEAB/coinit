# coinit

CoInitializeEx wrapper

```rust
fn main() {
    let _coinit = coinit::init(coinit::MULTITHREADED | coinit::DISABLE_OLE1DDE).unwrap();
    // `_coinit` is dropped, call `CoUninitialize`
}
```

Licensed under [Zlib license](LICENSE)

Copyright (C) 2022 LNSEAB