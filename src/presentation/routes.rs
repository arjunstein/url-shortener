use crate::presentation::handlers::{
    create_short_handler, delete_url_handler, get_all_handler, redirect_handler,
};
use salvo::oapi::OpenApi;
use salvo::prelude::*;

pub fn router() -> Router {
    let api_router = Router::new()
        .path("/api/v1")
        .push(
            Router::new()
                .path("/shorten")
                .post(create_short_handler)
                .get(get_all_handler)
                .push(Router::new().path("/{code}").delete(delete_url_handler)),
        )
        .push(Router::new().path("/{code}").get(redirect_handler));

    let doc = OpenApi::default().merge_router(&api_router);

    Router::new()
        .push(api_router)
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("/documentation"))
}
