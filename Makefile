SHELL := /bin/bash

.PHONY: deploy_build
deploy_build:
	cargo build

.PHONY: deploy_local
deploy_local: deploy_build
	cp ./target/debug/kubesess ./src/kubesess.sh ~/kubesess/
	sudo mv ~/kubesess/kubesess /usr/local/bin/kubesess

.PHONY: hyperfine
hyperfine: deploy_local
	hyperfine --warmup 5 --runs 100 --shell zsh 'source ~/kubesess/kubesess.sh; eval kc -v docker-desktop context' 'kubectx docker-desktop' --export-markdown ./tests/hyperfine/markdown.md --export-json ./tests/hyperfine/timings.json
