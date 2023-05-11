use simple::{Config, SimpleService};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let config = Config::builder()
        //.plugin(PluginRunsBeforeAuthenticate)
        // If we don't call this to configure it then `build()` will yield an error.
        .aws_auth_authenticate("authenticator".to_owned()) // Applies authn plugin
        //.plugin(PluginRunsAfterAuthenticate)
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
