use salvo::http::{header, ParseError, StatusCode, StatusError};
use salvo::{oapi, prelude::*};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::AppResponse;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("public: `{0}`")]
    Public(String),
    #[error("internal: `{0}`")]
    Internal(String),
    #[error("salvo internal error: `{0}`")]
    Salvo(#[from] ::salvo::Error),
    #[error("http status error: `{0}`")]
    HttpStatus(#[from] StatusError),
    #[error("http parse error:`{0}`")]
    HttpParse(#[from] ParseError),
    #[error("anyhow error:`{0}`")]
    Anyhow(#[from] anyhow::Error),
    #[error("rbatis::Error:`{0}`")]
    RbatisError(#[from] rbatis::Error),
    #[error("validation error:`{0}`")]
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

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct ErrorBody {
    pub code: i32,
    pub name: String,
    pub message: String,

}

#[async_trait]
impl Writer for AppError {
    async fn write(mut self, req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let code = match &self {
            Self::HttpStatus(e) => e.code,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        
        if let Some(content_type) = req.content_type() {
            let _ = res.add_header(header::CONTENT_TYPE, content_type.to_string(), true);
        }
        
        res.render(Json(AppResponse::<()>::error(code, self.to_string())));
    }
}

impl EndpointOutRegister for AppError {
    fn register(_components: &mut salvo::oapi::Components, _operation: &mut salvo::oapi::Operation) {
        
    }
}