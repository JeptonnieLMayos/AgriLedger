
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


pub fn update_field(
    conn: &Connection,
    id: i64,
    name: &str,
    area: f64,
    area_unit: &str,
    crop: &str,
    planting_date: &str,
    status: &str,
) -> Result<bool> {
    let changed = conn.execute(
        "UPDATE fields
         SET name = ?1,
             area = ?2,
             area_unit = ?3,
             crop = ?4,
             planting_date = ?5,
             status = ?6
         WHERE id = ?7",
        (
            name,
            area,
            area_unit,
            crop,
            planting_date,
            status,
            id,
        ),
    )?;

    Ok(changed > 0)
}

pub fn delete_field(conn: &Connection, id: i64) -> Result<bool> {
    let deleted = conn.execute(
        "DELETE FROM fields WHERE id = ?1",
        [id],
    )?;

    Ok(deleted > 0)
}


pub struct FieldRecord {
    pub id: i64,
    pub name: String,
    pub area: f64,
    pub area_unit: String,
    pub crop: String,
    pub planting_date: String,
    pub status: String,
}

pub fn get_fields(conn: &Connection) -> Result<Vec<FieldRecord>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, area, area_unit, crop, planting_date, status
         FROM fields
         ORDER BY id",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(FieldRecord {
            id: row.get(0)?,
            name: row.get(1)?,
            area: row.get(2)?,
            area_unit: row.get(3)?,
            crop: row.get(4)?,
            planting_date: row.get(5)?,
            status: row.get(6)?,
        })
    })?;

    rows.collect()
}
