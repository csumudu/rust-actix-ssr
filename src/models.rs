use diesel::{r2d2::ConnectionManager, Insertable, PgConnection, Queryable};
use r2d2::Pool;
use serde::{Deserialize, Serialize};



#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct RustLogos {
    pub id: i32,
    pub name: String,
    pub image_path: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = super::schema::rust_logos)]
pub struct RustLogosIns {
    pub name: String,
    pub image_path: String,
}

#[derive(Serialize, Debug)]
pub struct IndexTemplateData {
    pub project_name: String,
    pub logos: Vec<RustLogos>,
}

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
