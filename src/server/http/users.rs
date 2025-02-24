mod get_followers;
mod get_following;
mod get_user;
mod post_inbox;
use axum::{
    routing::{get, post},
    Router,
};
pub use get_followers::*;
pub use get_following::*;
pub use get_user::*;
pub use post_inbox::*;

use super::web_finger;

pub fn router<T>(router: Router<T>) -> Router<T>
where
    T: Clone + Send + Sync + 'static,
{
    router
        .route("/users/:id/inbox", post(http_post_user_inbox))
        .route("/.well-known/webfinger", get(web_finger))
        //.route("/users/{user}", get(routes::users::http_get_user)) -- AXUM 0.8
        .route("/users/:id", get(http_get_user))
        .route("/users/:id/following", get(http_get_user_following))
        .route("/users/:id/followers", get(http_get_user_followers))
}
