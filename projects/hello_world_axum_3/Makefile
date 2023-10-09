build:
	docker compose build

db: 
	docker compose up -d

connect:
	docker compose exec database bash

connect-test:
	docker compose exec database-test bash

include .env
dev:
	sqlx db create
	sqlx migrate run --database-url $(DATABASE_URL)
	sqlx migrate run --database-url $(DATABASE_URL_TEST)
	cargo watch -x run

test:
	cargo test
