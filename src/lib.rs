mod db;

slint::include_modules!();

use slint::{ModelRc, SharedString, VecModel};

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

    window.on_save_field(
        move |name, area, unit, crop, date, status| {
            let area = match area.trim().parse::<f64>() {
                Ok(value) if value.is_finite() && value > 0.0 => value,
                _ => return,
            };

            if name.trim().is_empty() {
                return;
            }

            if let Err(error) = db::insert_field(
                &conn,
                name.trim(),
                area,
                unit.as_str(),
                crop.as_str(),
                date.as_str(),
                status.as_str(),
            ) {
                eprintln!("Could not save field: {error}");
                return;
            }

            if let Some(window) = weak_window.upgrade() {
                if let Err(error) = refresh_fields(&window, &conn) {
                    eprintln!("Could not refresh fields: {error}");
                } else {
                    window.set_show_field_form(false);
                }
            }
        },
    );

    window.run().unwrap();
}

fn refresh_fields(
    window: &MainWindow,
    conn: &rusqlite::Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    let fields = db::list_fields(conn)?;

    window.set_has_fields(!fields.is_empty());

    let model: Vec<SharedString> = fields
        .into_iter()
        .map(SharedString::from)
        .collect();

    window.set_field_lines(ModelRc::new(VecModel::from(model)));

    Ok(())
}
