use futures_util::SinkExt;
use futures_util::stream::StreamExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:8080")) // <-- Changed to 8080
            .connect()
            .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    loop {
        tokio::select! {
            // Task 1: Read user input from terminal and send it to the server
            Ok(Some(line)) = stdin.next_line() => {
                ws_stream.send(Message::text(line)).await?;
            }
            
            // Task 2: Receive messages from the server and print them
            Some(Ok(msg)) = ws_stream.next() => {
                if let Some(text) = msg.as_text() {
                    println!("From server: {}", text);
                }
            }
            else => break, // Server disconnected
        }
    }
    
    Ok(())
}