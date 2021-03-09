SHELL := /bin/bash

ceres:
	cargo build --release

ui:
	yarn build

all: ceres ui

