use argon2::{password_hash::{self, rand_core::OsRng, SaltString}, Algorithm, Argon2, Params, PasswordHash, PasswordVerifier};

use serde::{Deserialize, Serialize};

use super::user::{UserError, VerifiedUser};



#[derive(Debug,Serialize, Deserialize)]
pub struct AuthUser{
    pub username: String,
    pub password: String
}



impl AuthUser{

    pub fn hashify_password(&mut self, salt: &str) -> Result<(),UserError>{
        
        let argon = Argon2::new_with_secret(
            b"banaynays", 
            Algorithm::Argon2id, 
            argon2::Version::V0x13, 
            Params::DEFAULT
        )?;

        let mut hash_output = [0u8;64];
        argon.hash_password_into(
            self.password.as_bytes(), 
            salt.as_bytes(), 
            &mut hash_output
        );

        self.password = hex::encode(hash_output);

        Ok(())
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

