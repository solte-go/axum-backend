use async_trait::async_trait;
use axum::RequestPartsExt;
use axum::extract::FromRequestParts;
use axum::http::Request;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::Cookies;
use uuid::Uuid;

use crate::ctx::Ctx;
use crate::web::AUTH_TOKEN;
use crate::{Error, Result}; 

pub async fn mw_require_auth<B>(
    ctx: Result<Ctx>,
    req: Request<B>, 
    next: Next<B>,
) -> Result<Response> {
    println!("->> {:<12} - require_auth - {ctx:?}", "MIDDLEWARE");

    ctx?;

    // TODO Token validation
    Ok(next.run(req).await)
}

#[async_trait]
impl <S: Send + Sync>FromRequestParts<S> for Ctx{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self>{
        println!("->> {:<12} - ctx", "EXTRACTOR");
        let cookies = parts.extract::<Cookies>().await.unwrap();
        
        let auth_token: Option<String> = cookies.get(AUTH_TOKEN)
        .map(|c| c.value().to_string());


        // Parse token.
        let (user_id, exp, sign) = auth_token
            .ok_or(Error::AuthFaliNoAuthTokenCookie)
            .and_then(parse_token)?;
        
        Ok(Ctx::new(user_id))
    }
}

fn parse_token(token: String) -> Result<(Uuid, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+).(.+)"#,
        &token
    )
    .ok_or(Error::AuthFailWrongTokenFormat)?; 

    let user_id: Uuid = user_id.parse()
        .map_err(|_| Error::AuthFailWrongTokenFormat)?;
    Ok((user_id, exp.to_string(), sign.to_string()))
}