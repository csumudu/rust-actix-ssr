use std::{env, time::Duration};
use std::io::Error;
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::Pool;

use crate::models::DBPool;

pub fn get_pool() -> Result<DBPool,Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = Pool::builder()
        .max_size(5)
        .min_idle(Some(2))
        .idle_timeout(Some(Duration::from_secs(10)))
        .build(manager)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    Ok(pool)
}