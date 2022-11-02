main:
	cargo bootimage

fixbuild:
	rustup update
	rustup toolchain install nightly
	rustup default nightly
	rustup component add rust-src
	rustup component add llvm-tools-preview

clean:
	cargo clean