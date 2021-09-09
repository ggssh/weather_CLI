.PHONY: clean build-d install

run: clean build-d
	target/debug/weather_CLI --city=Beijing

clean:
	cargo clean

build-d:
	cargo build 

test:
	@target/release/weather_CLI --city=Qinhuangdao
