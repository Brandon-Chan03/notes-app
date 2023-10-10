CREATE TABLE notes (
    id INTEGER PRIMARY KEY NOT NULL,
    note_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    content TEXT DEFAULT "" NOT NULL,
    created_at DATETIME DEFAULT (strftime('%Y-%m-%d %H:%M:%S', 'now', 'localtime')) NOT NULL,
    last_updated DATETIME DEFAULT (strftime('%Y-%m-%d %H:%M:%S', 'now', 'localtime')) NOT NULL
);