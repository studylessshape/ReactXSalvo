pub mod auth;
pub mod menu;
pub mod user;

use auth::*;
use salvo::Router;

use crate::{config, hoops::jwt};
pub fn create_api_router() -> Router {
    let jwt = &config::get().jwt;
    Router::with_path("/api")
        .push(Router::with_path("/auth").push(Router::with_path("/login").post(post_login)))
        .push(
            Router::with_path("/user")
                .hoop(jwt::auth_hoop(jwt))
                .push(Router::with_path("/add").post(user::add_user))
                .push(Router::with_path("/profile").get(user::get_user_profile))
                .push(Router::with_path("/del").delete(user::del_user)),
        )
        .push(
            Router::with_path("/menu")
                .hoop(jwt::auth_hoop(jwt))
                .push(Router::with_path("/get").get(menu::get_menu))
                .push(Router::with_path("/update").post(menu::update_menu))
                .push(Router::with_path("/del").delete(menu::delete_menu))
                .push(Router::with_path("/add").post(menu::add_menu)),
        )
}
