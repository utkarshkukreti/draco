default: README.md examples

README.md: README.md.erb examples
	erb $< > $@

examples:
	cd examples && rake

deploy:
	make examples
	cd examples && netlifyctl deploy -P .

.PHONY: default examples deploy
