use rusqlite::{Connection, Result};

use crate::models::{field::Field, group::Group, group_with_fields::GroupWithFields};

pub fn get_group_with_fields(
    conn: &Connection,
    farm_id: i64,
    group_id: i64,
) -> Result<Option<GroupWithFields>> {
    let mut stmt = conn.prepare(
        "SELECT
            g.id, g.farm_id, g.name, g.description, g.notes,
            f.id, f.farm_id, f.group_id, f.name, f.area, f.notes
         FROM groups g
         LEFT JOIN fields f ON f.group_id = g.id
         WHERE g.farm_id = ?1
           AND g.id = ?2
         ORDER BY f.id"
    )?;

    let rows = stmt.query_map([farm_id, group_id], |row| {
        let group = Group {
            id: row.get(0)?,
            farm_id: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            notes: row.get(4)?,
        };

        let field_id: Option<i64> = row.get(5)?;

        let field = match field_id {
            Some(id) => Some(Field {
                id,
                farm_id: row.get(6)?,
                group_id: row.get(7)?,
                name: row.get(8)?,
                area: row.get(9)?,
                notes: row.get(10)?,
            }),
            None => None,
        };

        Ok((group, field))
    })?;

    let mut result: Option<GroupWithFields> = None;

    for row in rows {
        let (group, field) = row?;

        let entry = result.get_or_insert_with(|| GroupWithFields {
            group,
            fields: Vec::new(),
        });

        if let Some(field) = field {
            entry.fields.push(field);
        }
    }

    Ok(result)
}