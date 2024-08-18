#[cfg(any(feature = "grpc", feature = "websocket"))]
use tokio;



#[cfg(feature = "grpc")]
mod grpc;


#[cfg(feature = "grpc")]
async fn grpc_function() {
    println!("gRPC running");
    grpc::grpc_main().await.expect("TODO: panic message");
    println!("gRPC stopped");
}

#[cfg(feature = "websocket")]
async fn websocket_function() {
    println!("WebSocket running");
    // TODO:! add ws impl.
}


#[cfg(any(feature = "grpc", feature = "websocket"))]
#[tokio::main]
async fn main() {
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