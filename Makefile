examples:
	cd examples && rake

deploy:
	make examples
	cd examples && netlifyctl deploy -P .

.PHONY: examples deploy
