docker pull rustlang/rust:nightly

docker build -t my-rust-app .

docker run -it --rm --name my-running-app my-rust-app