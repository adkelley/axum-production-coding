use axum::handler::HandlerWithoutStateExt;
use axum::http::StatusCode;
use axum::routing::{any_service, MethodRouter};
use tower_http::services::ServeDir;

const WEB_FOLDER: &str = "web-folder";

// Note: Here we can just return a MethodRouter rather than a full Router
// since ServeDir is a service.

pub fn serve_dir() -> MethodRouter {
    async fn handle_404() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "Resource not Found")
    }

    any_service(ServeDir::new(WEB_FOLDER).not_found_service(handle_404.into_service()))
}
