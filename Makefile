.PHONY: client server

client:
	cd client ;\
	wasm-pack build --target web --out-name wasm --out-dir ../static ;\
	cd ..

server:
	cd server ;\
	cargo build ;\
	cd ..

all: client server

run: client server
	cargo run -p server

clean:
	rm ./static/js/client.js
	cargo clean
