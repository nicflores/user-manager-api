pub mod api;
pub mod dal;
pub mod models;
pub mod service;
pub mod utils;

use axum::{
    body::Bytes,
    extract::{MatchedPath, Request},
    http::HeaderMap,
    response::Response,
};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use dal::AwsSecretsManagerUserDal;
use service::UserService;
use std::sync::Arc;
use std::{net::SocketAddr, time::Duration};
use tower_http::{
    classify::ServerErrorsFailureClass,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::{info_span, Level, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Initialize and run the web server.
pub async fn run(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize global application state, such as the DAL and service layer.
    let user_dal = AwsSecretsManagerUserDal::new()
        .await
        .expect("Failed to initialize user DAL");

    let user_service = Arc::new(UserService::new(Arc::new(user_dal)));

    // Initialize the tracing subscriber for logging, useful for debugging and monitoring.
    init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers()?;

    // tracing_subscriber::registry()
    //     .with(
    //         tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
    //             // axum logs rejections from built-in extractors with the `axum::rejection`
    //             // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
    //             "example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace".into()
    //         }),
    //     )
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();

    // tracing_subscriber::registry()
    //     .with(tracing_subscriber::EnvFilter::new("info"))
    //     .init();
    //.with(TraceLayer::new_for_http())

    // Setup the API routes
    let app = api::init_api(user_service)
        // include trace context as header into the response
        .layer(OtelInResponseLayer::default())
        // start OpenTelemetry trace on incoming request
        .layer(OtelAxumLayer::default());

    // .layer(
    //     TraceLayer::new_for_http()
    //         .make_span_with(|request: &Request<_>| {
    //             // Log the matched route's path (with placeholders not filled in).
    //             // Use request.uri() or OriginalUri if you want the real path.
    //             let matched_path = request
    //                 .extensions()
    //                 .get::<MatchedPath>()
    //                 .map(MatchedPath::as_str);

    //             info_span!(
    //                 "http_request",
    //                 method = ?request.method(),
    //                 matched_path,
    //                 some_other_field = tracing::field::Empty,
    //             )
    //         })
    //         .on_request(|_request: &Request<_>, _span: &Span| {
    //             // You can use `_span.record("some_other_field", value)` in one of these
    //             // closures to attach a value to the initially empty field in the info_span
    //             // created above.
    //         })
    //         .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
    //             // ...
    //         })
    //         .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
    //             // ...
    //         })
    //         .on_eos(
    //             |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
    //                 // ...
    //             },
    //         )
    //         .on_failure(
    //             |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
    //                 // ...
    //             },
    //         ),
    // );

    // .layer(
    //     TraceLayer::new_for_http(), //.make_span_with(DefaultMakeSpan::new().level(Level::INFO))
    //                                 //.on_response(DefaultOnResponse::new().level(Level::INFO)),
    // );

    // Run the server
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Error creating tcp listener");

    let x: Result<(), std::io::Error> = axum::serve(listener, app).await;
    Ok(x?)
}
