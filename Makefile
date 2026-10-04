.PHONY: help build release test check install outdated

.DEFAULT_GOAL := help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  %-12s %s\n", $$1, $$2}'

build: ## build debug binary
	cargo build

release: ## build release binary
	cargo build --release

test: ## run all tests
	cargo test

check: build test ## full verification (build + test)
	@echo "ALL CHECKS PASSED"

install: ## build release and install to ~/.local/bin
	bin/install/install-from-source.sh

outdated: ## list outdated dependencies
	bin/support/find-outdated-dependencies.sh
