.PHONY: setup
setup:
	mkdir -p db_data
	docker-compose up -d
	docker pull izumin5210/ridgepole 
	docker run --net=kyokdy-api_default --env-file migration.env -v $(PWD):/workdir izumin5210/ridgepole -c $$DB_URL --apply -f Schemafile -o Schemafile

.PHONY: run
run:
	cargo run

.PHONY: db-dry-run
db-dry-run:
	docker run --net=kyokdy-api_default --env-file migration.env -v $(PWD):/workdir izumin5210/ridgepole -c $$DB_URL --apply --dry-run -f Schemafile -o Schemafile
