main:
	cargo bootimage

fixbuild:
	echo 如果运行失败请检查rust的安装
	rustup update
	rustup toolchain install nightly
	rustup default nightly
	rustup component add rust-src
	rustup component add llvm-tools-preview
	cargo add xbuild
	cargo add bootimage
	echo 运行完后记得把Cargo.toml文件中dependencies下的xbuild和bootimage删掉，否则会编译错误

clean:
	cargo clean