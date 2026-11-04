use rusqlite::{Connection, Result};


pub fn insert_activity(
    conn: &Connection,
    category: &str,
    description: &str,
    crop_id: i64,
    material_id: i64,
    scheduled_at: &str,
    status: &str,
    started_at: &str,
    completed_at: &str,
    notes: &str,
    created_at: &str,
    updated_at: &str,
) -> Result<()> {
    conn.execute(
        "INSERT INTO activities
            (category, description, crop_id, material_id, scheduled_at, status, started_at, completed_at, notes, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        (
            category,
            description,
            crop_id,
            material_id,
            scheduled_at,
            status,
            started_at,
            completed_at,
            notes,
            created_at,
            updated_at
        ),
    )?;

    Ok(())
}

pub fn count_activities(conn: &Connection) -> Result<i64> {
    conn.query_row(
        "SELECT COUNT(*) FROM activities",
        [],
        |row| row.get(0),
    )
}

pub fn list_activities(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT
            category,
            description,
            crop_id,
            material_id,
            scheduled_at,
            status,
            started_at,
            completed_at,
            notes,
            created_at,
            updated_at
         FROM activities
         ORDER BY id",
    )?;

    let rows = stmt.query_map([], |row| {
        let category: String = row.get(0)?;
        let description: String = row.get(1)?;
        let crop_id: f64 = row.get(2)?;
        let material_id: f64 = row.get(3)?;
        let scheduled_at: String = row.get(4)?;
        let status: String = row.get(5)?;
        let started_at: String = row.get(6)?;
        let completed_at: String = row.get(7)?;
        let notes: String = row.get(8)?;
        let created_at: String = row.get(9)?;
        let updated_at: String = row.get(10)?;

        Ok(format!(
            "\nCategory: {}\nDescription: {}\nNotes: {}\nStatus: {}",
            category,
            description,
            notes,
            status
        ))
    })?;

    rows.collect()
}


pub fn update_activities(
    conn: &Connection,
    id: i64,
    category: &str,
    description: &str,
    crop_id: i64,
    material_id: i64,
    scheduled_at: &str,
    status: &str,
    started_at: &str,
    completed_at: &str,
    notes: &str,
    created_at: &str,
    updated_at: &str
) -> Result<bool> {
    let changed = conn.execute(
        "UPDATE activities
         SET category = ?1,
             description = ?2,
             crop_id = ?3,
             material_id = ?4,
             scheduled_at = ?5,
             status = ?6,
             started_at = ?7,
             completed_at = ?8,
             notes = ?9,
             created_at = ?10,
             updated_at = ?11
         WHERE id = ?12",
        (
            category,
            description,
            crop_id,
            material_id,
            scheduled_at,
            status,
            started_at,
            completed_at,
            notes,
            created_at,
            updated_at,
            id
        ),
    )?;

    Ok(changed > 0)
}

pub fn delete_activity(conn: &Connection, id: i64) -> Result<bool> {
    let deleted = conn.execute(
        "DELETE FROM activities WHERE id = ?1",
        [id],
    )?;

    Ok(deleted > 0)
}


pub struct ActivityRecord {
    pub id: i64,
    pub category: String,
    pub description: String,
    pub crop_id: f64,
    pub material_id: f64,
    pub scheduled_at: String,
    pub status: String,
    pub started_at: String,
    pub completed_at: String,
    pub notes: String,
    pub created_at: String,
    pub updated_at: String
}

pub fn get_activities(conn: &Connection) -> Result<Vec<ActivityRecord>> {
    let mut stmt = conn.prepare(
        "SELECT id, category, description, crop_id, material_id, scheduled_at, status, started_at, completed_at, notes, created_at, updated_at
         FROM activities
         ORDER BY id",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(ActivityRecord {
            id: row.get(0)?,
            category: row.get(1)?,
            description: row.get(2)?,
            crop_id: row.get(3)?,
            material_id: row.get(4)?,
            scheduled_at: row.get(5)?,
            status: row.get(6)?,
            started_at: row.get(7)?,
            completed_at: row.get(8)?,
            notes: row.get(9)?,
            created_at: row.get(10)?,
            updated_at: row.get(11)?
        })
    })?;

    rows.collect()
}