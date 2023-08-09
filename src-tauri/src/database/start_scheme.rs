use diesel::{prelude::*, result::Error, SqliteConnection};

use super::models::start_scheme::dsl::*;
use super::models::NewStartScheme;
use super::{models::start_scheme, models::StartScheme};

pub fn get_scheme_by_id(
    conn: &mut SqliteConnection,
    scheme_id: &str,
) -> Result<Option<StartScheme>, Error> {
    let mut result: Vec<StartScheme> = start_scheme
        .filter(id.eq(scheme_id))
        .load::<StartScheme>(conn)?;

    Ok(result.pop())
}

pub fn get_start_schemes(
    conn: &mut SqliteConnection,
    search_project_id: &str,
) -> Result<Vec<StartScheme>, Error> {
    let results = start_scheme
        .filter(project_id.eq(search_project_id))
        .load::<StartScheme>(conn)?;

    Ok(results)
}

pub fn create_start_scheme<'a>(
    conn: &mut SqliteConnection,
    new_start_scheme: StartScheme,
) -> Result<(), Error> {
    // 构建待插入对象
    let new_start_scheme = NewStartScheme {
        id: &new_start_scheme.id,
        name: &new_start_scheme.name,
        project_id: &new_start_scheme.project_id,
        program: &new_start_scheme.program,
        args: &new_start_scheme.args,
    };

    // 插入到数据库
    diesel::insert_into(start_scheme::table)
        .values(&new_start_scheme)
        .execute(conn)?;

    Ok(())
}

pub fn update_start_scheme(
    conn: &mut SqliteConnection,
    update_start_scheme: StartScheme,
) -> Result<(), Error> {
    diesel::update(start_scheme.filter(id.eq(update_start_scheme.id)))
        .set((
            name.eq(update_start_scheme.name),
            project_id.eq(update_start_scheme.project_id),
            program.eq(update_start_scheme.program),
            args.eq(update_start_scheme.args),
        ))
        .execute(conn)?;

    Ok(())
}

pub fn delete_start_scheme(conn: &mut SqliteConnection, deleted_id: &str) -> Result<(), Error> {
    diesel::delete(start_scheme.filter(id.eq(deleted_id))).execute(conn)?;

    Ok(())
}
