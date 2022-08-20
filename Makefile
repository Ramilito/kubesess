SHELL := /bin/bash

LATEST_TAG := $(shell git describe --abbrev=0 --tags $(git rev-list --tags --max-count=1))

define bundle_release
	@echo ""
	if [[ "$(1)" == *"windows"* ]]; then  \
		tar -czvf ./target/$(1)/release/kubesess_$(1).tar.gz scripts/ ./target/$(1)/release/kubesess.exe; \
	else \
		tar -czvf ./target/$(1)/release/kubesess_$(1).tar.gz scripts/ ./target/$(1)/release/kubesess; \
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
	cp ./target/release/kubesess ./scripts/sh/kubesess.sh ~/.kube/kubesess/
	sudo mv ~/.kube/kubesess/kubesess /usr/local/bin/kubesess

.PHONY: benchmark
benchmark: deploy_local
	sh ./tests/benchmark.sh
	hyperfine --warmup 5 --runs 10 --shell none 'kubesess -v docker-desktop context' 'kubectx docker-desktop' --export-markdown ./tests/hyperfine/context-markdown-kubectx.md
	hyperfine --warmup 5 --runs 10 --shell none 'kubesess -v docker-desktop context' 'kubie ctx docker-desktop' --export-markdown ./tests/hyperfine/context-markdown-kubie.md

.PHONY: benchmark-ns
benchmark-ns:
	hyperfine --warmup 5 --runs 10 --shell none 'kubesess -v monitoring namespace' 'kubens monitoring' --export-markdown ./tests/hyperfine/namespace-markdown.md


