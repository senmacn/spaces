
use spaces::database::{db::{init_db, get_connection}, models::ProjectItem, project_item::{create_project_item, update_project_item}};

#[test]
fn test_insert() {
    let mut conn = init_db();
    create_project_item(
        &mut conn,
        ProjectItem {
            id: String::from("123"),
            name: String::from("12321"),
            description: String::from("213213"),
            used_at: String::from("213"),
            deleted_at: Some(String::from("1232")),
            path: String::from("123213"),
            favorite: false,
            tags: String::from("123123"),
        },
    )
    .unwrap();
}

#[test]
fn test_update() {
    let mut conn = get_connection();
    update_project_item(
        &mut conn,
        ProjectItem {
            id: String::from("123"),
            name: String::from("2222"),
            description: String::from("2222"),
            used_at: String::from("222"),
            deleted_at: Some(String::from("222")),
            path: String::from("222"),
            favorite: false,
            tags: String::from("222"),
        },
    )
    .unwrap();
}
