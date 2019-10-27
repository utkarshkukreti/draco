default:
	cargo fmt -- --check
	erb README.md.erb > README.md
	cd examples && rake

deploy: default
	cd examples && netlifyctl deploy -P .

test: default
	cargo test
	wasm-pack test --firefox --headless
	cd tests && yarn test

.PHONY: default deploy test
