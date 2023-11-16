use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::web::AUTH_TOKEN;
use crate::web::{Error, Result};
use async_trait::async_trait;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};
use tracing::info;
use uuid::Uuid;

#[allow(dead_code)] // For now, until we have the rpc.
pub async fn mw_require_auth<B>(
    ctx: Result<Ctx>,
    req: Request<B>, 
    next: Next<B>,
) -> Result<Response> {
    info!("{:<12} - require_auth - {ctx:?}", "MIDDLEWARE");

    ctx?;

    // TODO Token validation
    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver<B>(
    _mc: State<ModelManager>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    info!("{:<12} - mw_ctx_resolver", "MIDDLEWARE");
    
    let auth_token: Option<String> = cookies.get(AUTH_TOKEN)
    .map(|c| c.value().to_string());

    let result_ctx = match auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token) 
        {
            Ok((user_id, _exp, _sing)) => {
                Ok(Ctx::new(user_id))
            }
            Err(e) => Err(e),
        };

    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) {
        cookies.remove(Cookie::named(AUTH_TOKEN))
    }

    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

#[async_trait]
impl <S: Send + Sync>FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self>{
        info!("{:<12} - ctx", "EXTRACTOR");
      
      parts
        .extensions
        .get::<CtxExtResult>()
        .ok_or(Error::CtxExt(CtxExtError::CtxNotInRequestExt))?
        .clone()
        .map_err(Error::CtxExt)
    }
}


type CtxExtResult = core::result::Result<Ctx, CtxExtError>;

#[derive(Clone, Serialize, Debug)]
pub enum CtxExtError {
	TokenNotInCookie,
	CtxNotInRequestExt,
	CtxCreateFail(String),
}


fn parse_token(token: String) -> Result<(Uuid, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-([a-f0-9]*-[a-f0-9]*-[a-f0-9]*-[a-f0-9]*-[a-f0-9]*)\.(.+).(.+)"#,
        &token
    )
    .ok_or(Error::AuthFailWrongTokenFormat)?; 

    let user_id: Uuid = user_id.parse()
        .map_err(|_| Error::AuthFailWrongTokenFormat)?;
    Ok((user_id, exp.to_string(), sign.to_string()))
}