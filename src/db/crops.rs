use rusqlite::{Connection, Result};

use crate::models::crop::Crop;


pub fn insert_crop(
    conn: &Connection,
    name: &str,
    area: f64,
    farm_id: i64,
    field_id: i64,
    planting_date: &str,
    status: &str,
) -> Result<()> {
    conn.execute(
        "INSERT INTO crops
            (name, area, farm_id, field_id, planting_date, status)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (
            name,
            area,
            farm_id,
            field_id,
            planting_date,
            status,
        ),
    )?;

    Ok(())
}

pub fn update_crop(
    conn: &Connection,
    id: i64,
    name: &str,
    area: f64,
    farm_id: i64,
    field_id: i64,
    planting_date: &str,
    status: &str,
) -> Result<bool> {
    let changed = conn.execute(
        "UPDATE crops
         SET name = ?1,
             area = ?2,
             farm_id = ?3
             field_id = ?4,
             planting_date = ?5,
             status = ?6
         WHERE id = ?7",
        (
            name,
            area,
            farm_id,
            field_id,
            planting_date,
            status,
            id,
        ),
    )?;

    Ok(changed > 0)
}

pub fn delete_crop(conn: &Connection, id: i64) -> Result<bool> {
    let deleted = conn.execute(
        "DELETE FROM crops WHERE id = ?1",
        [id],
    )?;

    Ok(deleted > 0)
}


pub fn get_crops(conn: &Connection, farm_id: i64) -> Result<Vec<Crop>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, area, farm_id, field_id, planting_date, status
         FROM crops
         WHERE farm_id = ?1
         ORDER BY id",
    )?;

    let rows = stmt.query_map([farm_id], |row| {
        Ok(Crop {
            id: row.get(0)?,
            name: row.get(1)?,
            area: row.get(2)?,
            farm_id: row.get(3)?,
            field_id: row.get(4)?,
            planting_date: row.get(5)?,
            status: row.get(6)?,
        })
    })?;

    rows.collect()
}