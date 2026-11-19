use axum::Router;
use axum::extract::OriginalUri;
use axum::response::IntoResponse;
use axum::routing::get;

use tower_http::compression::CompressionLayer;

use tracing::warn;

use crate::state::AppState;
use crate::template::HtmlTemplate;
use crate::template::about;
use crate::template::error::Error404;
use crate::template::feed;
use crate::template::home;

pub fn router() -> Router<AppState> {
  Router::new()
    .route("/", get(home))
    .route("/about", get(about))
    .route("/feed", get(feed))
    .fallback(error_404)
    .layer(CompressionLayer::new().br(true).gzip(true))
}

async fn home() -> impl IntoResponse {
  let home = home::build_template().await;
  HtmlTemplate::from(home)
}

async fn about() -> impl IntoResponse {
  let about = about::build_template().await;
  HtmlTemplate::from(about)
}

async fn feed() -> impl IntoResponse {
  let feed = feed::build_template().await;
  HtmlTemplate::from(feed)
}

async fn error_404(OriginalUri(uri): OriginalUri) -> impl IntoResponse {
  warn!("unable to find resource: {uri}");
  HtmlTemplate::from(Error404)
}
