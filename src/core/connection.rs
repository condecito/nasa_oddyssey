/*
use std::str::Bytes;
use app_properties::AppProperties;
use http_body_util::Empty;
use hyper::{Request, Uri};
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;

pub async fn setup_connection(url:String){
    let url = url.parse::<hyper::Uri>()?;
    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(80);
    let address = format!("{}:{}", host, port);

    let stream = TcpStream::connect(address).await?;


    let io = TokioIo::new(stream);


    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;


    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

}
pub async fn send_request(url:Uri){
    let authority = url.authority().unwrap().clone();

    let req = Request::builder()
        .uri(url)
        .header(hyper::header::HOST, authority.as_str())
        .body(Empty::<Bytes>::new())?;


    let mut res = sender.send_request(req).await?;

    println!("Response status: {}", res.status());
}*/