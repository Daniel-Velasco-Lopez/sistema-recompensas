default: build

all: test

test: build
	cargo test

build:
	stellar contract build
	@echo "🧩 WASM generado:"
	@ls -lh target/wasm32v1-none/release/*.wasm

fmt:
	cargo fmt --all

clean:
	cargo clean

info:
	@echo "📦 Contract: sistema-recompensas"
	@echo "📁 WASM Directory: target/wasm32v1-none/release/"
