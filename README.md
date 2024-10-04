# swift-demangle

A Swift Demangle function in Rust

## Installation

```shell
cargo add swift-demangle
```

## API usage

```rust
use swift_demangle::demangle;

demangle("$sSa"); // returns "Swift.Array"
```

## CLI usage

> [!IMPORTANT]  
> It's important to escape `$` in your shell interface.

```shell
cargo run "_\$s7SwiftUI12__GridLayoutVAA014_VariadicView_F4RootAAWP"
# protocol witness table for SwiftUI.__GridLayout : SwiftUI._VariadicView_ViewRoot in SwiftUI
# Swift.Array
```

## Licence

[MIT](/LICENSE)

## Copyright

© 2024, Eugene Hauptmann