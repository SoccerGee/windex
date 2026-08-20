# Windex — build and packaging entry points.

.PHONY: help build app dmg install uninstall run clean

help:
	@echo "make build      Compile the release binary"
	@echo "make app        Build dist/Windex.app (universal)"
	@echo "make dmg        Build dist/Windex-<version>.dmg"
	@echo "make install    Build and install into /Applications"
	@echo "make uninstall  Remove the app and its login item"
	@echo "make run        Build and run in the foreground with debug logs"
	@echo "make clean      Remove build output"

build:
	cargo build --release --bin windex

app:
	./scripts/build-app.sh

dmg:
	./scripts/build-dmg.sh

install:
	./install.sh

uninstall:
	./install.sh uninstall

run: build
	RUST_LOG=debug ./target/release/windex

clean:
	cargo clean
	rm -rf dist
