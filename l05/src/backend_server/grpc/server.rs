use crate::backend_server::grpc::basic::BaseResponse;
use crate::backend_server::grpc::goodbye::goodbye_server::Goodbye;
use crate::backend_server::grpc::goodbye::{GoodbyeRequest, GoodbyeResponse};
use crate::backend_server::grpc::hello::hello_server::Hello;
use crate::backend_server::grpc::hello::{HelloRequest, HelloResponse};
use std::fmt::format;
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct HelloService {}

#[tonic::async_trait]
impl Hello for HelloService {
    async fn hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        log::info!("hello receive request: {:?}", request);
        println!("hello receive request: {:?}", request);
        let response = HelloResponse {
            data: format!("Hello, {}", request.into_inner().name),
            message: Some(BaseResponse {
                message: "Ok".to_string(),
                code: 200,
            }),
        };
        Ok(Response::new(response))
    }
}

#[derive(Default)]
pub struct GoodbyeService {}

#[tonic::async_trait]
impl Goodbye for GoodbyeService {
    async fn goodbye(
        &self,
        request: Request<GoodbyeRequest>,
    ) -> Result<Response<GoodbyeResponse>, Status> {
        log::info!("goodbye receive request: {:?}", request);
        println!("Goodbye receive request: {:?}", request);
        println!("{}", request.remote_addr().unwrap().to_string());
        let response = GoodbyeResponse {
            data: format!("Goodbye, {}", request.into_inner().name),
            message: Some(BaseResponse {
                message: "Ok".to_string(),
                code: 200,
            }),
        };
        Ok(Response::new(response))
    }
}
