mod db;

slint::include_modules!();

use rusqlite::types::Type::Null;
use slint::{ModelRc, VecModel, SharedString, ToSharedString};
use chrono::{Datelike};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = dirs::data_local_dir()
        .ok_or("Could not find the application data directory")?
        .join("AgriLedger");

    std::fs::create_dir_all(&data_dir)?;

    let db_path = data_dir.join("agriledger.db");
    let farm_conn = db::initialize::initialize(&db_path)?;
    let group_conn = db::initialize::initialize(&db_path)?;
    let field_conn = db::initialize::initialize(&db_path)?;
    let crop_conn = db::initialize::initialize(&db_path)?;
    let activity_conn = db::initialize::initialize(&db_path)?;

    



    let window = MainWindow::new()?;

    refresh_farms(&window, &farm_conn)?;
    refresh_groups(&window, &group_conn)?;
    refresh_fields(&window, &field_conn)?;
    refresh_crops(&window, &crop_conn)?;
    refresh_activities(&window, &activity_conn)?;
    
    let farm_weak_window = window.as_weak();
    let group_weak_window = window.as_weak();
    let field_weak_window = window.as_weak();
    let crop_weak_window = window.as_weak();
    let activity_weak_window = window.as_weak();

    let delete_farm_conn = db::initialize::initialize(&db_path)?;
    let delete_group_conn = db::initialize::initialize(&db_path)?;
    let delete_field_conn = db::initialize::initialize(&db_path)?;
    let delete_crop_conn = db::initialize::initialize(&db_path)?;
    let delete_activity_conn = db::initialize::initialize(&db_path)?;

    let delete_farm_weak_window = window.as_weak();
    let delete_group_weak_window = window.as_weak();
    let delete_field_weak_window = window.as_weak();
    let delete_crop_weak_window = window.as_weak();
    let delete_activity_weak_window = window.as_weak();

    // current date
    let now = chrono::Local::now().date_naive();

    window.set_current_date(Date {
        year: now.year() as i32,
        month: now.month() as i32,
        day: now.day() as i32,
    });
    // current date

    window.on_save_farm(move |farm| {
        let result = if farm.id > 0 {
            db::farms::update_farm(
                &farm_conn,
                farm.id as i64,
                farm.name.trim(),
                farm.description.trim(),
                farm.notes.trim(),
            )
        } else {
            db::farms::insert_farm(
                &farm_conn,
                farm.name.trim(),
                farm.description.trim(),
                farm.notes.trim(),
            )
            .map(|_| true)
        };

        match result {
            Ok(true) => {}
            Ok(false) => {
                eprintln!("No farm was updated. The selected farm may no longer exist.");  // log
                return;
            }
            Err(error) => {
                eprintln!("Could not save farm: {error}");  // log
                return;
            }
        }

        if let Some(window) = farm_weak_window.upgrade() {
            match refresh_farms(&window, &farm_conn) {
                Ok(()) => window.set_farm_current_page("farms".to_shared_string()),
                Err(error) => eprintln!("Could not refresh farms: {error}"),
            }
        }
    });

    window.on_save_group(move |group| {
        let result = if group.id > 0 {
            db::groups::update_group(
                &group_conn,
                group.id as i64,
                group.farm_id as i64,
                group.name.trim(),
                group.description.trim(),
                group.notes.trim(),
            )
        } else {
            db::groups::insert_group(
                &group_conn,
                group.farm_id as i64,
                group.name.trim(),
                group.description.trim(),
                group.notes.trim(),
            )
            .map(|_| true)
        };

        match result {
            Ok(true) => {}
            Ok(false) => {
                eprintln!("No group was updated. The selected group may no longer exist.");  // log
                return;
            }
            Err(error) => {
                eprintln!("Could not save group: {error}");  // log
                return;
            }
        }

        if let Some(window) = group_weak_window.upgrade() {
            match refresh_groups(&window, &group_conn) {
                Ok(()) => window.set_group_current_page("groups".to_shared_string()),
                Err(error) => eprintln!("Could not refresh groups: {error}"),
            }
        }
    });

    window.on_save_field(move |field| {
        let result = if field.id > 0 {
            db::fields::update_field(
                &field_conn,
                field.id as i64,
                field.farm_id as i64,
                field.group_id as i64,
                field.name.trim(),
                field.area as f64,
                field.notes.trim(),
            )
        } else {
            db::fields::insert_field(
                &field_conn,
                field.farm_id as i64,
                field.group_id as i64,
                field.name.trim(),
                field.area as f64,
                field.notes.trim(),
            )
            .map(|_| true)
        };

        match result {
            Ok(true) => {}
            Ok(false) => {
                eprintln!("No field was updated. The selected field may no longer exist.");  // log
                return;
            }
            Err(error) => {
                eprintln!("Could not save field: {error}");  // log
                return;
            }
        }

        if let Some(window) = field_weak_window.upgrade() {
            match refresh_fields(&window, &field_conn) {
                Ok(()) => window.set_field_current_page("fields".to_shared_string()),
                Err(error) => eprintln!("Could not refresh farms: {error}"),
            }
        }
    });

    window.on_save_crop(move |crop| {
        let area = match crop.area.trim().parse::<f64>() {
            Ok(value) if value.is_finite() => value,
            _ => {
                eprintln!("Crop area must be a valid number greater than zero.");  // log
                return;
            }
        };

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
                eprintln!("No crop was updated. The selected crop may no longer exist.");  // log
                return;
            }
            Err(error) => {
                eprintln!("Could not save crop: {error}");  // log
                return;
            }
        }

        if let Some(window) = crop_weak_window.upgrade() {
            match refresh_crops(&window, &crop_conn) {
                Ok(()) => window.set_crop_current_page("crops".to_shared_string()),
                Err(error) => eprintln!("Could not refresh crops: {error}"),
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
            match refresh_activities(&window, &activity_conn) {
                Ok(()) => window.set_activities_current_page("activities".to_shared_string()),
                Err(error) => eprintln!("Could not refresh activities: {error}"),
            }
        }
    });



    window.on_delete_farm(move |id| {
        match db::farms::delete_farm(&delete_farm_conn, id as i64) {
            Ok(true) => {
                if let Some(window) = delete_farm_weak_window.upgrade() {
                    if let Err(error) = refresh_farms(&window, &delete_farm_conn) {
                        eprintln!("Could not refresh farms: {error}");  // log
                    }
                }
            }
            Ok(false) => {
                eprintln!("No farm was deleted. ID: {id}");
            }
            Err(error) => {
                eprintln!("Could not delete farm: {error}");
            }
        }
    });

    window.on_delete_group(move |id| {
        match db::groups::delete_group(&delete_group_conn, id as i64) {
            Ok(true) => {
                if let Some(window) = delete_group_weak_window.upgrade() {
                    if let Err(error) = refresh_groups(&window, &delete_group_conn) {
                        eprintln!("Could not refresh groups: {error}");  // log
                    }
                }
            }
            Ok(false) => {
                eprintln!("No group was deleted. ID: {id}");
            }
            Err(error) => {
                eprintln!("Could not delete group: {error}");
            }
        }
    });

    window.on_delete_field(move |id| {
        match db::fields::delete_field(&delete_field_conn, id as i64) {
            Ok(true) => {
                if let Some(window) = delete_field_weak_window.upgrade() {
                    if let Err(error) = refresh_fields(&window, &delete_field_conn) {
                        eprintln!("Could not refresh fields: {error}");  // log
                    }
                }
            }
            Ok(false) => {
                eprintln!("No field was deleted. ID: {id}");
            }
            Err(error) => {
                eprintln!("Could not delete field: {error}");
            }
        }
    });

    window.on_delete_crop(move |id| {
        match db::crops::delete_crop(&delete_crop_conn, id as i64) {
            Ok(true) => {
                if let Some(window) = delete_crop_weak_window.upgrade() {
                    if let Err(error) = refresh_crops(&window, &delete_crop_conn) {
                        eprintln!("Could not refresh crops: {error}");  // log
                    }
                }
            }
            Ok(false) => {
                eprintln!("No crop was deleted. ID: {id}");
            }
            Err(error) => {
                eprintln!("Could not delete crop: {error}");
            }
        }
    });

    window.on_delete_activity(move |id| {
        match db::activities::delete_activity(&delete_activity_conn, id as i64) {
            Ok(true) => {
                if let Some(window) = delete_activity_weak_window.upgrade() {
                    if let Err(error) = refresh_activities(&window, &delete_activity_conn) {
                        eprintln!("Could not refresh activities: {error}");  // log
                    }
                }
            }
            Ok(false) => {
                eprintln!("No crop was deleted. ID: {id}");
            }
            Err(error) => {
                eprintln!("Could not delete crop: {error}");
            }
        }
    });

    window.run()?;

    Ok(())
}

