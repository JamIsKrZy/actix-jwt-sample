use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;

use crate::{app_state::DbDummy, models::user::VerifiedUser, routes::TODO};




#[post("/new-user")]
pub async fn new_user(
    user: web::Json<VerifiedUser>,
    appdata: web::Data<DbDummy>
) -> impl Responder{
    
    if user.salt.is_some() {return HttpResponse::BadRequest().body(format!("Found salt! {:?}", user.salt ))}

    if !appdata.verified_user_available(&user.0) {
        return HttpResponse::BadRequest().body("Found salt!")
    }
    
    let (hash_pass, salt) = match VerifiedUser::hash_password(&user.0){
        Ok(e) => e,
        Err(_) => return HttpResponse::InternalServerError().body("Something wrong in hashin password!"),
    };

    let user  = VerifiedUser {
        email: user.email.clone(),
        username: user.username.clone(),
        password: hash_pass,
        salt: Some(salt)
    };

    appdata.insert_user(user);
    HttpResponse::Ok().body("Added to the list")
}

#[get("/list")]
pub async fn get_list(
    appdata: web::Data<DbDummy>
) -> impl Responder{

    let vec_list = appdata.0.lock().expect("poisoned").clone();
    HttpResponse::Ok().json(json!(vec_list))
}
