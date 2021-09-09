.PHONY: clean build install

run: clean build
	target/release/weather_CLI --city=Beijing

clean:
	cargo clean

build:
	cargo build --release

test:
	@target/release/weather_CLI --city=Qinhuangdao
