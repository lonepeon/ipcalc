CARGO_BIN := cargo

.git/hooks/pre-commit: scripts/git-pre-commit.sh
	mkdir -p .git/hooks
	cp $< $@

.PHONY: setup
setup: .git/hooks/pre-commit
	asdf install

.PHONY: test-unit
test-unit:
	$(CARGO_BIN) test --verbose

.PHONY: test-style
test-style:
	$(CARGO_BIN) fmt --all -- --check

.PHONY: test-lint
test-lint:
	$(CARGO_BIN) clippy -- -D warnings
