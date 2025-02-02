use hyper::{Client, Request, Body};
use hyper_tls::HttpsConnector;
use tokio::net::UdpSocket;
use std::net::SocketAddr;

const DOH_SERVER: &str = "https://cloudflare-dns.com/dns-query";
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:53").await?;
    let mut buf = [0; 512];
    loop {
        let (size, addr) = socket.recv_from(&mut buf).await?;
        println!("Received DNS query from {}", addr);
        let dns_query = &buf[..size];
        let response = send_doh_request(dns_query).await.unwrap_or_else(|_| vec![]);
        socket.send_to(&response, addr).await?;
    }
}

async fn send_doh_request(dns_query: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let req = Request::post(DOH_SERVER)
        .header("Content-Type", "application/dns-message")
        .body(Body::from(dns_query.to_vec()))?;
    let resp = client.request(req).await?;
    let bytes = hyper::body::to_bytes(resp.into_body()).await?;

    Ok(bytes.to_vec())
}
