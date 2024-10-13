use rusqlite::{Connection, Error};

pub fn db_connection() -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open("db.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            password TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    Ok(conn)
}

pub fn open_users_db() -> Result<Connection, Error> {
    let conn = Connection::open("db.db")?;
    Ok(conn)
}
