use std::{fmt::Display, str::FromStr};
use crate::crypt::{encrypt_into_b64u, Error, Result};

use crate::{config::config, utils::{b64u_encode, b64u_decode, now_utc_plus_sec_str, parse_utc, now_utc}};


/// String format: `ident_b64u.exp_b64u.sign_b64u`
#[derive(Debug)]
pub struct Token {
    pub ident: String,      //Identifier (username, login, etc).
    pub exp: String,        // Expiration date in RFC3339.
    pub sign_b64u: String   // Signature, base64url encoded.
}


impl FromStr for Token {
    type Err = Error; 

    fn from_str(token_str: &str) -> std::result::Result<Self, Self::Err> {
        let splits: Vec<&str> = token_str.split('.').collect();
        if splits.len() != 3 {
            return Err(Error::TokenInvalidFormat);
        }

        let (ident_b64u, exp_b64u, sign_b64u) = (splits[0], splits[1], splits[2]);
        Ok(Self { 
            ident: b64u_decode(ident_b64u)
                .map_err(|_| Error::TokenCannotDecodeIdent)?, 
            exp: b64u_decode(exp_b64u)
                .map_err(|_| Error::TokenCannotDecodeExp)?, 
            sign_b64u: sign_b64u.to_string(),
         })
    }
}

impl Display for Token{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            b64u_encode(&self.ident),
            b64u_encode(&self.exp),
            self.sign_b64u
        )
    }
}

pub fn generate_web_token(user: &str, salt: &str) -> Result<Token> {
    let config = &config();
    _generate_token(user, config.token_duration_sec, salt, &config.token_key)
}

pub fn validate_web_token(original_token: &Token, salt: &str) -> Result<()> {
    let config = &config();

    _validate_token_sign_and_exp(
        original_token, 
        salt, 
        &config.token_key)?;

        Ok(())
}


fn _generate_token(ident: &str, duration_sec: f64, salt: &str, key: &[u8]) -> Result<Token> { 
    let ident = ident.to_string();
	let exp = now_utc_plus_sec_str(duration_sec)?;

	// -- Sign the two first components.
	let sign_b64u = _token_sign_into_b64u(&ident, &exp, salt, key)?;

	Ok(Token {
		ident,
		exp,
		sign_b64u,
	})
} 

fn _validate_token_sign_and_exp(original_token: &Token, salt: &str, key: &[u8]) -> Result<()> { 
    let new_sign_b64u = _token_sign_into_b64u(
        &original_token.ident, 
        &original_token.exp, 
        salt, 
        key)?;

    if new_sign_b64u != original_token.sign_b64u { 
        return Err(Error::TokenSignatureNotMatching);
    }  

    let origin_exp = parse_utc(&original_token.exp)
        .map_err(|_| Error::TokenExpNotIso)?;

    let now = now_utc();

    if origin_exp < now {
        return Err(Error::TokenExpired);
    }

    Ok(())
} 

/// Create token signature from token parts and salt.
fn _token_sign_into_b64u(ident: &str, exp: &str, salt: &str, key: &[u8]) -> Result<String>{  
    let content = format!("{}.{}", b64u_encode(ident), b64u_encode(exp));
    let sugnature = encrypt_into_b64u(
        key, 
        &super::EncryptContent {
            content, 
            salt: salt.to_string(), 
        },
    )?;

    Ok(sugnature)
} 


#[cfg(test)]
mod tests{
    use std::{thread, time::Duration};

    use super::*;
    use anyhow::{Result, Ok};

    #[test]
    fn test_token_display_ok() -> Result<()> {
        let fx_token_str = "ZngtdG9rZW4tMDE.MjAyMy0xMS0xMVQxNjozMDowMFo.some-sign-b64-encoded";

        let fx_token = Token{
            ident: "fx-token-01".to_string(),
            exp: "2023-11-11T16:30:00Z".to_string(),
            sign_b64u: "some-sign-b64-encoded".to_string(),
        };

        // println!("--> {fx_token}");
        assert_eq!(fx_token.to_string(), fx_token_str);

        Ok(())
    }

    #[test]
    fn test_token_from_str_ok() -> Result<()> {
        let fx_token_str = "ZngtdG9rZW4tMDE.MjAyMy0xMS0xMVQxNjozMDowMFo.some-sign-b64-encoded";

        let fx_token = Token{
            ident: "fx-token-01".to_string(),
            exp: "2023-11-11T16:30:00Z".to_string(),
            sign_b64u: "some-sign-b64-encoded".to_string(),
        };

        let token: Token = fx_token_str.parse()?;

        assert_eq!(format!("{token:?}"), format!("{fx_token:?}"));

        Ok(())
    }

    #[test]
    fn test_token_ok() -> Result<()>{
        let fx_ident = "pupu-the-tester";
        let duration_sec:f64 = 0.02;
        let salt = "paper";
        let key = &config().token_key;

        let token: Token = _generate_token(
            fx_ident, duration_sec, salt, key
        )?;

        thread::sleep(Duration::from_millis(10));

       let res =  _validate_token_sign_and_exp(&token, salt, key);

       res?;

       Ok(())
    }

    #[test]
    fn test_token_validate_err_expired() -> Result<()>{
        let fx_ident = "pupu-the-tester";
        let duration_sec:f64 = 0.01;
        let salt = "paper";
        let key = &config().token_key;

        let token: Token = _generate_token(
            fx_ident, duration_sec, salt, key
        )?;

        thread::sleep(Duration::from_millis(20));

       let res =  _validate_token_sign_and_exp(&token, salt, key);

       assert!(
            matches!(res, Err(Error::TokenExpired)),
            "Should have matched `Err(Error::TokenExpired)` but was `{res:?}`"
       );

       Ok(())
    }

}