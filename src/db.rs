
use rusqlite::{Connection, Result};
use std::path::Path;

pub fn initialize(path: &Path) -> Result<Connection> {
    let conn = Connection::open(path)?;

    conn.execute_batch(
        "
        PRAGMA foreign_keys = ON;

        CREATE TABLE IF NOT EXISTS fields (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            area REAL NOT NULL CHECK (area > 0),
            area_unit TEXT NOT NULL DEFAULT 'm²',
            crop TEXT NOT NULL DEFAULT '',
            planting_date TEXT NOT NULL DEFAULT '',
            status TEXT NOT NULL DEFAULT 'Fallow',
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        ",
    )?;

    Ok(conn)
}

pub fn insert_field(
    conn: &Connection,
    name: &str,
    area: f64,
    area_unit: &str,
    crop: &str,
    planting_date: &str,
    status: &str,
) -> Result<()> {
    conn.execute(
        "INSERT INTO fields
            (name, area, area_unit, crop, planting_date, status)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (
            name,
            area,
            area_unit,
            crop,
            planting_date,
            status,
        ),
    )?;

    Ok(())
}

pub fn count_fields(conn: &Connection) -> Result<i64> {
    conn.query_row(
        "SELECT COUNT(*) FROM fields",
        [],
        |row| row.get(0),
    )
}

pub fn list_fields(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT name, area, area_unit, crop, planting_date, status
         FROM fields
         ORDER BY id",
    )?;

    let rows = stmt.query_map([], |row| {
        let name: String = row.get(0)?;
        let area: f64 = row.get(1)?;
        let unit: String = row.get(2)?;
        let crop: String = row.get(3)?;
        let date: String = row.get(4)?;
        let status: String = row.get(5)?;

        Ok(format!(
            "{}\nArea: {} {}\nCrop: {}\nPlanted: {}\nStatus: {}",
            name,
            area,
            unit,
            if crop.is_empty() { "Not specified" } else { &crop },
            if date.is_empty() { "Not specified" } else { &date },
            status
        ))
    })?;

    rows.collect()
}
