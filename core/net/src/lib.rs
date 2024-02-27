use axum::routing::get;
use axum::Router;
use err::Error;
use tower::ServiceBuilder;
use tower_http::request_id::MakeRequestUuid;
use tower_http::trace::TraceLayer;
use tower_http::ServiceBuilderExt;

mod rpc;
mod tracer;

pub async fn serve() -> err::Result<(), Error> {
	let service = ServiceBuilder::new()
		.catch_panic()
		.set_x_request_id(MakeRequestUuid)
		.propagate_x_request_id();

	let service = service.layer(
		TraceLayer::new_for_http()
			.make_span_with(tracer::HttpTraceLayerHooks)
			.on_request(tracer::HttpTraceLayerHooks)
			.on_response(tracer::HttpTraceLayerHooks)
			.on_failure(tracer::HttpTraceLayerHooks),
	);

	let app = Router::new().route("/status", get(|| async {})).merge(rpc::router()).layer(service);

	// TODO: add shutdown signal
	let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
	axum::serve(listener, app).await?;

	Ok(())
}
