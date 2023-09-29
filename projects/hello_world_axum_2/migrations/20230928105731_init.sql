-- Todo テーブルの作成
CREATE TABLE todos
(
    id          UUID PRIMARY KEY,
    text        TEXT    NOT NULL,
    completed   BOOLEAN NOT NULL DEFAULT false
);