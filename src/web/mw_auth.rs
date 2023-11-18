use crate::crypt::token::{Token, validate_web_token};
use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::user::{UserForAuth, UserMC};
use crate::web::{AUTH_TOKEN, set_token_cookies};
use crate::web::{Error, Result};
use async_trait::async_trait;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};
use tracing::info;

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
    mm: State<ModelManager>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    info!("{:<12} - mw_ctx_resolver", "MIDDLEWARE");
    
    let ctx_ext_result = _ctx_resolve(mm, &cookies).await;

    if ctx_ext_result.is_err() 
       && !matches!(ctx_ext_result, Err(CtxExtError::TokenNotInCookie))
    {
       cookies.remove(Cookie::named(AUTH_TOKEN))
    }

    req.extensions_mut().insert(ctx_ext_result);

    Ok(next.run(req).await)
}

async fn _ctx_resolve(mm: State<ModelManager>, cookies: &Cookies) -> CtxExtResult{
    // -- Get Token Stirng
    let token = cookies.get(AUTH_TOKEN) 
        .map(|c|c.value().to_string())
        .ok_or(CtxExtError::TokenNotInCookie)?;

    // -- Parse Token
    let token = token.parse::<Token>().map_err(|_| CtxExtError::TokenWrongFormat)?;

    // -- get UserForAuth

    let user: UserForAuth = UserMC::first_by_username(&Ctx::root_ctx(), &mm, &token.ident)
    .await 
    .map_err(|ex|CtxExtError::ModelAccessError(ex.to_string()))?
    .ok_or(CtxExtError::UserNotFound)?;
    
    // -- Validate Token
    validate_web_token(&token, &user.token_salt.to_string())
        .map_err(|_| CtxExtError::FailValidate)?;
 
    // -- Update Token
    set_token_cookies(cookies, &user.user_name, &user.token_salt.to_string())
        .map_err(|_| CtxExtError::CannotSetTokenCookie)?;

    // -- Create CtxResult
    Ctx::new(user.id).map_err(|ex| CtxExtError::CtxCreateFail(ex.to_string()))
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
    TokenWrongFormat,

    UserNotFound,
    ModelAccessError(String),
    FailValidate,
    CannotSetTokenCookie,

	CtxNotInRequestExt,
	CtxCreateFail(String),
}


// fn parse_token(token: String) -> Result<(Uuid, String, String)> {
//     let (_whole, user_id, exp, sign) = regex_captures!(
//         r#"^user-([a-f0-9]*-[a-f0-9]*-[a-f0-9]*-[a-f0-9]*-[a-f0-9]*)\.(.+).(.+)"#,
//         &token
//     )
//     .ok_or(Error::AuthFailWrongTokenFormat)?; 

//     let user_id: Uuid = user_id.parse()
//         .map_err(|_| Error::AuthFailWrongTokenFormat)?;
//     Ok((user_id, exp.to_string(), sign.to_string()))
// }