TARGET := wasm32-unknown-unknown

.PHONY: build
build:
	cargo build --target $(TARGET)

.PHONY: release
release:
	cargo build --release --target $(TARGET)

.PHONY: add-target
add-target:
	rustup target add $(TARGET)
