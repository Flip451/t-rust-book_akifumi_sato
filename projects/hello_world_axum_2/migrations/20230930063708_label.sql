-- labels テーブルを追加
CREATE TABLE labels
(
    id          UUID PRIMARY KEY,
    name        TEXT    NOT NULL  -- TODO: UNIQUE 制約を付ける？
);

-- todos テーブルと labels テーブルを多対多対応させるためのテーブルを作成
CREATE TABLE todo_labels
(
    todo_id     UUID    NOT NULL,
    FOREIGN KEY (todo_id) REFERENCES todos(id) ON DELETE CASCADE DEFERRABLE INITIALLY DEFERRED,
    label_id    UUID    NOT NULL,
    FOREIGN KEY (label_id) REFERENCES labels(id) ON DELETE CASCADE DEFERRABLE INITIALLY DEFERRED,
    PRIMARY KEY (todo_id, label_id)
);