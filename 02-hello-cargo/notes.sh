# create cargo project
cd ../
cargo new --name hello-cargo 02-hello-cargo
cd 02-hello-cargo

# debug

# compile
cargo build

# run built binary
./target/debug/hello-cargo

# build (if needed) + run
cargo run


# release

# compile
cargo build --release

# run built binary
./target/release/hello-cargo

# build (if needed) + run
cargo run --release
