LANGS = en de en es fr pt ru

.PHONY: build
build:
	make cleanup
	cd ../rust/wasm && wasm-pack build --target bundler --out-dir ../../javascript/pkg
	./node_modules/.bin/tsc
	LANG=de ./node_modules/.bin/rollup -c rollup/config.js

cleanup:
	rm -rf ./build
	rm -rf ./pkg

test:
	./node_modules/.bin/jest tests

bench:
	node benches/search.js

bump:
	npm version $(V)
	cd ../rust/core && cargo bump $(V)
	cd ../rust/wasm && cargo bump $(V)
	cd ../examples/browser-plain    && sed -i '' 's/"lucid-suggest": "^*.*.*"/"lucid-suggest": "^$(V)"/' package.json
	cd ../examples/browser-react-ts && sed -i '' 's/"lucid-suggest": "^*.*.*"/"lucid-suggest": "^$(V)"/' package.json
	cd ../examples/browser-vue      && sed -i '' 's/"lucid-suggest": "^*.*.*"/"lucid-suggest": "^$(V)"/' package.json
	cd ../examples/node-ts          && sed -i '' 's/"lucid-suggest": "^*.*.*"/"lucid-suggest": "^$(V)"/' package.json
