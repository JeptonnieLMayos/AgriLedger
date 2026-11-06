mod db;

slint::include_modules!();

use slint::{ModelRc, VecModel, SharedString};
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
    let crop_conn = db::initialize::initialize(&db_path).unwrap();
    let activity_conn = db::initialize::initialize(&db_path).unwrap();

    let window = MainWindow::new().unwrap();

    refresh_crops(&window, &crop_conn).unwrap();
    refresh_activities(&window, &activity_conn).unwrap();

    let crop_weak_window = window.as_weak();
    let activity_weak_window = window.as_weak();

    let crop_delete_conn = db::initialize::initialize(&db_path).unwrap();
    let crop_delete_weak_window = window.as_weak();
    let activity_delete_conn = db::initialize::initialize(&db_path).unwrap();
    let activity_delete_weak_window = window.as_weak();

    // current date
    let now = chrono::Local::now().date_naive();

    window.set_current_date(Date {
        year: now.year() as i32,
        month: now.month() as i32,
        day: now.day() as i32,
    });
  
    // current date

    window.on_save_crop(move |crop| {
        let area = match crop.area.trim().parse::<f64>() {
            Ok(value) if value.is_finite() => value,
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
            db::crops::update_crop(
                &crop_conn,
                crop.id as i64,
                crop.name.trim(),
                area,
                -1,
                crop.planting_date.as_str(),
                crop.status.as_str(),
            )
        } else {
            db::crops::insert_crop(
                &crop_conn,
                crop.name.trim(),
                area,
                -1,
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

        if let Some(window) = crop_weak_window.upgrade() {
            if let Err(error) = refresh_crops(&window, &crop_conn) {
                eprintln!("Could not refresh crops: {error}");  // log
            } else {
                window.set_show_crop_form(false);
            }
        }
    });



    window.on_save_activity(move |activity| {

        let result = if activity.id > 0 {
            db::activities::update_activities(
                &activity_conn,
                activity.id as i64,
                activity.category.as_str(),
                activity.description.as_str(),
                activity.crop_id as i64,
                activity.material_id as i64,
                activity.scheduled_at.as_str(),
                activity.status.as_str(),
                activity.started_at.as_str(),
                activity.completed_at.as_str(),
                activity.is_paused as bool,
                activity.is_stalled as bool,
                activity.is_canceled as bool,
                activity.priority.as_str(),
                activity.notes.as_str(),
                activity.created_at.as_str(),
                activity.updated_at.as_str()
            )
        } else {
            db::activities::insert_activity(
                &activity_conn,
                activity.category.as_str(),
                activity.description.as_str(),
                activity.crop_id as i64,
                activity.material_id as i64,
                activity.scheduled_at.as_str(),
                activity.status.as_str(),
                activity.started_at.as_str(),
                activity.completed_at.as_str(),
                activity.is_paused as bool,
                activity.is_stalled as bool,
                activity.is_canceled as bool,
                activity.priority.as_str(),
                activity.notes.as_str(),
                activity.created_at.as_str(),
                activity.updated_at.as_str()
            )
            .map(|_| true)
        };

        match result {
            Ok(true) => {}
            Ok(false) => {
                eprintln!("No activity was updated. The selected activity may no longer exist.");  // log
                return;
            }
            Err(error) => {
                eprintln!("Could not save activity: {error}");  // log
                return;
            }
        }

        if let Some(window) = activity_weak_window.upgrade() {
            if let Err(error) = refresh_activities(&window, &activity_conn) {
                eprintln!("Could not refresh activities: {error}");  // log
            } else {
                window.set_show_activity_form(false);
            }
        }
    });


    window.on_delete_crop(move |id| {
        match db::crops::delete_crop(&crop_delete_conn, id as i64) {
            Ok(true) => {
                if let Some(window) = crop_delete_weak_window.upgrade() {
                    if let Err(error) = refresh_crops(&window, &crop_delete_conn) {
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


    window.on_delete_activity(move |id| {
        match db::activities::delete_activity(&activity_delete_conn, id as i64) {
            Ok(true) => {
                if let Some(window) = activity_delete_weak_window.upgrade() {
                    if let Err(error) = refresh_activities(&window, &activity_delete_conn) {
                        eprintln!("Could not refresh activities: {error}");  // log
                    }
                }
            }
            Ok(false) => {
                eprintln!("No activity was deleted. It may already be gone.");
            }
            Err(error) => {
                eprintln!("Could not delete activity: {error}");
            }
        }
    });

    window.run().unwrap();
}

fn refresh_crops(
    window: &MainWindow,
    conn: &rusqlite::Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    let crops = db::crops::get_crops(conn)?;
    let mut crop_names: Vec<SharedString> = vec![
    SharedString::from("Not specified")
    ];
    crop_names.extend(
    crops.iter()
        .map(|crop| SharedString::from(crop.name.as_str()))
        .filter(|name| !name.is_empty())
    );

    window.set_has_crops(!crops.is_empty());

    let model: Vec<CropData> = crops
        .into_iter()
        .map(|crop| {
            let display_text = format!(
                "{}\nArea: {} m²\nField: {}\nPlanted: {}\nStatus: {}",
                crop.name,
                crop.area,
                if crop.field_id == -1 { "Not specified".to_string() } else { crop.field_id.to_string() },
                if crop.planting_date.is_empty() { "Not specified" } else { &crop.planting_date },
                crop.status
            );

            CropData {
                id: crop.id as i32,
                name: crop.name.into(),
                area: crop.area.to_string().into(),
                field: crop.field_id.to_string().into(),
                planting_date: crop.planting_date.into(),
                status: crop.status.into(),
                display_text: display_text.into(),
            }
        })
        .collect();

    window.set_crops(ModelRc::new(VecModel::from(model)));
    window.set_crop_names(ModelRc::new(VecModel::from(crop_names)));

    Ok(())
}


fn refresh_activities(
    window: &MainWindow,
    conn: &rusqlite::Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    let activities = db::activities::get_activities(conn)?;

    window.set_has_activities(!activities.is_empty());

    let model: Vec<ActivityData> = activities
        .into_iter()
        .map(|activity| {
            let display_text = format!(
            "\nCategory: {}\n Crop: {}\nStatus: {}\nScheduled At: {}\nDescription: {}",
            activity.category,
            activity.crop_id,
            activity.status,
            activity.scheduled_at,
            activity.description
            );

            ActivityData {
                id: activity.id as i32,
                category: activity.category.into(),
                description: activity.description.into(),
                crop_id: activity.crop_id as i32,
                material_id: activity.material_id as i32,
                scheduled_at: activity.scheduled_at.into(),
                status: activity.status.into(),
                started_at: activity.started_at.into(),
                completed_at: activity.completed_at.into(),
                is_paused: activity.is_paused as bool,
                is_stalled: activity.is_stalled as bool,
                is_canceled: activity.is_canceled as bool,
                priority: activity.priority.into(),
                notes: activity.notes.into(),
                created_at: activity.created_at.into(),
                updated_at: activity.updated_at.into(),
                display_text: display_text.into()
            }
        })
        .collect();

    window.set_activities(ModelRc::new(VecModel::from(model)));

    Ok(())
}