-- Todo テーブルの作成
CREATE TABLE todos
(
    id          SERIAL PRIMARY kEY,
    text        TEXT    NOT NULL,
    completed   BOOLEAN NOT NULL DEFAULT false
);