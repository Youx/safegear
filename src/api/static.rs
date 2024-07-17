use axum::{
    body::Body,
    http::{header, Response, StatusCode, Uri},
    response::IntoResponse as _,
};

#[derive(rust_embed::Embed)]
#[folder = "webui/dist/"]
struct Assets;

pub(crate) async fn static_handler(uri: Uri) -> Response<Body> {
    let path = uri.path().trim_start_matches("/").to_string();

    match Assets::get(&path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
    }
}

pub(crate) async fn index_handler() -> Response<Body> {
    static_handler(Uri::from_static("/index.html")).await
}

pub(crate) fn list_assets() {
    tracing::info!("Static files served:");
    for filename in Assets::iter() {
        tracing::info!("- {filename}");
    }
}
