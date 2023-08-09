use diesel::table;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};

table! {
    project_item (id) {
        id -> VarChar,
        name -> VarChar,
        description -> VarChar,
        default_scheme -> Nullable<VarChar>,
        icon -> VarChar,
        used_at-> VarChar,
        deleted_at-> Nullable<VarChar>,
        path-> VarChar,
        favorite-> Bool,
        tags-> VarChar,
    }
}

// 用于查询
#[derive(
    Queryable, Identifiable, AsChangeset, serde::Serialize, serde::Deserialize, Debug, Clone,
)]
#[serde(rename_all = "camelCase")]
#[table_name = "project_item"]
pub struct ProjectItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub default_scheme: Option<String>,
    pub icon: String,
    pub used_at: String,
    pub deleted_at: Option<String>,
    pub path: String,
    pub favorite: bool,
    pub tags: String,
}

// 用于创建
#[derive(Insertable)]
#[table_name = "project_item"]
pub struct NewProjectItem<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub description: &'a str,
    pub default_scheme: Option<String>,
    pub icon: &'a str,
    pub used_at: &'a str,
    pub deleted_at: Option<String>,
    pub path: &'a str,
    pub favorite: bool,
    pub tags: &'a str,
}

table! {
    start_scheme (id) {
        id -> VarChar,
        project_id-> VarChar,
        name-> VarChar,
        program-> VarChar,
        args-> VarChar,
    }
}

#[derive(
    Queryable, Identifiable, AsChangeset, serde::Serialize, serde::Deserialize, Debug, Clone,
)]
#[serde(rename_all = "camelCase")]
#[table_name = "start_scheme"]
pub struct StartScheme {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub program: String,
    pub args: String,
}

// 用于创建
#[derive(Insertable)]
#[table_name = "start_scheme"]
pub struct NewStartScheme<'a> {
    pub id: &'a str,
    pub project_id: &'a str,
    pub name: &'a str,
    pub program: &'a str,
    pub args: &'a str,
}
