use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt,AsyncBufReadExt, AsyncReadExt, self};

// TODO: let client be able to type into terminal to send input
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    env_logger::init();
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    log::info!("created stream");
    // Send data to the server]
    loop {
        
        let stdin = io::stdin();
        let mut reader = io::BufReader::new(stdin);

        let mut input = String::new();
        reader.read_line(&mut input).await?;
        input.pop();



        // let message = "Hello, server!\n";
        if !input.is_empty() {
            stream.write_all(input.as_bytes()).await?;
            stream.write_all(b"##END##").await?;
            log::info!("User input: {}", input);
        } else {
            continue;
        }
        
        // Continue reading the server's response
        let mut buf = Vec::new();
        loop {
            let mut chunk = vec![0; 1024];
            let bytes_read = stream.read(&mut chunk).await.unwrap();
            buf.extend_from_slice(&chunk[..bytes_read]);

            if buf.ends_with(b"##END##") {
                buf.truncate(buf.len() - 7); // Remove the delimiter from the buffer
                break;
            }
        }
        log::info!("Server response: {}", String::from_utf8(buf.clone()).unwrap());
        // tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    }
}
// use tokio::net::TcpStream;
// use tokio::io::{AsyncWriteExt, AsyncReadExt, BufReader, stdin};

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     env_logger::init();
//     let mut write_stream = TcpStream::connect("127.0.0.1:8080").await?;
//     let mut read_stream = TcpStream::connect("127.0.0.1:8080").await?;
//     log::info!("created stream");

//     let user_input_handler = tokio::spawn(async move {
//         loop {
//             let stdin = stdin();
//             let mut reader = BufReader::new(stdin);
    
//             let mut input = Vec::new();
//             let reader_result = reader.read(&mut input).await;
//             match reader_result {
//                 Ok(_) => {},
//                 Err(e) => {
//                     log::error!("error reading from stdin: {}", e);
//                     break;
//                 }
//             }
//                 // Remove the trailing newline character
//             input.pop();
    
//             if !input.is_empty() {
//                 let first_stream_write_result =write_stream.write_all(&input).await; 
//                 match first_stream_write_result {
//                     Ok(_) => {},
//                     Err(e) => {
//                         log::error!("error writing to stream: {}", e);
//                         break;
//                     }
//                 }
//                 let second_stream_write_result = write_stream.write_all(b"##END##").await;
//                 match second_stream_write_result {
//                     Ok(_) => {},
//                     Err(e) => {
//                         log::error!("error writing to stream: {}", e);
//                         break;
//                     }
//                 }
//                 log::info!("sent message");
//             }
//         }
//     });

//     let server_response_handler = tokio::spawn(async move {
//         let mut buf = Vec::new();
//         let mut chunk = vec![0; 1024];
//         loop {
//             let bytes_read = read_stream.read(&mut chunk).await;
//             match bytes_read {
//                 Ok(_) => {},
//                 Err(e) => {
//                     log::error!("error reading from stream: {}", e);
//                     break;
//                 }
//             }
//             let bytes_read_frfr = bytes_read.unwrap();
//             buf.extend_from_slice(&chunk[..bytes_read_frfr]);

//             if buf.ends_with(b"##END##") {
//                 buf.truncate(buf.len() - 7);
//                 log::info!("Server response: {}", String::from_utf8_lossy(&buf));
//                 buf.clear();
//             }
//         }
//     });

//     tokio::try_join!(user_input_handler, server_response_handler)?;

//     Ok(())
// }
