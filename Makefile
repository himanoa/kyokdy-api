.PHONY: setup
setup:
	mkdir -p db_data
	docker-compose up -d
	docker pull izumin5210/ridgepole 
	docker run --net=kyokdy-api_default --env-file migration.env izumin5210/ridgepole --apply

.PHONY: run
run:
	cargo run
