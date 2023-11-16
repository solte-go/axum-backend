use super::{Result, Error};
use crate::config::{config};
use super::{EncryptContent, encrypt_into_b64u};

pub fn encrypt_pwd(enc_content: &EncryptContent) -> Result<String>{
    let key = &config().pwd_key;

    let enpcypted = encrypt_into_b64u(key, enc_content)?;

    Ok(format!("#01#{enpcypted}"))
}

pub fn validate_pwd(enc_content: &EncryptContent, pwd_ref: &str) -> Result<()>{
    let key = encrypt_pwd(enc_content)?;

    if key == pwd_ref{
        Ok(())
    } else {
        Err(Error::PwdNotMatching)
    }
}