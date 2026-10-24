.PHONY: help help-all build release test check install outdated

.DEFAULT_GOAL := help

help:
	@echo "Usage: make [target]"
	@echo ""
	@echo "  check    - Full verification (build + test)"
	@echo "  build    - Build debug binary"
	@echo "  release  - Build release binary"
	@echo "  test     - Run all tests"
	@echo "  install  - Build release and install to ~/.local/bin"
	@echo ""
	@echo "Run 'make help-all' for the full list, including internal targets."

help-all:
	@echo "Public targets:"
	@echo "  check    - Full verification (build + test)"
	@echo "  build    - Build debug binary"
	@echo "  release  - Build release binary"
	@echo "  test     - Run all tests"
	@echo "  install  - Build release and install to ~/.local/bin"
	@echo ""
	@echo "Internal targets:"
	@echo "  outdated - List outdated dependencies"

build:
	cargo build

release:
	cargo build --release

test:
	cargo test

check: build test
	@echo "ALL CHECKS PASSED"

install:
	bin/install/install-from-source.sh

outdated:
	bin/support/find-outdated-dependencies.sh
