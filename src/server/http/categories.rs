mod get_category;
use axum::{routing::get, Router};
pub use get_category::*;

pub fn router<T>(router: Router<T>) -> Router<T>
where
    T: Clone + Send + Sync + 'static,
{
    router.route("/categories/:name", get(http_get_category))
}
