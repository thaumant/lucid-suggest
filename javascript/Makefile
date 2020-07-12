LANGS = english german spanish french portuguese russian

.PHONY: build
.PHONY: wasm


build:
	make cleanup
	make wasm
	make tsc


tsc:
	./node_modules/.bin/tsc


wasm:
	cd ../rust/wasm && wasm-pack build --target bundler --out-dir ../../javascript/tmp
	./node_modules/.bin/rollup -c rollup/config-wasm.js
	rm -rf ./tmp


cleanup:
	rm -rf ./index.{js,d.ts}
	rm -rf ./utils.{js,d.ts}
	rm -rf ./wasm.{js,d.ts}
	rm -rf ./lang
	rm -rf ./tokenization


test:
	./node_modules/.bin/jest tests


bench:
	node benches/search.js


sizes:
	rm -rf ./tmp/
	./node_modules/.bin/tsc --module es2020

	for lang in ${LANGS}; do \
		LANG=$$lang ./node_modules/.bin/rollup -c rollup/config-sizes.js; \
		./node_modules/.bin/uglifyjs tmp/$$lang.js -o tmp/$$lang.min.js; \
		tar -cvzf tmp/$$lang.tar.gz tmp/$$lang.min.js; \
	done

	rm tmp/*.min.js
	ls -lAh tmp


bump:
	npm version $(V)
	cd ../rust/core && cargo bump $(V)
	cd ../rust/wasm && cargo bump $(V)
	cd ../examples/browser-plain    && sed -i '' 's/"lucid-suggest": "^*.*.*"/"lucid-suggest": "^$(V)"/' package.json
	cd ../examples/browser-react-ts && sed -i '' 's/"lucid-suggest": "^*.*.*"/"lucid-suggest": "^$(V)"/' package.json
	cd ../examples/browser-vue      && sed -i '' 's/"lucid-suggest": "^*.*.*"/"lucid-suggest": "^$(V)"/' package.json
	cd ../examples/node-ts          && sed -i '' 's/"lucid-suggest": "^*.*.*"/"lucid-suggest": "^$(V)"/' package.json
