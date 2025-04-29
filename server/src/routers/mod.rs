use auth::*;
use salvo::Router;

use crate::{config, hoops::jwt};

pub mod auth;
pub mod user;

pub fn create_router() -> Router {
    Router::with_path("/api")
        .push(
            Router::with_path("/auth")
                .push(Router::with_path("/login").post(post_login))
        )
        .push(
            Router::with_path("/user")
                .hoop(jwt::auth_hoop(&config::get().jwt))
                .push(Router::with_path("/add").post(user::add_user))
                .push(Router::with_path("/profile").get(user::get_user_profile)),
        )
}
