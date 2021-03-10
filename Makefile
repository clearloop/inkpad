ceres:
	cargo build --release

ui:
	yarn build

all: ceres ui
