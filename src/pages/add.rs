use std::collections::HashMap;

use crate::models::RustLogos;
use actix_web::{http, web, Error, HttpResponse};
use diesel::prelude::*;
use handlebars::Handlebars;
use regex::Regex;

use crate::models::{DBPool, RustLogosIns};

use crate::schema::rust_logos::dsl::*;

pub async fn add_logo_page(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let body = hb.render("add_logo", &{}).unwrap();
    HttpResponse::Ok().body(body)
}

pub async fn insert_loago(
    pool: web::Data<DBPool>,
    mut parts: awmp::Parts,
) -> Result<HttpResponse, Error> {
    let mut con = pool.get().expect("Connection required");

    let file = parts
        .files
        .take("image")
        .pop()
        .and_then(|f| f.persist_in("./static/images").ok())
        .unwrap_or_default();

    print!("File saved");

    let field_map: HashMap<_, _> = parts.texts.as_pairs().into_iter().collect();
    let path = file.to_string_lossy().to_string().replace("\\", "/");
    
    println!("Image Path---->{}", path);
    
    
    let draft_logo = RustLogosIns {
        image_path: path,
        name: field_map.get("name").unwrap().to_string(),
    };
    

    web::block(move || {
        diesel::insert_into(rust_logos)
            .values(&draft_logo)
            .execute(&mut con)
    })
    .await
    .map_err(|_| HttpResponse::InternalServerError().finish())
    .unwrap();

    Ok(HttpResponse::SeeOther()
        .append_header((http::header::LOCATION, "/"))
        .finish())
}

pub async fn view_loago(
    hb: web::Data<Handlebars<'_>>,
    pool: web::Data<DBPool>,
    logo_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().expect("Connection failed");

    let logo_data = web::block(move || {
        rust_logos
            .filter(id.eq(logo_id.into_inner()))
            .first::<RustLogos>(&mut conn)
    })
    .await
    .map_err(|_| HttpResponse::InternalServerError().finish())
    .unwrap();

    let body = hb.render("view", &logo_data.unwrap()).unwrap();

    Ok(HttpResponse::Ok().body(body))
}
