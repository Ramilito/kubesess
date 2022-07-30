SHELL := /bin/bash

.PHONY: deploy_local
deploy_local:
	cargo build && cp ./target/debug/kubesess ~/kubesess
