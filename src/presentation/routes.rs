use crate::presentation::handlers::{create_short_handler, redirect_handler};
use salvo::prelude::*;

pub fn router() -> Router {
    Router::new()
        .push(Router::with_path("/shorten").post(create_short_handler))
        .push(Router::with_path("/{code}").get(redirect_handler))
}
