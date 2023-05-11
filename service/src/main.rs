use simple::{Config, SimpleService};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let config = Config::builder()
        // .plugin(PluginRunsBeforeAuthenticate)
        // Applies authz plugin; fails to build because authn must go before authz.
        // .aws_auth_authorize("authorizer".to_owned())
        .aws_auth_authenticate("authenticator".to_owned()) // Applies authn plugin
        // .plugin(PluginRunsAfterAuthenticate)
        .build()
        .expect("failed to build configuration for service");

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
