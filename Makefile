all: frontend backend

backend: src/*
	cargo build --features backend


frontend: src/*
	wasm-pack build -t browser -- --features frontend


.PHONY: backend frontend
