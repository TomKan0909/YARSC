use tokio::io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let tcp_stream = TcpStream::connect("127.0.0.1:8080").await?;
    let socket_addr = tcp_stream.peer_addr()?;

    let (mut rd_stream, mut wrt_stream) = tcp_stream.into_split();
    log::info!("created stream");

    let user_input_handler = tokio::spawn(async move {
        loop {
            let stdin = io::stdin();
            let mut reader = io::BufReader::new(stdin);

            let mut input = String::new();
            reader.read_line(&mut input).await.unwrap();
            input.pop();

            if !input.is_empty() {
                let first_stream_write_result = wrt_stream.write_all(input.as_bytes()).await;
                match first_stream_write_result {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("error writing to stream: {}", e);
                        break;
                    }
                }
                let second_stream_write_result = wrt_stream.write_all(b"##END##").await;
                match second_stream_write_result {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("error writing to stream: {}", e);
                        break;
                    }
                }
                log::info!("{}: {}", socket_addr.to_string(), input);
            }
        }
    });

    let server_response_handler = tokio::spawn(async move {
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
            buf.extend_from_slice(&chunk[..bytes_read_frfr]);

            if buf.ends_with(b"##END##") {
                buf.truncate(buf.len() - 7);
                log::info!("{}", String::from_utf8_lossy(&buf));
                buf.clear();
            }
        }
    });

    tokio::try_join!(user_input_handler, server_response_handler)?;
    // tcp_stream.shutdown().await?;
    Ok(())
}
