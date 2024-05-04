use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("db.sqlite").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS languages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
}
