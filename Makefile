.PHONY: setup
setup:
	mkdir db_data

.PHONY: run
run:
	docker-compose up -d
	cargo run
