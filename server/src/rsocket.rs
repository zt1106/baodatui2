use rsocket_rust::prelude::{Payload, RSocket, RSocketFactory};

use rsocket_rust::Result;
use rsocket_rust_transport_websocket::WebsocketServerTransport;
pub async fn start_rsocket_server(port: u16) -> Result<()> {
    RSocketFactory::receive()
        .transport(WebsocketServerTransport::from("ws:://127.0.0.1:7979"))
        .acceptor(Box::new(|setup, _sending_socket| todo!()))
        .serve()
        .await
}

pub struct MyRSocket;

impl RSocket for MyRSocket {
    fn metadata_push<'life0, 'async_trait>(
        &'life0 self,
        req: Payload,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<()>> + core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    fn fire_and_forget<'life0, 'async_trait>(
        &'life0 self,
        req: Payload,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = Result<()>> + core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    fn request_response<'life0, 'async_trait>(
        &'life0 self,
        req: Payload,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<Option<Payload>>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    fn request_stream(&self, req: Payload) -> rsocket_rust::prelude::Flux<Result<Payload>> {
        todo!()
    }

    fn request_channel(
        &self,
        reqs: rsocket_rust::prelude::Flux<Result<Payload>>,
    ) -> rsocket_rust::prelude::Flux<Result<Payload>> {
        todo!()
    }
}
