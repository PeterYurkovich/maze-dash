use rusqlite::Connection;
use rusqlite_migration::{M, Migrations, Result};

pub fn migrate() -> Result<()> {
    // 1️⃣ Define migrations
    let migrations = Migrations::new(vec![M::up(
        "CREATE TABLE IF NOT EXISTS maze (
        key   TEXT PRIMARY KEY,
        maze  BLOB NOT NULL
    );",
    )]);

    let mut connection = Connection::open("./sqlite.db3")?;
    // 2️⃣ Update the database schema, atomically
    migrations.to_latest(&mut connection)?;

    Ok(())
}
