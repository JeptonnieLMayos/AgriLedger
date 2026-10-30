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

    refresh_crops(&window, &conn).unwrap();

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

    window.on_save_crop(move |crop| {
        let area = match crop.area.trim().parse::<f64>() {
            Ok(value) if value.is_finite() && value > 0.0 => value,
            _ => {
                eprintln!("crop area must be a valid number greater than zero."); // log
                return;
            }
        };

        if crop.name.trim().is_empty() {
            eprintln!("crop name is required.");  // log
            return;
        }

        let result = if crop.id > 0 {
            db::update_crop(
                &conn,
                crop.id as i64,
                crop.name.trim(),
                area,
                crop.area_unit.as_str(),
                crop.crop.as_str(),
                crop.planting_date.as_str(),
                crop.status.as_str(),
            )
        } else {
            db::insert_crop(
                &conn,
                crop.name.trim(),
                area,
                crop.area_unit.as_str(),
                crop.crop.as_str(),
                crop.planting_date.as_str(),
                crop.status.as_str(),
            )
            .map(|_| true)
        };

        match result {
            Ok(true) => {}
            Ok(false) => {
                eprintln!("No crop was updated. It may have been deleted.");  // log
                return;
            }
            Err(error) => {
                eprintln!("Could not save crop: {error}");  // log
                return;
            }
        }

        if let Some(window) = weak_window.upgrade() {
            if let Err(error) = refresh_crops(&window, &conn) {
                eprintln!("Could not refresh crops: {error}");  // log
            } else {
                window.set_show_crop_form(false);
            }
        }
    });

    let delete_conn = db::initialize(&db_path).unwrap();
    let delete_weak_window = window.as_weak();

    window.on_delete_crop(move |id| {
        match db::delete_crop(&delete_conn, id as i64) {
            Ok(true) => {
                if let Some(window) = delete_weak_window.upgrade() {
                    if let Err(error) = refresh_crops(&window, &delete_conn) {
                        eprintln!("Could not refresh crops: {error}");  // log
                    }
                }
            }
            Ok(false) => {
                eprintln!("No crop was deleted. It may already be gone.");
            }
            Err(error) => {
                eprintln!("Could not delete crop: {error}");
            }
        }
    });

    window.run().unwrap();
}

fn refresh_crops(
    window: &MainWindow,
    conn: &rusqlite::Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    let crops = db::get_crops(conn)?;

    window.set_has_crops(!crops.is_empty());

    let model: Vec<CropData> = crops
        .into_iter()
        .map(|crop| {
            let display_text = format!(
                "{}\nArea: {} {}\nCrop: {}\nPlanted: {}\nStatus: {}",
                crop.name,
                crop.area,
                crop.area_unit,
                if crop.crop.is_empty() {
                    "Not specified"
                } else {
                    &crop.crop
                },
                if crop.planting_date.is_empty() {
                    "Not specified"
                } else {
                    &crop.planting_date
                },
                crop.status
            );

            CropData {
                id: crop.id as i32,
                name: crop.name.into(),
                area: crop.area.to_string().into(),
                area_unit: crop.area_unit.into(),
                crop: crop.crop.into(),
                planting_date: crop.planting_date.into(),
                status: crop.status.into(),
                display_text: display_text.into(),
            }
        })
        .collect();

    window.set_crops(ModelRc::new(VecModel::from(model)));

    Ok(())
}
