use aws_smithy_http_server::{
    instrumentation::InstrumentPlugin, layer::alb_health_check::AlbHealthCheckLayer,
};
use simple::{Config, SimpleService};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let health_check_layer =
        AlbHealthCheckLayer::from_handler("/ping", |_req| async { http::status::StatusCode::OK });
    let config = Config::builder()
        .http_plugin(InstrumentPlugin)
        .layer(&health_check_layer)
        .build();
    let app = SimpleService::builder(config)
        .operation(operation)
        .build()
        .expect("failed to build an instance of SimpleService");

    let bind: SocketAddr = "127.0.0.1:6969"
        .parse()
        .expect("unable to parse the server bind address and port");
    let server = hyper::Server::bind(&bind).serve(app.into_make_service());

    // Run your service!
    if let Err(err) = server.await {
        eprintln!("server error: {:?}", err);
    }
}

use simple::{input, output};

async fn operation(_input: input::OperationInput) -> output::OperationOutput {
    todo!()
}
