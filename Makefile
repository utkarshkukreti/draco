default:
	cargo fmt -- --check
	erb README.md.erb > README.md
	cd examples && rake

deploy: default
	cd examples && netlifyctl deploy -P .

test:
	cd tests && yarn test

.PHONY: default deploy
