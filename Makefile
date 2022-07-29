SHELL := /bin/bash

.PHONY: build
build:
	cargo build && cp ./target/debug/kubesess .
