pub mod server;


pub async fn grpc_main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: std::net::SocketAddr = "[::1]:4444".parse()?;
    tracing::info!("Server Listening on:  {}", addr);

    tonic::transport::Server::builder()
        .add_service(
            tonic_reflection::server::Builder::configure()
                .register_encoded_file_descriptor_set(server::FILE_DESCRIPTOR_SET)
                .build()?,
        )
        .add_service(server::LoggerServer::new(server::LogService::default()))
        .serve(addr)
        .await.unwrap();

    Ok(())
}