debug: setup_dev
	cargo build
	cp target/debug/libmuzzman_module_logger.so liblogger.so

release: setup_dev
	cargo build --release
	cp target/release/libmuzzman_module_logger.so liblogger.so

install: release
	cp ./liblogger.so ~/.local/share/MuzzMan/modules/

clean:
	cargo clean

check:
	cargo check

setup_dev:

