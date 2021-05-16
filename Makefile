.PHONY: client server

client: client/src/lib.rs
	cd client ;\
	wasm-pack build --target web --out-name wasm --out-dir ../static ;\
	cd ..

server: server/src/main.rs
	cd server ;\
	cargo build ;\
	cd ..

run: client server
	cargo run -p server

clean:
	rm ./static/wasm*.*
	cd server ;\
	cargo clean ;\
	cd ..
