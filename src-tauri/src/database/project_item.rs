use std::collections::HashMap;

use diesel::{prelude::*, result::Error, SqliteConnection};

use super::models::project_item::dsl::*;
use super::models::NewProjectItem;
use super::{models::project_item, models::ProjectItem};

pub fn get_project_items(
    conn: &mut SqliteConnection,
    deleted: bool,
) -> Result<Vec<ProjectItem>, Error> {
    let results = match deleted {
        true => project_item
            .filter(deleted_at.is_not_null())
            .load::<ProjectItem>(conn)?,

        false => project_item
            .filter(deleted_at.is_null())
            .load::<ProjectItem>(conn)?,
    };

    Ok(results)
}

pub fn create_project_item<'a>(
    conn: &mut SqliteConnection,
    new_project_item: ProjectItem,
) -> Result<(), Error> {
    // 构建待插入对象
    let new_project_item = NewProjectItem {
        id: &new_project_item.id,
        name: &new_project_item.name,
        description: &new_project_item.description,
        default_scheme: new_project_item.default_scheme,
        icon: &new_project_item.icon,
        used_at: &new_project_item.used_at,
        deleted_at: new_project_item.deleted_at,
        path: &new_project_item.path,
        favorite: new_project_item.favorite,
        tags: &new_project_item.tags,
    };

    // 插入到数据库
    diesel::insert_into(project_item::table)
        .values(&new_project_item)
        .execute(conn)?;

    Ok(())
}

pub fn update_project_item_property(
    connection: &mut SqliteConnection,
    update_id: String,
    updates: HashMap<String, String>,
) -> Result<usize, Error> {
    let mut query = String::from("UPDATE project_item SET ");

    for (column, value) in updates.iter() {
        // 注意：此处可能存在SQL注入的风险，请确保传入的值是可信的，或者使用额外的验证和转义机制。
        query.push_str(&format!("{} = '{}', ", column, value));
    }

    // 移除最后一个逗号和空格
    query.pop();
    query.pop();

    query.push_str(&format!(" WHERE id = '{}';", update_id));

    // 使用sql_query直接执行原生SQL
    diesel::sql_query(query).execute(connection)
}

pub fn update_project_item(
    conn: &mut SqliteConnection,
    update_project_item: ProjectItem,
) -> Result<(), Error> {
    diesel::update(project_item.filter(id.eq(update_project_item.id)))
        .set((
            name.eq(update_project_item.name),
            description.eq(update_project_item.description),
            icon.eq(update_project_item.icon),
            used_at.eq(update_project_item.used_at),
            deleted_at.eq(update_project_item.deleted_at),
            path.eq(update_project_item.path),
            favorite.eq(update_project_item.favorite),
            tags.eq(update_project_item.tags),
        ))
        .execute(conn)?;

    Ok(())
}

pub fn delete_project_item(conn: &mut SqliteConnection, deleted_id: &str) -> Result<(), Error> {
    diesel::delete(project_item.filter(id.eq(deleted_id))).execute(conn)?;

    Ok(())
}
