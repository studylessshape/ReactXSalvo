use auth::*;
use salvo::Router;

pub mod auth;
pub mod user;

pub fn create_router() -> Router {
    Router::with_path("/api").push(
        Router::with_path("/auth")
            .push(Router::with_path("/login").post(post_login))
            .push(Router::with_path("/logout").get(get_logout)),
    )
}
