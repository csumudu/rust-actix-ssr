use diesel::{Queryable, r2d2::ConnectionManager, PgConnection};
use r2d2::Pool;
use serde::{Deserialize, Serialize};
use super::schema::rust_logos;


#[derive(Queryable, Serialize,Debug)]
pub struct RustLogos {
    pub id: i32,
    pub name: String,
    pub image_path: String,
}


#[derive(Serialize, Debug)]
pub struct IndexTemplateData {
    pub project_name: String,
    pub logos: Vec<RustLogos>,
}

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
