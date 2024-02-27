all:
	@CARGO_TARGET_DIR=/tmp/gomoku/ cargo run
# @CARGO_TARGET_DIR=/tmp/gomoku/ cargo build --release

run:
	/goinfre/gomoku/release/gomoku

.PHONY: all