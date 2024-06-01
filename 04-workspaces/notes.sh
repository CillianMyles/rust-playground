# update /Cargo.toml
```
[workspace]

members = [
    "package-1", "package-2",
]
```

### add new package
cargo new 05-package --name package

### add new library
cargo new 06-library --name library --lib

### make package depend on library
[dependencies]
library = { path = "../06-library" }

### run package
cargo run -p package
