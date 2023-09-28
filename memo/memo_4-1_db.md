# データベースの準備

以下のファイルを追加：

- `Dockerfile`

  ```Dockerfile
  FROM postgres:13-alpine AS database
  ENV LANG ja_JP.utf8
  ```

- `compose.yml`
  
  ```yml
  version: "3.8"
  services:
    database:
      build: 
        context: .
        dockerfile: Dockerfile
        target: 'database'
      ports:
        - "5432:5432"
      volumes:
        - pgdata:/var/llib/postgresql/data
      environment:
        - POSTGRES_PASSWORD=admin
        - POSTGRES_USER=admin
        - POSTGRES_DB=todos
        - TZ=Asia/Tokyo
      restart: always
  volumes:
    pgdata:
  ```

- `Makefile`

  ```Makefile
  build:
      docker compose build

  db: 
      docker compose up -d

  connect:
      docker compose exec database bash

  dev:
      cargo watch -x run

  test:
      cargo test
  ```
