.PHONY: setup
include migration.env
export
setup:
	mkdir -p db_data
	rm -f docker-compose.env
	touch docker-compose.env
	$(eval UID := $(shell id -u $(USER)))
	$(eval GID := $(shell id -g $(USER)))
	echo "UID=$(UID)" >> docker-compose.env
	echo "GID=$(GID)" >> docker-compose.env
	echo "UNAME=$(USER)" >> docker-compose.env
	docker pull izumin5210/ridgepole 
	docker run --net=kyokdy-api_default  -v $(PWD):/workdir izumin5210/ridgepole -c $(DB_URL) --apply -f Schemafile -o Schemafile 
	docker run --net=kyokdy-api_default  -v $(PWD):/workdir izumin5210/ridgepole -c $(TESTING_DB_URL) --apply -f Schemafile -o Schemafile

.PHONY: run
run:
	RUST_LOG=debug cargo run --bin server

.PHONY: db-dry-run
db-dry-run:
	docker run --net=kyokdy-api_default -v $(PWD):/workdir izumin5210/ridgepole -c $(DB_URL) --apply --dry-run -f Schemafile -o Schemafile

.PHONY: db-migrate
db-migrate:
	docker run --net=kyokdy-api_default  -v $(PWD):/workdir izumin5210/ridgepole -c $(DB_URL) --apply -f Schemafile -o Schemafile 
	docker run --net=kyokdy-api_default  -v $(PWD):/workdir izumin5210/ridgepole -c $(TESTING_DB_URL) --apply -f Schemafile -o Schemafile

.PHONY: unit-test
unit-test:
	cargo test

.PHONY: integration-test
integration-test:
	cargo test --features integration_test

.PHONY: fmt
fmt:
	cargo fmt
