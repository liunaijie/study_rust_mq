use log::info;
use tokio::select;
use tokio::sync::broadcast;
use tonic::transport::Server;
use protocol::goodbye::goodbye_server::GoodbyeServer;
use protocol::hello::hello_server::HelloServer;
use crate::server::grpc::server::{GoodbyeService, HelloService};

pub async fn start_grpc_server(port:usize, stop_sx: broadcast::Sender<bool>) {
    let mut stop_rx = stop_sx.subscribe();
    let addr = format!("0.0.0.0:{}", port).parse().unwrap();
    info!("grpc backend_server staring at : {}", addr);
    select! {
        val = stop_rx.recv() => {
            match val {
                Ok(_) => {},
                Err(_) => {}
            }
        },
        val = Server::builder()
        .add_service(HelloServer::new(HelloService::default()))
        .add_service(GoodbyeServer::new(GoodbyeService::default()))
        .serve(addr)
        => {
            match val {
                Ok(()) => {},
                Err(e) => {
                    panic!("Error: {}", e);
                }
            }
        }
    }

}