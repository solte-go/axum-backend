use crate::crypt;
use crate::web;
use crate::model;
use axum::{response::IntoResponse, http::StatusCode};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize ,strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // -- RPC 
    RpcMethodUnknown(String),
    RpcMissingParams { rpc_method: String },
    RpcFailJsonParams { rpc_method: String },

    // -- Login
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

    // -- Exturnal Modules
    SerdeJson(String),
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

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJson(value.to_string())
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
            &Self::Model(model::Error::EntryNotFound { entry, id } ) => {
                (StatusCode::BAD_REQUEST, ClientError::DataNotFound { entity: entry, id })
            }

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


#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "message", content = "detail")]
pub enum ClientError {
    LoginFail,
    NoAuth, 
    InvalidParams,
    ServiceError,
    DataNotFound { entity: &'static str, id: i64},
}