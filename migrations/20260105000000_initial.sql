CREATE TABLE IF NOT EXISTS Requests (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    reward TEXT,
    client_id TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS Labels (
    request_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    PRIMARY KEY (request_id, key),
    FOREIGN KEY (request_id) REFERENCES Requests(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS Tags (
    request_id TEXT NOT NULL,
    tag TEXT NOT NULL,
    PRIMARY KEY (request_id, tag),
    FOREIGN KEY (request_id) REFERENCES Requests(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_labels_key ON Labels(key);
CREATE INDEX IF NOT EXISTS idx_labels_key_value ON Labels(key, value);
CREATE INDEX IF NOT EXISTS idx_tags_tag ON Tags(tag);