use std::collections::HashMap;

use crate::database::models::{ProjectItem, StartScheme};
use crate::database::{project_item, start_scheme};
use crate::error::SpacesError;
use crate::utils::program;

use crate::UserManagementWrapper;

#[tauri::command]
pub fn get_project_item_list(
    manage: tauri::State<UserManagementWrapper>,
) -> Result<Vec<ProjectItem>, SpacesError> {
    match project_item::get_project_items(&mut manage.0.lock().unwrap().conn, false) {
        Ok(val) => Ok(val),
        Err(e) => Err(SpacesError::from_str(&e.to_string())),
    }
}

#[tauri::command]
pub fn add_project_item(
    item: ProjectItem,
    manage: tauri::State<UserManagementWrapper>,
) -> Result<(), SpacesError> {
    match project_item::create_project_item(&mut manage.0.lock().unwrap().conn, item) {
        Ok(()) => Ok(()),
        Err(e) => Err(SpacesError::from_str(&e.to_string())),
    }
}

#[tauri::command]
pub fn update_project_item(
    item: ProjectItem,
    manage: tauri::State<UserManagementWrapper>,
) -> Result<(), SpacesError> {
    match project_item::update_project_item(&mut manage.0.lock().unwrap().conn, item) {
        Ok(()) => Ok(()),
        Err(e) => Err(SpacesError::from_str(&e.to_string())),
    }
}

#[tauri::command]
pub fn update_project_item_property(
    id: String,
    updates: HashMap<String, String>,
    manage: tauri::State<UserManagementWrapper>,
) -> Result<(), SpacesError> {
    match project_item::update_project_item_property(
        &mut manage.0.lock().unwrap().conn,
        id,
        updates,
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(SpacesError::from_str(&e.to_string())),
    }
}

#[tauri::command]
pub fn delete_project_item(
    deleted_id: String,
    manage: tauri::State<UserManagementWrapper>,
) -> Result<(), SpacesError> {
    match project_item::delete_project_item(&mut manage.0.lock().unwrap().conn, &deleted_id) {
        Ok(()) => Ok(()),
        Err(e) => Err(SpacesError::from_str(&e.to_string())),
    }
}

#[tauri::command]
pub fn get_scheme_by_id(
    scheme_id: String,
    manage: tauri::State<UserManagementWrapper>,
) -> Result<StartScheme, SpacesError> {
    match start_scheme::get_scheme_by_id(&mut manage.0.lock().unwrap().conn, &scheme_id) {
        Ok(scheme_op) => match scheme_op {
            Some(scheme) => Ok(scheme),
            None => Err(SpacesError::from_str("未找到默认启动方式！")),
        },
        Err(e) => Err(SpacesError::from_str(&e.to_string())),
    }
}

#[tauri::command]
pub fn get_start_scheme_list(
    project_id: String,
    manage: tauri::State<UserManagementWrapper>,
) -> Result<Vec<StartScheme>, SpacesError> {
    match start_scheme::get_start_schemes(&mut manage.0.lock().unwrap().conn, &project_id) {
        Ok(val) => Ok(val),
        Err(e) => Err(SpacesError::from_str(&e.to_string())),
    }
}

#[tauri::command]
pub fn add_start_scheme(
    scheme: StartScheme,
    manage: tauri::State<UserManagementWrapper>,
) -> Result<(), SpacesError> {
    match start_scheme::create_start_scheme(&mut manage.0.lock().unwrap().conn, scheme) {
        Ok(()) => Ok(()),
        Err(e) => Err(SpacesError::from_str(&e.to_string())),
    }
}

#[tauri::command]
pub fn update_start_scheme(
    scheme: StartScheme,
    manage: tauri::State<UserManagementWrapper>,
) -> Result<(), SpacesError> {
    match start_scheme::update_start_scheme(&mut manage.0.lock().unwrap().conn, scheme) {
        Ok(()) => Ok(()),
        Err(e) => Err(SpacesError::from_str(&e.to_string())),
    }
}

#[tauri::command]
pub fn delete_start_scheme(
    deleted_id: String,
    manage: tauri::State<UserManagementWrapper>,
) -> Result<(), SpacesError> {
    match start_scheme::delete_start_scheme(&mut manage.0.lock().unwrap().conn, &deleted_id) {
        Ok(()) => Ok(()),
        Err(e) => Err(SpacesError::from_str(&e.to_string())),
    }
}

#[tauri::command]
pub fn open_program(path: String) -> Result<String, String> {
    match program::open_file_or_program(&path, &[]) {
        Ok(status) => Ok(status.to_string()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn execute_command(command: String) -> Result<String, String> {
    match program::execute_command(&command, &[]) {
        Ok(status) => Ok(status.to_string()),
        Err(err) => Err(err.to_string()),
    }
}
