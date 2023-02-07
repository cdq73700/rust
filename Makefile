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
	docker compose run rust-backend bash -c "cargo install --path ."
cb:
	docker compose run rust-backend bash -c "cargo build"
cbr:
	docker compose run rust-backend bash -c "cargo build --release"
backend:
	docker compose exec rust-backend bash
mysql:
	docker compose exec rust-mysql bash
phpmyadmin:
	docker compose exec rust-phpmyadmin bash