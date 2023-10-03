update-release:
	cargo build --release
	rm -rf ~/bin/poldot
	cp target/release/poldot ~/bin/poldot