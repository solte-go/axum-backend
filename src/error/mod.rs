use axum::{response::IntoResponse, http::StatusCode};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,

    //Auth Error

    AuthFaliNoAuthTokenCookie,
    AuthFailWrongTokenFormat,

    //Model Error
    TicketDeleteFailNotFound {id: String},
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

        match self {
            Self::LoginFail => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            },
            Self::AuthFaliNoAuthTokenCookie => {
                (StatusCode::UNAUTHORIZED, "Unauthiruzed").into_response()
            },
            Self::AuthFailWrongTokenFormat => {
                (StatusCode::UNAUTHORIZED, "Unauthiruzed").into_response()
            },
            Self::TicketDeleteFailNotFound{id:_} => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }        
        }
    }
}