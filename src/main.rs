mod core;
#[cfg(feature = "grpc")]
mod grpc;

#[cfg(feature = "grpc")]
async fn grpc_function() {
    tracing::info!("gRPC running");
    grpc::grpc_main().await.expect("TODO: panic message");
    tracing::info!("gRPC stopped");
}

#[cfg(feature = "websocket")]
async fn websocket_function() {
    tracing::info!("WebSocket running");
    // TODO:! add ws impl.
}

#[cfg(any(feature = "grpc", feature = "websocket"))]
#[tokio::main]
async fn main() {
    use logging_lib::init_tracing;

    init_tracing!("LOGGING_SERVICE_LOG");

    let mut handles = vec![];

    #[cfg(feature = "grpc")]
    handles.push(tokio::spawn(grpc_function()));

    #[cfg(feature = "websocket")]
    handles.push(tokio::spawn(websocket_function()));

    for handle in handles {
        handle.await.unwrap();
    }
}

#[cfg(not(any(feature = "grpc", feature = "websocket")))]
fn main() {
    compile_error!("At least one of 'grpc' or 'websocket' features must be enabled");
}
