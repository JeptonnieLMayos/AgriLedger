use rusqlite::{Connection, Result};


pub fn insert_group(
    conn: &Connection,
    farm_id: i64,
    name: &str,
    description: &str,
    notes: &str,
) -> Result<()> {
    conn.execute(
        "INSERT INTO groups
            (farm_id, name, description, notes)
         VALUES (?1, ?2, ?3, ?4)",
        (
            farm_id,
            name,
            description,
            notes,
        ),
    )?;

    Ok(())
}

pub fn count_groups(conn: &Connection) -> Result<i64> {
    conn.query_row(
        "SELECT COUNT(*) FROM groups",
        [],
        |row| row.get(0),
    )
}

pub fn list_groups(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT farm_id, name, description, notes
         FROM groups
         ORDER BY id",
    )?;

    let rows = stmt.query_map([], |row| {
        let farm_id: i64 = row.get(0)?;
        let name: String = row.get(1)?;
        let description: String = row.get(2)?;
        let notes: String = row.get(3)?;

        Ok(format!(
            "{} \nFarm {} \nDescription: {}\nNotes: {}",
            name,
            farm_id,
            description,
            notes
        ))
    })?;

    rows.collect()
}


pub fn update_group(
    conn: &Connection,
    id: i64,
    farm_id: i64,
    name: &str,
    description: &str,
    notes: &str,
) -> Result<bool> {
    let changed = conn.execute(
        "UPDATE groups
         SET farm_id = ?1,
             name = ?2,
             description = ?3,
             notes = ?4
         WHERE id = ?5",
        (
            farm_id,
            name,
            description,
            notes,
            id,
        ),
    )?;

    Ok(changed > 0)
}

pub fn delete_group(conn: &Connection, id: i64) -> Result<bool> {
    let deleted = conn.execute(
        "DELETE FROM groups WHERE id = ?1",
        [id],
    )?;

    Ok(deleted > 0)
}


pub struct FarmRecord {
    pub id: i64,
    pub farm_id: i64,
    pub name: String,
    pub description: String,
    pub notes: String,
}

pub fn get_groups(conn: &Connection) -> Result<Vec<FarmRecord>> {
    let mut stmt = conn.prepare(
        "SELECT id, farm_id, name, description, notes
         FROM groups
         ORDER BY id",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(FarmRecord {
            id: row.get(0)?,
            farm_id: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            notes: row.get(4)?
        })
    })?;

    rows.collect()
}