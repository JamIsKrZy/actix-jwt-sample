use std::sync::Mutex;

use actix_web::web;
use jsonwebtoken::{Header, Validation};

use crate::models::{auth::AuthUser, user::{UnverifiedUser, UserError, VerifiedUser}};



pub struct AppState{
    pub header: Header,
    pub validation: Validation
}

impl AppState{

    pub fn default() -> web::Data<AppState>{
        web::Data::new(
            AppState { header: Header::default(), validation: Validation::default() }
        )
    }
}


pub struct DbDummy(
    pub Mutex<Vec<VerifiedUser>>
);

impl DbDummy{

    pub fn init_app_data() -> web::Data<Self>{
        web::Data::new(Self(
            Mutex::new(Vec::new())
        ))
    }

    pub fn insert_user(&self, user: VerifiedUser){
        let mut lock = self.0.lock().expect("Poisoned!");
        lock.push(user);
    }

    pub fn find_user_salt(&self, user: &AuthUser) -> Option<String> {
        let lock = self.0.lock().expect("Poisoned!");
        lock.iter()
            .find(|v| v.email == user.username || v.username == user.username)
            .and_then(|v| v.salt.clone())
    }

    pub fn is_available(&self, user: &UnverifiedUser) -> bool {
        let lock = self.0.lock().expect("Poisoned!");
        lock.iter().find(|&v| v.email == user.email).is_none()
    }

    pub fn verified_user_available(&self, user: &VerifiedUser) -> bool {
        let lock = self.0.lock().expect("Poisoned!");
        lock.iter().find(|&v| {
            v.email == user.username || v.username == user.username   
        }).is_none()
    }

    pub fn find_user(&self, user: AuthUser) -> Result<VerifiedUser, UserError>{
        let lock = self.0.lock().expect("Poisoned!");
        match lock.iter().find(|&v| {
            (v.email == user.username || v.username == user.username) && v.password == user.password   
        }) {
            Some(v) => Ok(v.clone()),
            None => Err(UserError::MismatchPassword),
        }

    }
}