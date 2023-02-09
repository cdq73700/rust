.PHONY: backend mysql phpmyadmin

build:
	docker compose build
up:
	docker compose up -d
stop:
	docker compose stop
ps:
	docker compose ps
ci:
	docker compose run --rm rust-backend bash -c "cargo install --path ."
	docker compose run --rm rust-backend bash -c "cargo install diesel_cli"
	docker compose run --rm rust-backend bash -c "diesel setup"
cb:
	docker compose run --rm rust-backend bash -c "cargo build"
cbr:
	docker compose run --rm rust-backend bash -c "cargo build --release"
backend:
	docker compose exec rust-backend bash
mysql:
	docker compose exec rust-mysql bash -c "mysql -u test -p"
phpmyadmin:
	docker compose exec rust-phpmyadmin bash
migration:
	docker compose run --rm rust-backend bash -c "diesel migration run"
	docker compose run --rm rust-backend bash -c "cargo run --bin migrations"
migration-redo:
	docker compose run --rm rust-backend bash -c "diesel migration redo --all"
	docker compose run --rm rust-backend bash -c "cargo run --bin migrations"
# make create_table-test
create_table-%:
	docker compose run --rm rust-backend bash -c 'diesel migration generate create_"${@:create_table-%=%}"'
fossa:
	docker compose run --rm rust-backend bash -c 'fossa analyze'