use rusqlite::{Connection, Result};
use std::path::Path;

pub fn initialize(path: &Path) -> Result<Connection> {
    let conn = Connection::open(path)?;

    conn.execute_batch(
        "
        PRAGMA foreign_keys = ON;

        CREATE TABLE IF NOT EXISTS crops (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            area REAL NOT NULL CHECK (area > 0),
            area_unit TEXT NOT NULL DEFAULT 'm²',
            crop TEXT NOT NULL DEFAULT '',
            planting_date TEXT NOT NULL DEFAULT '',
            status TEXT NOT NULL DEFAULT 'Fallow',
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS activities (
            id INTEGER PRIMARY KEY,
            category TEXT not NULL,
            description TEXT,
            crop_id INTEGER REFERENCES crops(id) ON DELETE SET NULL,
            material_id INTEGER,
            scheduled_at TEXT NOT NULL,
            status TEXT,
            started_at TEXT,
            completed_at TEXT,
            is_paused INTEGER NOT NULL DEFAULT 0,
            is_stalled INTEGER NOT NULL DEFAULT 0,
            is_canceled INTEGER NOT NULL DEFAULT 0,
            priority TEXT NOT NULL DEFAULT 'normal',
            notes TEXT,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS activity_material (
            id INTEGER PRIMARY KEY,
            activity_id INTEGER NOT NULL,
            quantity REAL NOT NULL,
            unit TEXT NOT NULL,
            cost REAL DEFAULT 0
        );
        ",
    )?;

    Ok(conn)
}