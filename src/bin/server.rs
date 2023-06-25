use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            loop {
                let mut buf = Vec::new();

                // Read the data until the delimiter is encountered
                loop {
                    let mut chunk = vec![0; 1024];
                    let bytes_read = socket.read(&mut chunk).await.unwrap();
                    buf.extend_from_slice(&chunk[..bytes_read]);

                    if buf.ends_with(b"##END##") {
                        buf.truncate(buf.len() - 7); // Remove the delimiter from the buffer
                        break;
                    }
                }

                log::info!("Received message from {}: {}", socket.peer_addr().unwrap(), String::from_utf8_lossy(&buf));

                // Process the message and send the response
                let response = format!("{}: {}", socket.peer_addr().unwrap(), String::from_utf8_lossy(&buf));
                socket.write_all(response.as_bytes()).await.unwrap();
                socket.write_all(b"##END##").await.unwrap();

                log::info!("Sent response to {}", socket.peer_addr().unwrap());
            }
        });
    }
}   