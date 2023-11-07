mod error;
pub mod pwd;

pub  use self::error::{Error, Result};

use hmac::{Hmac, Mac};
use sha2::Sha512;

pub struct EncryptContent {
    pub content: String,
    pub sait: String,
}

pub fn encrypt_into_b64u(key: &[u8], encoded_content: &EncryptContent)
    -> Result<String> {
        let EncryptContent {content, sait } = encoded_content;

        let mut hmac_sha512 = 
            Hmac::<Sha512>::new_from_slice(key).map_err(|_| Error::KeyFailHmac)?;
        
        hmac_sha512.update(content.as_bytes());
        hmac_sha512.update(sait.as_bytes());

        let hmac_result = hmac_sha512.finalize();
        let result_bytes = hmac_result.into_bytes();

        Ok(base64_url::encode(&result_bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Result, Ok};
    use rand::RngCore;

    #[test]
    fn test_encrypt_into_b64u_ok() -> Result<()> {

        let mut fx_key = [0u8; 64]; // 512 bits = 64 bytes
        rand::thread_rng().fill_bytes(&mut fx_key);
        
        let fx_encoded_content = EncryptContent {
            content: "The Pupu".to_string(),
            sait: "transverse carrots".to_string(),
        };

        let fx_res = encrypt_into_b64u(&fx_key, &fx_encoded_content)?;

        let fx_control_res = encrypt_into_b64u(&fx_key, &fx_encoded_content)?;

        assert_eq!(fx_res, fx_control_res);
        println!("->> {fx_res}");

        Ok(())
    }
}