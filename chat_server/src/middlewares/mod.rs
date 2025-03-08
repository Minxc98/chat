use axum::Router;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::LatencyUnit;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

mod request_id;



pub fn set_router_layers(app : Router) -> Router{
    app.layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http()
                .make_span_with(
                    DefaultMakeSpan::new().include_headers(true)
                )
                .on_request(
                    DefaultOnRequest::new().level(Level::INFO)
                )
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Micros)
                ))
            .layer(CompressionLayer::new().gzip(true).br(true).deflate(true))
    )

}