LANGS = de en es pt ru

build:
	make cleanup
	make compile

cleanup:
	rm -rf dist
	rm -rf pkg

compile:
	for LANG in ${LANGS}; do \
		cd ../rust/wasm ; \
		RUSTFLAGS="--cfg lang=\"$$LANG\"" wasm-pack build --target bundler --out-dir ../../javascript/pkg; \
		cd ../../javascript ; \
		RUSTFLAGS="--cfg lang=\"$$LANG\"" ./node_modules/.bin/rollup -c rollup/config-cjs.js; \
	done;
