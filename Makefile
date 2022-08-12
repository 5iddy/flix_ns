.PHONY: docs docs-clean install-deps unit-test cleanup build

docs:
	cargo doc -r --no-deps -p flix_ns
	cp -r target/doc docs/
	echo "<meta http-equiv=\"refresh\" content=\"0; url=flix_ns\">" > docs/index.html

watch-docs:
	cargo watch -qw -x "doc -r --no-deps -p flix_ns"

docs-clean:
	rm -r docs/

unit-test:
	cargo test --profile release -p flix_ns

build:
	RUSTFLAGS="-C link-args=-s" cargo wasm-build

cleanup:
	rm -rf target/

