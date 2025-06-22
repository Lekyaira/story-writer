use rusqlite::{Connection, Result};

pub struct Database {
    pub conn: Connection,
}

impl Database {
    /// Open a new SQLite database connection at the given path.
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Self { conn })
    }
} 