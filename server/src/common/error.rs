use salvo::http::{ParseError, StatusCode, StatusError};
use salvo::prelude::*;
use thiserror::Error;

use super::AppResponse;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    Public(String),
    #[error("{0}")]
    Internal(String),
    #[error("salvo internal error: `{0}`")]
    Salvo(#[from] ::salvo::Error),
    #[error("Http status error: `{0}`")]
    HttpStatus(#[from] StatusError),
    #[error("Http parse error:`{0}`")]
    HttpParse(#[from] ParseError),
    #[error("{0}")]
    Anyhow(#[from] anyhow::Error),
    #[error("DatabaseError:`{0}`")]
    DatabaseError(#[from] sea_orm::error::DbErr),
    #[error("Validation error:`{0}`")]
    Validation(#[from] validator::ValidationErrors),
}
impl AppError {
    pub fn public<S: Into<String>>(msg: S) -> Self {
        Self::Public(msg.into())
    }

    pub fn internal<S: Into<String>>(msg: S) -> Self {
        Self::Internal(msg.into())
    }
}

#[async_trait]
impl Writer for AppError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let code = match &self {
            Self::HttpStatus(e) => e.code,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        res.render(Json(AppResponse::<()>::error(code, format!("{}", self))));
    }
}

impl EndpointOutRegister for AppError {
    fn register(_components: &mut salvo::oapi::Components, _operation: &mut salvo::oapi::Operation) {

    }
}

impl From<uuid::Error> for AppError {
    fn from(value: uuid::Error) -> Self {
        Self::Internal(format!("{}", value))
    }
}
