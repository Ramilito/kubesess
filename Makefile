SHELL := /bin/bash

.PHONY: deploy_local
deploy_local:
	cargo build && cp ./target/debug/kubesess ~/kubesess/ && cp ./src/alias.sh ~/kubesess/

.PHONY: hyperfine
hyperfine: deploy_local
	hyperfine --warmup 5 --runs 100 --shell zsh 'source ~/kubesess/alias.sh; eval kc docker-desktop' 'kubectx docker-desktop' --export-markdown ./tests/hyperfine/markdown.md --export-json ./tests/hyperfine/timings.json
