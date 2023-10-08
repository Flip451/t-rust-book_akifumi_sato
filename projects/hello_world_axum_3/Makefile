build:
	docker compose build

db: 
	docker compose up -d

connect:
	docker compose exec database bash

dev:
	sqlx db create
	sqlx migrate run
	cargo watch -x run

test:
	cargo test