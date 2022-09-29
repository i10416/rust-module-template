mod gen;
use gen::hello::HelloReply;
use tonic::{transport::Server, Response, Status};

use gen::hello::greeter_server::{Greeter, GreeterServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = GreetService::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}

#[derive(Default)]
struct GreetService {}

#[tonic::async_trait]
impl Greeter for GreetService {
    async fn say_hello(
        &self,
        request: tonic::Request<gen::hello::HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let res = HelloReply {
            message: format! {"Hello! {:?}",request.into_inner().name},
        };
        Ok(Response::new(res))
    }
}
