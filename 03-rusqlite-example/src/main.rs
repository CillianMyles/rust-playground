use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("db.sqlite").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS languages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        )",
        [],
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO languages (name) VALUES (?1), (?2), (?3), (?4), (?5)",
        &["Python", "Java", "Kotlin", "Dart", "Rust"],
    )?;

    Ok(())
}
