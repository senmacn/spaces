pub const CREATE_TABLE_PROJECT_ITEM: &str = "CREATE TABLE IF NOT EXISTS project_item (
    id CHAR(10) PRIMARY KEY,
    name CHAR(100) NOT NULL,
    description CHAR(256) NOT NULL,
    default_scheme CHAR(10) NULL DEFAULT NULL,
    icon CHAR(256) NOT NULL DEFAULT '',
    used_at CHAR(100) NOT NULL,
    deleted_at CHAR(100) NULL DEFAULT NULL,
    path CHAR(100) NOT NULL,
    favorite BOOLEAN NOT NULL,
    tags CHAR(100) NOT NULL 
)
;";

pub const CREATE_TABLE_START_SCHEME: &str = "CREATE TABLE IF NOT EXISTS start_scheme (
    id CHAR(10) PRIMARY KEY,
    project_id CHAR(10),
    name CHAR(100) NOT NULL,
    program CHAR(100) NOT NULL,
    args CHAR(100) NOT NULL 
)
;";
