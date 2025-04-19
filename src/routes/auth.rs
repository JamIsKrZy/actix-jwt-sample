use actix_web::{cookie::{self, Cookie, CookieBuilder}, post, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;

use crate::{app_state::{AppState, DbDummy}, auth, models::{auth::AuthUser, user::{UnverifiedUser, VerifiedUser}}};

use super::TODO;


#[post("")]
async fn login_user(
    db: web::Data<DbDummy>,
    app_state: web::Data<AppState>,
    data: web::Json<AuthUser>
) -> impl Responder{

    //get the salt of user

        

    let auth_user = match data.get_hash_pass(){
        Ok(hash) => {
            println!("Hash: {}", hash);
            AuthUser { username: data.username.clone(), password: hash}
        },
        Err(_) => return HttpResponse::InternalServerError().json(json!({
            "error": "Theres a problem in setting up claim!"
        })),
    };

    match db.find_user(auth_user){
        Ok(v) => {

            let token = match auth::create_claims(v.username, &app_state.header){
                Ok(e) => e,
                Err(e) => return HttpResponse::InternalServerError().json(json!({
                    "error": "Theres a problem in setting up claim!"
                })),
            };


            let cookie = Cookie::build("auth-token", token)
                .http_only(true)
                .secure(false)
                .same_site(cookie::SameSite::Strict)
                .path("/")
                .finish();

            HttpResponse::Ok()
                .cookie(cookie)
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
    data: web::Json<AuthUser>
) -> impl Responder {
    match data.get_hash_pass() {
        Ok(hash_output) => {
            HttpResponse::Ok().body(format!("Hashed password {}", hex::encode(hash_output)))
        },

        Err(e) => {
            println!("{:?}", e);
            HttpResponse::NotFound().body("User not found")
        },
    }

    
}