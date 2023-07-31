use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::{mpsc, Mutex};

// #[tokio::main]
/*
    Think current issue is that. I'm trying to creat a read handler and write handler per client.
    It should be one read handler and one write handler for all clients. I.e one process for handling reads
    and one process for handling writes
*/

struct Client {
    sender: mpsc::Sender<String>,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let clients = Arc::new(Mutex::new(HashMap::<String, Client>::new())); // HashMap to keep track of clients

    loop {
        log::info!("listening for a new connection");
        let (tcp_stream, socket_addr) = listener.accept().await?;
        let (mut rd_stream, mut wrt_stream) = tcp_stream.into_split();
        let (tx, mut rx) = mpsc::channel::<String>(100); // Channel to send messages to write handlers

        let clients_clone = clients.clone();
        clients_clone.lock().await.insert(
            socket_addr.to_string().clone(),
            Client { sender: tx.clone() },
        );

        let read_handler = tokio::spawn(async move {
            loop {
                let mut buf = Vec::new();
                let mut chunk = vec![0; 1024];
                loop {
                    let bytes_read = rd_stream.read(&mut chunk).await;
                    match bytes_read {
                        Ok(_) => {}
                        Err(e) => {
                            log::error!("error reading from stream: {}", e);
                            break;
                        }
                    }
                    let bytes_read_frfr = bytes_read.unwrap();
                    log::info!("bytes read {}", bytes_read_frfr);
                    buf.extend_from_slice(&chunk[..bytes_read_frfr]);

                    if buf.ends_with(b"##END##") {
                        buf.truncate(buf.len() - 7);
                        break;
                    }
                }
                let message = String::from_utf8_lossy(&buf).to_string();
                log::info!("buffer length {}", buf.len());
                // log::info!("Received message from {}: {}", socket_addr, message);
                // tx.send(message).await.unwrap();

                for (key, value) in clients_clone.lock().await.iter() {
                    value
                        .sender
                        .send(socket_addr.to_string() + ": " + &message.clone())
                        .await
                        .unwrap();
                }
                buf.clear();
            }
            // Unlock clients and loop over hashmap to send message to all clients
        });
        let write_handler = tokio::spawn(async move {
            loop {
                while let Some(message) = rx.recv().await {
                    log::info!("Received message {}", message);
                    wrt_stream.write_all(message.as_bytes()).await.unwrap();
                    wrt_stream.write_all(b"##END##").await.unwrap();
                    log::info!("All good");
                }
            }
        });
    }
    Ok(())
}
