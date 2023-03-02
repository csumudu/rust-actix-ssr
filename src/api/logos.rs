use actix_web::http::Error;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;

use crate::models::{DBPool, RustLogos};
use crate::schema::rust_logos::dsl::*;

pub async fn get_all_logos(pool: web::Data<DBPool>) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().expect("Connection ");

    let logo_data = web::block(move || rust_logos.limit(100).load::<RustLogos>(&mut conn))
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())
        .unwrap();

    Ok(HttpResponse::Ok()
        .append_header(("content-type", "application/json"))
        .append_header(("custom-header-type", "sumudu"))
        .json(logo_data.unwrap()))
}

#[cfg(test)]
mod tests {
    use actix_web::{
        test,
        web::{self, Data},
        App,
    };
    use diesel::prelude::*;
    use diesel::RunQueryDsl;

    use crate::schema::rust_logos::dsl::*;
    use crate::{
        models::{DBPool, RustLogos, RustLogosIns},
        util::db_util::get_pool,
    };

    use super::get_all_logos;

    #[actix_rt::test]
    async fn test_get_all_logos() {
        let pool = get_pool().unwrap();

        insert_test_data(&pool);

        let mut app = test::init_service(
            App::new()
                .app_data(Data::new(pool.clone()))
                .route("/api/logs", web::get().to(get_all_logos)),
        )
        .await;

        let req = test::TestRequest::get().uri("/api/logs").to_request();

        let res = test::call_service(&mut app, req).await;

        let status = res.status();

        let body = test::read_body(res).await;
        
        let logo_data: Vec<RustLogos> = serde_json::from_slice(&body).unwrap();

        assert!(logo_data.len()>=10);

        assert_eq!(status, 200);

        clean_up();
    }

    async fn insert_test_data(pool: &DBPool) {
        let mut conn = pool.get().unwrap();

        for i in 1..=10 {
            let logo = RustLogosIns {
                image_path: format!("Image path {}", i),
                name: format!("name-->{}", i),
            };

            diesel::insert_into(rust_logos)
                .values(&logo)
                .execute(&mut conn)
                .unwrap();
        }
    }

    fn clean_up() {}
}