fn refresh_farms(
    window: &MainWindow,
    conn: &rusqlite::Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    let farms = db::farms::get_farms(conn)?;
    let mut farm_names: Vec<SharedString> = vec![
    SharedString::from("Not specified")
    ];
    farm_names.extend(
    farms.iter()
        .map(|farm| SharedString::from(farm.name.as_str()))
        .filter(|name| !name.is_empty())
    );

    window.set_has_farms(!farms.is_empty());

    let model: Vec<FarmData> = farms
        .into_iter()
        .map(|farm| {
            let display_text = format!(
                "{}\nDescription: {}\nnotes: {}",
                farm.name,
                farm.description,
                farm.notes
            );

            FarmData {
                id: farm.id as i32,
                name: farm.name.into(),
                description: farm.description.into(),
                notes: farm.notes.into(),
                display_text: display_text.into(),
            }
        })
        .collect();

    window.set_farms(ModelRc::new(VecModel::from(model)));
    window.set_farm_names(ModelRc::new(VecModel::from(farm_names)));

    Ok(())
}

fn refresh_groups(
    window: &MainWindow,
    conn: &rusqlite::Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    let groups = db::groups::get_groups(conn)?;
    let mut group_names: Vec<SharedString> = vec![
    SharedString::from("Not specified")
    ];
    group_names.extend(
    groups.iter()
        .map(|group| SharedString::from(group.name.as_str()))
        .filter(|name| !name.is_empty())
    );

    window.set_has_groups(!groups.is_empty());

    let model: Vec<GroupData> = groups
        .into_iter()
        .map(|group| {
            let display_text = format!(
                "{} \nFarm {} \nDescription: {}\nNotes: {}",
                group.name,
                group.farm_id,
                group.description,
                group.notes
            );

            GroupData {
                id: group.id as i32,
                farm_id: group.farm_id as i32,
                name: group.name.into(),
                description: group.description.into(),
                notes: group.notes.into(),
                display_text: display_text.into(),
            }
        })
        .collect();
println!("Groups being assigned: {}", model.len());
    window.set_groups(ModelRc::new(VecModel::from(model)));
    window.set_group_names(ModelRc::new(VecModel::from(group_names)));

    Ok(())
}

fn refresh_fields(
    window: &MainWindow,
    conn: &rusqlite::Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    let fields = db::fields::get_fields(conn)?;

    let mut field_names: Vec<SharedString> = vec![
        SharedString::from("Not specified")
        ];
        field_names.extend(
            fields.iter()
            .map(|field| SharedString::from(field.name.as_str()))
            .filter(|name| !name.is_empty())
        );
        
    window.set_has_fields(!fields.is_empty());

    let model: Vec<FieldData> = fields
        .into_iter()
        .map(|field| {
            let display_text = format!(
                "{} \nFarm {} \nGroup {} \nArea: {}\nNotes: {}",
                field.name,
                field.farm_id,
                field.group_id,
                field.area,
                field.notes
            );

            FieldData {
                id: field.id as i32,
                farm_id: field.farm_id as i32,
                group_id: field.group_id as i32,
                name: field.name.into(),
                area: field.area as f32,
                notes: field.notes.into(),
                display_text: display_text.into(),
            }
        })
        .collect();

    window.set_fields(ModelRc::new(VecModel::from(model)));
    window.set_field_names(ModelRc::new(VecModel::from(field_names)));

    Ok(())
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