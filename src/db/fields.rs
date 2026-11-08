use rusqlite::{Connection, Name, Result};


pub fn insert_field(
    conn: &Connection,
    farm_id: i64,
    group_id: i64,
    name: &str,
    area: f64,
    notes: &str,
) -> Result<()> {
    conn.execute(
        "INSERT INTO fields
            (farm_id, group_id, name, area, notes)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        (
            farm_id,
            group_id,
            name,
            area,
            notes,
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
        "SELECT farm_id, group_id, name, area, notes
         FROM fields
         ORDER BY id",
    )?;

    let rows = stmt.query_map([], |row| {
        let farm_id: i64 = row.get(0)?;
        let group_id: i64 = row.get(1)?;
        let name: String = row.get(2)?;
        let area: f64 = row.get(3)?;
        let notes: String = row.get(4)?;

        Ok(format!(
            "{} \nFarm {} \nGroup {} \nArea: {}\nNotes: {}",
            name,
            farm_id,
            group_id,
            area,
            notes
        ))
    })?;

    rows.collect()
}


pub fn update_field(
    conn: &Connection,
    id: i64,
    farm_id: i64,
    group_id: i64,
    name: &str,
    area: f64,
    notes: &str,
) -> Result<bool> {
    let changed = conn.execute(
        "UPDATE fields
         SET farm_id = ?1,
             group_id = ?2,
             name = ?3,
             area = ?4,
             notes = ?5
         WHERE id = ?6",
        (
            farm_id,
            group_id,
            name,
            area,
            notes,
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


pub struct FarmRecord {
    pub id: i64,
    pub farm_id: i64,
    pub group_id: i64,
    pub name: String,
    pub area: f64,
    pub notes: String,
}

pub fn get_fields(conn: &Connection) -> Result<Vec<FarmRecord>> {
    let mut stmt = conn.prepare(
        "SELECT id, farm_id, group_id, name, area, notes
         FROM fields
         ORDER BY id",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(FarmRecord {
            id: row.get(0)?,
            farm_id: row.get(1)?,
            group_id: row.get(2)?,
            name: row.get(3)?,
            area: row.get(4)?,
            notes: row.get(5)?
        })
    })?;

    rows.collect()
}