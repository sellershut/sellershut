mod get_followers;
mod get_following;
mod get_user;
use axum::{routing::get, Router};
pub use get_followers::*;
pub use get_following::*;
pub use get_user::*;

use super::web_finger;

pub fn router<T>(router: Router<T>) -> Router<T>
where
    T: Clone + Send + Sync + 'static,
{
    router
        .route("/.well-known/webfinger", get(web_finger))
        //.route("/users/{user}", get(routes::users::http_get_user)) -- AXUM 0.8
        .route("/users/:name", get(http_get_user))
        .route("/users/:name/following", get(http_get_user_following))
        .route("/users/:name/followers", get(http_get_user_followers))
}
