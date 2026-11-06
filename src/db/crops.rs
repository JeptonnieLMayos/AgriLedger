use rusqlite::{Connection, Result};


pub fn insert_crop(
    conn: &Connection,
    name: &str,
    area: f64,
    field_id: i64,
    planting_date: &str,
    status: &str,
) -> Result<()> {
    conn.execute(
        "INSERT INTO crops
            (name, area, field_id, planting_date, status)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        (
            name,
            area,
            field_id,
            planting_date,
            status,
        ),
    )?;

    Ok(())
}

pub fn count_crops(conn: &Connection) -> Result<i64> {
    conn.query_row(
        "SELECT COUNT(*) FROM crops",
        [],
        |row| row.get(0),
    )
}

pub fn list_crops(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT name, area, field_id, planting_date, status
         FROM crops
         ORDER BY id",
    )?;

    let rows = stmt.query_map([], |row| {
        let name: String = row.get(0)?;
        let area: f64 = row.get(1)?;
        let field_id: i64 = row.get(2)?;
        let date: String = row.get(3)?;
        let status: String = row.get(4)?;

        Ok(format!(
            "{}\nArea: {} m²\nField: {}\nPlanted: {}\nStatus: {}",
            name,
            area,
            if field_id > -1 { "Not specified".to_string() } else { field_id.to_string() },
            if date.is_empty() { "Not specified" } else { &date },
            status
        ))
    })?;

    rows.collect()
}


pub fn update_crop(
    conn: &Connection,
    id: i64,
    name: &str,
    area: f64,
    field_id: i64,
    planting_date: &str,
    status: &str,
) -> Result<bool> {
    let changed = conn.execute(
        "UPDATE crops
         SET name = ?1,
             area = ?2,
             field_id = ?3,
             planting_date = ?4,
             status = ?5
         WHERE id = ?6",
        (
            name,
            area,
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


pub struct CropRecord {
    pub id: i64,
    pub name: String,
    pub area: f64,
    pub field_id: i64,
    pub planting_date: String,
    pub status: String,
}

pub fn get_crops(conn: &Connection) -> Result<Vec<CropRecord>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, area, field_id, planting_date, status
         FROM crops
         ORDER BY id",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(CropRecord {
            id: row.get(0)?,
            name: row.get(1)?,
            area: row.get(2)?,
            field_id: row.get(3)?,
            planting_date: row.get(4)?,
            status: row.get(5)?,
        })
    })?;

    rows.collect()
}