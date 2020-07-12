LANGS = en de en es fr pt ru

build:
	make cleanup
	make compile

cleanup:
	rm -rf ./*.js
	rm -rf ./*.d.ts

compile:
	for LANG in ${LANGS}; do \
		cd ../rust/wasm ; \
		RUSTFLAGS="--cfg lang=\"$$LANG\"" wasm-pack build --target bundler --out-dir ../../javascript/pkg; \
		cd ../../javascript ; \
		RUSTFLAGS="--cfg lang=\"$$LANG\"" ./node_modules/.bin/rollup -c rollup/config.js; \
		cp src/index.d.ts ./$$LANG.d.ts; \
	done;

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
	cd ../demo                      && sed -i '' 's/"lucid-suggest": "^*.*.*"/"lucid-suggest": "^$(V)"/' package.json
