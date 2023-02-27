use actix_files::{Files, NamedFile};
use actix_web::web::Data;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::{
    r2d2::{ConnectionManager, Error},
    PgConnection,
};
use handlebars::Handlebars;
use std::{io::Result};

mod models;
mod pages;
mod schema;
mod util;

use crate::pages::hello::hello;
use crate::pages::index::index;
use crate::util::db_util::get_pool;

#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let mut hb = Handlebars::new();
    hb.register_templates_directory(".html", "./static/")
        .unwrap();

    let hb_ref = web::Data::new(hb);

    //DB Connection
    let pool = get_pool().unwrap();
    println!("connection pool created");

    println!("Server started");
    HttpServer::new(move || {
        App::new()
            .app_data(hb_ref.clone())
            .app_data(Data::new(pool.clone()))
            .service(Files::new("/static", "static"))
            .route("/", web::get().to(index))
            .route("/hello", web::get().to(hello))
    })
    .bind("127.0.0.1:4400")?
    .run()
    .await
}
