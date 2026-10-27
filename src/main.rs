mod db;

slint::include_modules!();
use std::thread::current;

use slint::{ModelRc, SharedString, VecModel};
use chrono::{Datelike, Local};

fn refresh_fields(
    window: &MainWindow,
    conn: &rusqlite::Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    let fields = db::get_fields(conn)?;

    window.set_has_fields(!fields.is_empty());

    let model: Vec<FieldData> = fields
        .into_iter()
        .map(|field| {
            let display_text = format!(
                "{}\nArea: {} {}\nCrop: {}\nPlanted: {}\nStatus: {}",
                field.name,
                field.area,
                field.area_unit,
                if field.crop.is_empty() { "Not specified" } else { &field.crop },
                if field.planting_date.is_empty() { "Not specified" } else { &field.planting_date },
                field.status
            );

            FieldData {
                id: field.id as i32,
                name: field.name.into(),
                area: field.area.to_string().into(),
                area_unit: field.area_unit.into(),
                crop: field.crop.into(),
                planting_date: field.planting_date.into(),
                status: field.status.into(),
                display_text: display_text.into(),
            }
        })
        .collect();

    window.set_fields(ModelRc::new(VecModel::from(model)));

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {



    let data_dir = dirs::data_local_dir()
        .ok_or("Could not find the application data directory")?
        .join("AgriLedger");

    std::fs::create_dir_all(&data_dir)?;

    let db_path = data_dir.join("agriledger.db");
    let conn = db::initialize(&db_path)?;


    let window = MainWindow::new()?;

    refresh_fields(&window, &conn)?;

    let weak_window = window.as_weak();

    // current date
    let now = chrono::Local::now().date_naive();

    window.set_current_date(Date {
        year: now.year() as i32,
        month: now.month() as i32,
        day: now.day() as i32,
    });


    let date = window.get_current_date();
    eprintln!("Date set in Slint: {}-{}-{}", date.year, date.month, date.day);
    // current date

    window.on_save_field(move |field| {
        let area = match field.area.trim().parse::<f64>() {
            Ok(value) if value.is_finite() && value > 0.0 => value,
            _ => {
                eprintln!("Field area must be a valid number greater than zero.");
                return;
            }
        };

        if field.name.trim().is_empty() {
            eprintln!("Field name is required.");
            return;
        }

        let result = if field.id > 0 {
            db::update_field(
                &conn,
                field.id as i64,
                field.name.trim(),
                area,
                field.area_unit.as_str(),
                field.crop.as_str(),
                field.planting_date.as_str(),
                field.status.as_str(),
            )
        } else {
            db::insert_field(
                &conn,
                field.name.trim(),
                area,
                field.area_unit.as_str(),
                field.crop.as_str(),
                field.planting_date.as_str(),
                field.status.as_str(),
            )
            .map(|_| true)
        };

        match result {
            Ok(true) => {}
            Ok(false) => {
                eprintln!("No field was updated. The selected field may no longer exist.");
                return;
            }
            Err(error) => {
                eprintln!("Could not save field: {error}");
                return;
            }
        }

        if let Some(window) = weak_window.upgrade() {
            match refresh_fields(&window, &conn) {
                Ok(()) => window.set_show_field_form(false),
                Err(error) => eprintln!("Could not refresh fields: {error}"),
            }
        }
    });

    window.run()?;

    Ok(())
}

