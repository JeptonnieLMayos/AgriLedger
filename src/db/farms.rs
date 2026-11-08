use rusqlite::{Connection, Result};


pub fn insert_farm(
    conn: &Connection,
    name: &str,
    description: &str,
    notes: &str,
) -> Result<()> {
    conn.execute(
        "INSERT INTO farms
            (name, description, notes)
         VALUES (?1, ?2, ?3)",
        (
            name,
            description,
            notes,
        ),
    )?;

    Ok(())
}

pub fn count_farms(conn: &Connection) -> Result<i64> {
    conn.query_row(
        "SELECT COUNT(*) FROM farms",
        [],
        |row| row.get(0),
    )
}

pub fn list_farms(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT name, description, notes
         FROM farms
         ORDER BY id",
    )?;

    let rows = stmt.query_map([], |row| {
        let name: String = row.get(0)?;
        let description: String = row.get(1)?;
        let notes: String = row.get(2)?;

        Ok(format!(
            "{}\nDescription: {}\nNotes: {}",
            name,
            description,
            notes
        ))
    })?;

    rows.collect()
}


pub fn update_farm(
    conn: &Connection,
    id: i64,
    name: &str,
    description: &str,
    notes: &str,
) -> Result<bool> {
    let changed = conn.execute(
        "UPDATE farms
         SET name = ?1,
             description = ?2,
             notes = ?3
         WHERE id = ?4",
        (
            name,
            description,
            notes,
            id,
        ),
    )?;

    Ok(changed > 0)
}

pub fn delete_farm(conn: &Connection, id: i64) -> Result<bool> {
    let deleted = conn.execute(
        "DELETE FROM farms WHERE id = ?1",
        [id],
    )?;

    Ok(deleted > 0)
}


pub struct FarmRecord {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub notes: String,
}

pub fn get_farms(conn: &Connection) -> Result<Vec<FarmRecord>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, notes
         FROM farms
         ORDER BY id",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(FarmRecord {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            notes: row.get(3)?,
        })
    })?;

    rows.collect()
}