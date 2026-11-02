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
            crop_id INTEGER,
            material_id INTEGER,
            scheduled_at TEXT NOT NULL,
            status TEXT,
            started_at TEXT,
            completed_at TEXT,
            notes TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (crop_id) REFERENCES crops(id),
            FOREIGN KEY (material_id) REFERENCES activity_material(id)
        );

        CREATE TABLE IF NOT EXISTS activity_material (
            id INTEGER PRIMARY KEY,
            activity_id INTEGER NOT NULL,
            quantity REAL NOT NULL,
            unit TEXT NOT NULL,
            cost REAL DEFAULT 0,
            FOREIGN KEY (activity_id) REFERENCES activities(id)
        );
        ",
    )?;

    Ok(conn)
}