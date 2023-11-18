use crate::config::Config;

/// String format: `ident_b64u.exp_b64u.sign_b64u`
pub struct Token {
    pub ident: String,      //Identifier (username, login, etc).
    pub exp: String,        // Expiration date in RFC3339.
    pub sign_b64u: String   // Signature, base64url encoded.
}

