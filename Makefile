all: frontend backend

backend: src/*
	cargo build --features backend


frontend: src/*
	wasm-pack build -t web -- --features frontend


.PHONY: backend frontend
