use actix_web::web;
use argon2::{password_hash::{self, rand_core::OsRng, SaltString}, Algorithm, Argon2, Params, PasswordHash, PasswordVerifier};
use serde::{Deserialize, Serialize};

use crate::app_state::DbDummy;


#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct UnverifiedUser {
    pub username: String,
    pub email: String,
    password: String
}

impl UnverifiedUser {

    pub fn promote(self, db: web::Data<DbDummy>) -> Result<VerifiedUser, UserError>{
        //checking in the database if email has an account
        if db.is_available(&self) { 
            return Err(UserError::TakenUser);
        }



        let (hashed_pass, salt) = self.get_hash_pass()?;

        Ok(VerifiedUser { 
            username: self.username, 
            email: self.email, 
            password: hashed_pass,
            salt: Some(salt)
        })
    }

    ///
    /// return: (String, SaltString)
    /// string - hashed password
    /// SaltString - salt
    pub fn get_hash_pass(&self) -> Result<(String, String),UserError> {
        let salt = SaltString::generate(&mut OsRng).to_string();

        let argon = Argon2::new_with_secret(
            b"banaynays", 
            Algorithm::Argon2id, 
            argon2::Version::V0x13, 
            Params::DEFAULT
        )?;
        
        let mut hash_output = [0u8;64];
        argon.hash_password_into(
            self.password.as_bytes(), 
            salt.as_str().as_bytes(), 
            &mut hash_output
        )?;

        Ok((hex::encode(hash_output), salt))
    } 
}

#[derive(Debug,Serialize, Deserialize, Clone, PartialEq)]
pub struct VerifiedUser{
    pub username: String,
    pub email: String,
    pub password: String,

    #[serde(skip_deserializing)]
    pub salt: Option<String>,
}

#[derive(Debug)]
pub enum UserError{
    InternalHashingError(String),
    InternalReverseHashError(String),
    MismatchPassword,
    TakenUser,
    SaltIsDefined
}

impl From<argon2::Error> for UserError {
    fn from(value: argon2::Error) -> Self {
        match value {
            _e => UserError::InternalHashingError(_e.to_string())
        }
    }


}

impl From<argon2::password_hash::errors::Error> for UserError {
    fn from(value: argon2::password_hash::errors::Error) -> Self {
        match value {
            _e => UserError::InternalHashingError(_e.to_string())
        }
    }
}

impl VerifiedUser {
    
    pub fn hash_password(user: &Self) -> Result<(String, String),UserError> {
        if user.salt.is_some() {
            return Err(UserError::SaltIsDefined);
        }

        let salt = SaltString::generate(&mut OsRng).to_string();

        let argon = Argon2::new_with_secret(
            b"banaynays", 
            Algorithm::Argon2id, 
            argon2::Version::V0x13, 
            Params::DEFAULT
        )?;
        
        let mut hash_output = [0u8;64];
        argon.hash_password_into(
            user.password.as_bytes(), 
            salt.as_str().as_bytes(), 
            &mut hash_output
        )?;

        Ok((hex::encode(hash_output), salt))
    } 

    pub fn verify_password(&self, hash_pas: &str) -> Result<bool, UserError>{
        let parsed_hash = PasswordHash::new(hash_pas)?;

        let argon = Argon2::new_with_secret(
            b"banaynays", 
            Algorithm::Argon2id, 
            argon2::Version::V0x13, 
            Params::DEFAULT
        )?;

        Ok(argon.verify_password(self.password.as_bytes(), &parsed_hash).is_ok())
    }
}

