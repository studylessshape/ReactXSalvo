pub mod cli;
pub mod model;
pub mod utils;
pub mod routers;
pub mod hoops;
pub mod config;
pub mod db;
pub mod error;

use error::AppError;
use salvo::prelude::*;
use serde::Serialize;
pub type JsonResult<T> = Result<Json<T>, error::AppError>;
pub type EmptyResult = Result<Json<Empty>, AppError>;

pub fn json_ok<T>(data: T) -> JsonResult<T> {
    Ok(Json(data))
}
#[derive(Serialize, ToSchema, Clone, Copy, Debug)]
pub struct Empty {}
pub fn empty_ok() -> JsonResult<Empty> {
    Ok(Json(Empty {}))
}