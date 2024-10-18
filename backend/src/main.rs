use crate::core::Logging;
use std::sync::Arc;

mod core;
#[cfg(feature = "grpc")]
mod grpc;

#[cfg(feature = "grpc")]
async fn grpc_function(core: Arc<Logging>) {
    tracing::info!("gRPC running");
    grpc::grpc_main(core).await.expect("gRPC failed");
    tracing::info!("gRPC stopped");
}

#[cfg(feature = "websocket")]
async fn websocket_function(core: Arc<Logging>) {
    tracing::info!("WebSocket running");
    // TODO: add WebSocket implementation
}

#[cfg(any(feature = "grpc", feature = "websocket"))]
#[tokio::main]
async fn main() {
    use logging_lib::init_tracing;

    init_tracing!("LOGGING_SERVICE_LOG");

    // todo! better method for setting (environment or config file)
    let database_url = "postgres://devuser:devpass@localhost/devdb";
    let core = Arc::new(
        Logging::new(database_url)
            .await
            .expect("Failed to initialize Logging"),
    );

    let mut handles = vec![];

    #[cfg(feature = "grpc")]
    {
        let core_clone_grpc = core.clone();
        handles.push(tokio::spawn(grpc_function(core_clone_grpc)));
    }

    #[cfg(feature = "websocket")]
    {
        let core_clone_web = core.clone();
        handles.push(tokio::spawn(websocket_function(core_clone_web)));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

#[cfg(not(any(feature = "grpc", feature = "websocket")))]
fn main() {
    compile_error!("At least one of 'grpc' or 'websocket' features must be enabled");
}
