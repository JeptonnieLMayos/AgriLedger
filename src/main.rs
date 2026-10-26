mod db;

slint::include_modules!();
use slint::{ModelRc, SharedString, VecModel};

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

    window.on_save_field(
        move |name, area, unit, crop, date, status| {
            let area = match area.trim().parse::<f64>() {
                Ok(value) if value.is_finite() && value > 0.0 => value,
                _ => {
                    eprintln!("Field area must be a valid number greater than zero.");
                    return;
                }
            };

            if name.trim().is_empty() {
                eprintln!("Field name is required.");
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
    match refresh_fields(&window, &conn) {
        Ok(()) => window.set_show_field_form(false),
        Err(error) => eprintln!("Could not refresh fields: {error}"),
    }
}
        },
    );

    window.run()?;

    Ok(())
}