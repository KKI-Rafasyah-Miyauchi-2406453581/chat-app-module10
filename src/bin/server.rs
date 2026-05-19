use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{Sender, channel};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    
    // Send a welcome message
    ws_stream.send(Message::text("Welcome to chat! Type a message")).await?;
    
    // Subscribe to the broadcast channel
    let mut bcast_rx = bcast_tx.subscribe();

    loop {
        tokio::select! {
            // Task 1: Receive messages from the broadcast channel and send to the client
            Ok(msg) = bcast_rx.recv() => {
                ws_stream.send(Message::text(msg)).await?;
            }
            
            // Task 2: Receive messages from the client and broadcast them to everyone
            Some(Ok(msg)) = ws_stream.next() => {
                if let Some(text) = msg.as_text() {
                    println!("From client {addr:?} {text:?}");
                    // We just broadcast the text as-is for 2.1
                    let _ = bcast_tx.send(text.to_string());
                }
            }
            else => break, // The client disconnected
        }
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);
    let listener = TcpListener::bind("127.0.0.1:2000").await?;
    println!("listening on port 2000");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        
        tokio::spawn(async move {
            // Wrap the raw TCP stream into a websocket.
            match ServerBuilder::new().accept(socket).await {
                Ok(ws_stream) => { // <-- THIS IS THE CHANGED LINE
                    if let Err(e) = handle_connection(addr, ws_stream, bcast_tx).await {
                        eprintln!("Error handling connection from {addr}: {e}");
                    }
                }
                Err(e) => eprintln!("Error accepting websocket connection: {e}"),
            }
        });
    }
}