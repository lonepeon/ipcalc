.git/hooks/pre-commit: scripts/git-pre-commit.sh
	mkdir --parents .git/hooks
	ln --force --symbolic $$(realpath $<) $@

.PHONY: setup
setup: .git/hooks/pre-commit
	rustup component add clippy
	cargo install cargo-watch

.PHONY: watch
watch:
	cargo watch --exec "fmt --all -- --check" --exec "clippy -- -Dwarnings" --exec test

.PHONY: test-unit
test-unit:
	cargo test

.PHONY: test-lint
test-lint:
	cargo fmt --all -- --check
	cargo clippy -- -Dwarnings
