.PHONY: help help-all build release test check install outdated

.DEFAULT_GOAL := help

help: ## show help (public targets only)
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		grep -E "^(check|build|release|test|install):" | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  %-12s %s\n", $$1, $$2}'
	@echo ""
	@echo "Run 'make help-all' for the full list, including internal targets."

help-all: ## show help (all targets)
	@echo "Public targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		grep -E "^(check|build|release|test|install):" | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  %-12s %s\n", $$1, $$2}'
	@echo ""
	@echo "Internal targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		grep -vE "^(help|help-all|check|build|release|test|install):" | \
		grep -E '^[a-zA-Z_-]+:.*?## .*$$' | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  %-12s %s\n", $$1, $$2}'

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
