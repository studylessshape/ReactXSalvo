pub mod error;

use salvo::{http::StatusCode, oapi::ToSchema, writing::Json};
use serde::Serialize;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct AppResponse<T> {
    pub code: u16,
    pub message: Option<String>,
    pub data: Option<T>,
}

impl<T> AppResponse<T> {
    #[inline]
    pub fn success(data: T) -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            message: None,
            data: Some(data),
        }
    }

    #[inline]
    pub fn error<F: Into<u16>>(code: F, message: String) -> Self {
        Self {
            code: code.into(),
            message: Some(message),
            data: None,
        }
    }
}

#[derive(Serialize, ToSchema, Clone, Copy, Debug)]
pub struct Empty {}

pub type AppResult<T> = Result<AppResponse<T>, error::AppError>;
pub type EmptyResult = Result<AppResponse<Empty>, error::AppError>;
pub type JsonResult<T> = Result<Json<AppResponse<T>>, error::AppError>;

pub fn json_ok<T>(data: T) -> JsonResult<T> {
    Ok(Json(AppResponse::<T>::success(data)))
}

pub fn empty_ok() -> JsonResult<Empty> {
    Ok(Json(AppResponse::<Empty>::success(Empty {})))
}

impl<T> From<T> for AppResponse<T> {
    fn from(value: T) -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            message: None,
            data: Some(value),
        }
    }
}
