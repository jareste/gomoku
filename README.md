docker pull rustlang/rust:nightly

docker build -t my-rust-app .

docker run -it --rm --name my-running-app my-rust-app



FOR COMPILING TO WINDOWS::
REMOVE FROM CARGO.TOML THE DINAMIC LINKING ON THE FEATURES.
RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target=x86_64-pc-windows-gnu