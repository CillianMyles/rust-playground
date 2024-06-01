use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("db.sqlite").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS languages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            ease_of_use REAL NOT NULL,
            performance REAL NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO languages (name, ease_of_use, performance) VALUES (?1, ?2, ?3), (?4, ?5, ?6), (?7, ?8, ?9), (?10, ?11, ?12), (?13, ?14, ?15)",
        ["Python", "78.0", "33.0", "Java", "59.0", "51.0", "Kotlin", "63.0", "49.0", "Dart", "81.0", "78.0", "Rust", "47.0", "93.0"],
    )?;

    let mut stmt = conn.prepare(
        "SELECT name, ease_of_use, performance, (ease_of_use * performance) as score
        FROM languages
        ORDER BY score DESC",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, f64>(1)?,
            row.get::<_, f64>(2)?,
            row.get::<_, f64>(3)?,
        ))
    })?;

    for row in rows {
        let (name, ease_of_use, performance, score) = row.unwrap();
        println!(
            "{name} ( ease_of_use: {ease_of_use}, performance: {performance}, score: {score} )",
        );
    }

    Ok(())
}
