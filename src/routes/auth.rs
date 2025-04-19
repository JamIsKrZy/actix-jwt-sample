use actix_web::{cookie::{self, Cookie, CookieBuilder}, post, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;

use crate::{app_state::{AppState, DbDummy}, auth, models::{auth::AuthUser, user::{UnverifiedUser}}};

use super::TODO;


#[post("")]
async fn login_user(
    db: web::Data<DbDummy>,
    app_state: web::Data<AppState>,
    mut data: web::Json<AuthUser>
) -> impl Responder{

    //get the salt of user   
    let Some(salt) =  db.find_user_salt(&data) else {
        return HttpResponse::NotFound().json(json!({
            "error": "User not found!"
        }));
    };

    data.hashify_password(salt.as_str());

    match db.find_user(data.0){
        Ok(v) => {

            let Ok(token) = auth::create_claims(v.username, &app_state.header) else {
                return HttpResponse::InternalServerError().json(json!({
                    "error": "Theres a problem in setting up claim!"
                }));
            };


            // let cookie = Cookie::build("auth-token", token)
            //     .http_only(true)
            //     .secure(false)
            //     .same_site(cookie::SameSite::Strict)
            //     .path("/")
            //     .finish();

            HttpResponse::Ok()
                // .cookie(cookie)
                .insert_header(("Authorization", format!("Bearer {}", token)))
                .json(json!({
                    "accept": "Sent token!"
                }))
        },

        Err(_) => {
            HttpResponse::Unauthorized().json(json!({
                "error": "User does not exist or Wrong Password hehe"
            }))
        },
    }
}

#[post("/register")]
async fn register_user(
    db: web::Data<DbDummy>,
    data: web::Json<UnverifiedUser>
) -> impl Responder {
    TODO
}

#[post("/verify-user")]
async fn verify_user(
    req: HttpRequest,
    data: web::Json<AuthUser>
) -> impl Responder {
    if let Some(cookie) = req.cookie("auth_token"){
        let token = cookie.value();
        
        // verify_jwt(token, validation)
        HttpResponse::Ok().body("Token Found, not verified yet")
    } else {
        HttpResponse::NotFound().body("Token not found")
    }
}

#[post("/hash-pass")]
pub async fn get_hash_password(
    db: web::Data<DbDummy>,
    mut data: web::Json<AuthUser>
) -> impl Responder {
    

    //get the salt of user   
    let Some(salt) =  db.find_user_salt(&data) else {
        return HttpResponse::NotFound().json(json!({
            "error": "User not found!"
        }));
    };

    if data.hashify_password(salt.as_str()).is_err(){
        return HttpResponse::InternalServerError().json(json!({
            "error": "Failed to hash function!"
        }));
    }

    let hash_pass = data.password.clone();
    HttpResponse::Ok().json(json!({
        "hash_pass": hash_pass
    }))

}




