use crate::crypt;
use crate::web;
use crate::model;
use axum::{response::IntoResponse, http::StatusCode};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize ,strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFailUserNameNotFound,
    LoginFailUserHasNoPassword {user_id: i64},
    LoginFailPasswordNotMatchng {user_id: i64},

    //Auth Error

    AuthFailNoAuthTokenCookie,
    AuthFailWrongTokenFormat,
    AuthFailCtxNotInRequestExt,

    //Ctx
    CtxExt(web::mw_auth::CtxExtError),

    //Model Error
    Model(model::Error),
    TicketDeleteFailNotFound {id: String},

    //Crypto
    Crypt(crypt::Error),
}

impl From<model::Error> for Error {
    fn from(value: model::Error) -> Self {
        Self::Model(value)
    }
}

impl From<crypt::Error> for Error {
    fn from(value: crypt::Error) -> Self {
        Self::Crypt(value)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response.extensions_mut().insert(self);

        response
        // match self {
        //     Self::LoginFail => {
        //         (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
        //     },
        //     Self::AuthFaliNoAuthTokenCookie => {
        //         (StatusCode::UNAUTHORIZED, "Unauthiruzed").into_response()
        //     },
        //     Self::AuthFailWrongTokenFormat => {
        //         (StatusCode::UNAUTHORIZED, "Unauthiruzed").into_response()
        //     },
        //     Self::AuthFailCtxNotInRequestExt => {
        //         (StatusCode::UNAUTHORIZED, "Unauthiruzed").into_response()
        //     },

        //     Self::TicketDeleteFailNotFound{id:_} => {
        //         (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
        //     }        
        // }
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError){
        match self {
            // -- LoginFail
            Self::LoginFailUserNameNotFound |
            Self::LoginFailUserHasNoPassword { .. } |
            Self::LoginFailPasswordNotMatchng { .. } => {
                (StatusCode::FORBIDDEN, ClientError::LoginFail)
            }

            // -- Auth
            Self::CtxExt(_)=> (StatusCode::FORBIDDEN,ClientError::NoAuth),
        
            // -- Model
            Self::TicketDeleteFailNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::InvalidParams)
            }

            // -- Fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::ServiceError,
            ),
        }
    }
}


#[derive(Debug, strum_macros::AsRefStr)]
pub enum ClientError {
    LoginFail,
    NoAuth,
    InvalidParams,
    ServiceError,
}