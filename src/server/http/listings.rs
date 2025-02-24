mod get_listing;
use axum::{routing::get, Router};
pub use get_listing::*;

pub fn router<T>(router: Router<T>) -> Router<T>
where
    T: Clone + Send + Sync + 'static,
{
    router.route("/listings/:id", get(http_get_listing))
}
