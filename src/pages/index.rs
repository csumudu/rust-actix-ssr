use actix_web::{web, HttpResponse};

use handlebars::Handlebars;

use diesel::prelude::*;

use crate::models::{RustLogos, IndexTemplateData, DBPool};
use crate::schema::rust_logos::dsl::*;



pub async fn index(hb: web::Data<Handlebars<'_>>, pool: web::Data<DBPool>) -> HttpResponse {
    let mut connection = pool.get().expect("Connnection faliure");

    let logos = web::block(move || rust_logos.limit(10).load::<RustLogos>(&mut connection))
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())
        .unwrap();

     let data_frm_db = IndexTemplateData {
        project_name: "Catdex".to_string(),
        logos: logos.unwrap(),
    };

    println!("Data from db-->{:?}", data_frm_db);

    let body = hb.render("index", &data_frm_db).unwrap();
    HttpResponse::Ok().body(body)
}