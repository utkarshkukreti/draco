default:
	erb README.md.erb > README.md
	cd examples && rake

deploy: default
	cd examples && netlifyctl deploy -P .

.PHONY: default deploy
