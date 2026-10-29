mod db;

slint::include_modules!();

use slint::{ModelRc, VecModel};
use chrono::{Datelike};

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]


fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app.clone()).unwrap();

    let data_dir = app
        .internal_data_path()
        .expect("Could not access Android internal data directory");

    std::fs::create_dir_all(&data_dir).unwrap();

    let db_path = data_dir.join("agriledger.db");
    let conn = db::initialize(&db_path).unwrap();

    let window = MainWindow::new().unwrap();

    refresh_fields(&window, &conn).unwrap();

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
                eprintln!("Field area must be a valid number greater than zero."); // log
                return;
            }
        };

        if field.name.trim().is_empty() {
            eprintln!("Field name is required.");  // log
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
                eprintln!("No field was updated. It may have been deleted.");  // log
                return;
            }
            Err(error) => {
                eprintln!("Could not save field: {error}");  // log
                return;
            }
        }

        if let Some(window) = weak_window.upgrade() {
            if let Err(error) = refresh_fields(&window, &conn) {
                eprintln!("Could not refresh fields: {error}");  // log
            } else {
                window.set_show_field_form(false);
            }
        }
    });

    let delete_conn = db::initialize(&db_path).unwrap();
    let delete_weak_window = window.as_weak();

    window.on_delete_field(move |id| {
        match db::delete_field(&delete_conn, id as i64) {
            Ok(true) => {
                if let Some(window) = delete_weak_window.upgrade() {
                    if let Err(error) = refresh_fields(&window, &delete_conn) {
                        eprintln!("Could not refresh fields: {error}");  // log
                    }
                }
            }
            Ok(false) => {
                eprintln!("No field was deleted. It may already be gone.");
            }
            Err(error) => {
                eprintln!("Could not delete field: {error}");
            }
        }
    });

    window.run().unwrap();
}

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
                if field.crop.is_empty() {
                    "Not specified"
                } else {
                    &field.crop
                },
                if field.planting_date.is_empty() {
                    "Not specified"
                } else {
                    &field.planting_date
                },
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
