# ビルド
build:
	docker compose build

# DB の立ち上げ
db: 
	docker compose up -d

# DB のコンソールへの接続
connect:
	docker compose exec database bash

# DB のマイグレーションとアプリケーションの起動
dev:
	sqlx db create
	sqlx migrate run
	RUST_LOG=debug cargo watch -x run

# テスト
test:
	cargo test

# スタンドアローンテスト
test-s:
	cargo test --no-default-features