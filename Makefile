test-watch:
	nodemon -e "*.toml *.rs"  -x "cargo check && cargo clippy && cargo test || true"

clean:
	cargo clean

clear:
	make clean
