use tokio::io::{self, AsyncBufReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    loop{
        let stdin = io::stdin();
        let mut reader = io::BufReader::new(stdin);

        let mut input = String::new();
        reader.read_line(&mut input).await?;

        println!("User input: {}", input);


    }

}