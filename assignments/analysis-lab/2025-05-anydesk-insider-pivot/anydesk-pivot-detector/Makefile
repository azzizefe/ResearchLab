# AnyDesk Pivot Detector - Makefile

.PHONY: dev test lint fmt coverage build docker clean audit e2e

dev:
	cargo tauri dev

test:
	cargo nextest run

lint:
	cargo clippy -- -D warnings

fmt:
	cargo fmt

coverage:
	cargo tarpaulin --out Html

build:
	cargo tauri build

docker:
	docker-compose up --build

clean:
	cargo clean
	cd frontend && rm -rf node_modules dist

audit:
	cargo audit
	cargo deny check

e2e:
	cargo nextest run --test-threads=1 -E 'test(e2e)'
