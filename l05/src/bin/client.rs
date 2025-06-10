use tonic::Request;
use tonic::transport::Endpoint;
use l05_1::backend_server::grpc::goodbye::goodbye_client::GoodbyeClient;
use l05_1::backend_server::grpc::goodbye::GoodbyeRequest;
use l05_1::backend_server::grpc::hello::hello_client::HelloClient;
use l05_1::backend_server::grpc::hello::HelloRequest;

#[tokio::main]
async fn main() {
    let addr = Endpoint::from_static("https://127.0.0.1:9983");

    let mut hello_cli = HelloClient::connect(addr.clone()).await.unwrap();
    let request = Request::new(HelloRequest {
        name: "tom".to_string(),
    });
    let response = hello_cli.hello(request).await.unwrap();
    println!("hello response: {:?}", response.into_inner());

    let mut goodbye_cli = GoodbyeClient::connect(addr).await.unwrap();
    let request = Request::new(GoodbyeRequest {
        name: "jerry".to_string(),
    });
    let response = goodbye_cli.goodbye(request).await.unwrap();
    println!("goodbye response: {:?}", response.into_inner());

}