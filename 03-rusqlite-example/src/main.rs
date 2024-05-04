use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("db.sqlite").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS languages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            compiled BOOLEAN NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS stats (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            language_id INTEGER NOT NULL,
            ease_of_use REAL NOT NULL,
            performance REAL NOT NULL,
            FOREIGN KEY (language_id) REFERENCES languages(id)
        )",
        [],
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO languages (name) VALUES (?1), (?2), (?3), (?4), (?5)",
        ["Python", "Java", "Kotlin", "Dart", "Rust"],
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO stats (language_id, ease_of_use, performance) VALUES
            ((SELECT id FROM languages WHERE name = 'Python'), 78.0, 33.0),
            ((SELECT id FROM languages WHERE name = 'Java'), 59.0, 51.0),
            ((SELECT id FROM languages WHERE name = 'Kotlin'), 63.0, 49.0),
            ((SELECT id FROM languages WHERE name = 'Dart'), 81.0, 78.0),
            ((SELECT id FROM languages WHERE name = 'Rust'), 47.0, 93.0)",
        [],
    )?;

    Ok(())
}
