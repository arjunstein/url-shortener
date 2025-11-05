use crate::presentation::handlers::{
    create_short_handler, delete_url_handler, get_all_handler, redirect_handler,
};
use salvo::prelude::*;

pub fn router() -> Router {
    Router::new()
        .path("/api/v1")
        .push(
            Router::new()
                .path("/shorten")
                .post(create_short_handler)
                .get(get_all_handler)
                .push(Router::new().path("/{code}").delete(delete_url_handler)),
        )
        .push(Router::new().path("/{code}").get(redirect_handler))
}
