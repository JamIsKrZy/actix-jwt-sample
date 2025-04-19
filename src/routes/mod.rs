use actix_web::{web::{self, ServiceConfig}, HttpResponse, Responder};
use serde_json::json;

mod auth;
mod admin;

struct TODO;

impl Responder for TODO{
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        HttpResponse::NotImplemented().json(json!({
            "error": "route is still in development!"
        }))
    }
}


pub fn config(cfg: &mut ServiceConfig){
    cfg
    .service(
        web::scope("/login")
            .default_service(web::to(|| async {TODO}))
            .service(auth::login_user)
            .service(auth::register_user)
            .service(auth::get_hash_password)
            .service(auth::verify_user)

    )
    .service(
        web::scope("/info")
            .default_service(web::to(|| async {TODO}))
    )
    .service(
        web::scope("/admin")
            .default_service(web::to(|| async {TODO}))
            .service(admin::new_user)
            .service(admin::get_list)

    )


    ;
}


