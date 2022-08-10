SHELL := /bin/bash

.PHONY: run
run:
	cargo run -- -v docker-desktop context 

.PHONY: build
build:
	cargo build

.PHONY: deploy_local
deploy_local: build
	mkdir -p $$HOME/.kube/kubesess
	cp ./target/debug/kubesess ./kubesess.sh ~/.kube/kubesess/
	sudo mv ~/.kube/kubesess/kubesess /usr/local/bin/kubesess

.PHONY: benchmark
benchmark: deploy_local
	sh ./tests/benchmark.sh
	hyperfine --warmup 5 --runs 10 --shell none 'kubesess -v docker-desktop context' 'kubectx docker-desktop' --export-markdown ./tests/hyperfine/context-markdown-kubectx.md
	hyperfine --warmup 5 --runs 10 --shell none 'kubesess -v docker-desktop context' 'kubie ctx docker-desktop' --export-markdown ./tests/hyperfine/context-markdown-kubie.md

.PHONY: benchmark-ns
benchmark-ns:
	hyperfine --warmup 5 --runs 10 --shell none 'kubesess -v monitoring namespace' 'kubens monitoring' --export-markdown ./tests/hyperfine/namespace-markdown.md
