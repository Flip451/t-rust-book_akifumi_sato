-- Todo テーブルの作成
CREATE TABLE todos
(
    id          UUID PRIMARY KEY,
    text        TEXT    NOT NULL,
    completed   BOOLEAN NOT NULL DEFAULT false
);

-- User テーブルの作成
CREATE TABLE users
(
    id          UUID PRIMARY KEY,
    name        TEXT    NOT NULL
);