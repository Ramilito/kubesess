SHELL := /bin/bash

LATEST_TAG := $(shell git describe --abbrev=0 --tags $(git rev-list --tags --max-count=1))

define bundle_release
	@echo ""
	if [[ "$(1)" == *"windows"* ]]; then  \
		tar -czvf ./target/$(1)/release/kubesess_$(1).tar.gz ./target/$(1)/release/kubesess.exe; \
	else \
		tar -czvf ./target/$(1)/release/kubesess_$(1).tar.gz ./target/$(1)/release/kubesess; \
	fi
endef

.PHONY: run
run:
	cargo run -- -v docker-desktop context

.PHONY: build
build:
	cargo build --release

.PHONY: clean
clean:
	rm -r -f $$HOME/.kube/kubesess
	sudo rm -r -f /usr/local/bin/kubesess

.PHONY: bundle_release
bundle_release:
	$(call bundle_release,${TARGET})

.PHONY: deploy_local
deploy_local: clean build
	mkdir -p $$HOME/.kube/kubesess
	sudo cp ./target/release/kubesess /usr/local/bin/kubesess
	@echo ""
	@echo "Installation complete. Add the following to your shell config:"
	@echo "  For bash/zsh: eval \"\$$(kubesess init bash)\""
	@echo "  For fish:     kubesess init fish | source"
	@echo "  For pwsh:     Invoke-Expression (&kubesess init powershell)"

.PHONY: benchmark
benchmark: deploy_local
	sh ./benches/benchmark.sh
	hyperfine --warmup 5 --runs 10 --shell none 'kubesess -v docker-desktop context' 'kubectx docker-desktop' --export-markdown ./benches/hyperfine/context-markdown-kubectx.md
	hyperfine --warmup 5 --runs 10 --shell none 'kubesess -v docker-desktop context' 'kubie ctx docker-desktop' --export-markdown ./benches/hyperfine/context-markdown-kubie.md

.PHONY: benchmark-ns
benchmark-ns: deploy_local
	sh ./benches/benchmark.sh
	hyperfine --warmup 5 --runs 10 --shell none 'kubesess -v monitoring namespace' 'kubens monitoring' --export-markdown ./benches/hyperfine/namespace-markdown.md


