use log::info;
use tokio::select;
use tokio::sync::broadcast;
use tonic::transport::Server;
use crate::backend_server::grpc::goodbye::goodbye_server::GoodbyeServer;
use crate::backend_server::grpc::hello::hello_server::HelloServer;
use crate::backend_server::grpc::server::{GoodbyeService, HelloService};

pub async fn start_grpc_server(stop_sx: broadcast::Sender<bool>) {
    let mut stop_rx = stop_sx.subscribe();
    let addr = format!("0.0.0.0:{}", "9983").parse().unwrap();
    println!("grpc backend_server staring at : {}", addr);
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
                Ok(()) => {
                    println!("grpc backend_server staring at [[[[: {}", addr)
                },
                Err(e) => {
                    panic!("Error: {}", e);
                }
            }
        }
    }

}