use futures::stream::SplitSink;
use futures::SinkExt;
use futures_util::StreamExt;
use serde::Deserialize;
use tokio_tungstenite::{accept_async, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;
use crate::vs::route;

/// Represents credentials sent by the client.
#[derive(Debug, Deserialize)]
pub(crate) struct Credentials {
    /// The action associated with the credentials.
    #[serde(rename = "action")]
    pub(crate) action: String,
    /// The username provided in the credentials.
    #[serde(rename = "username")]
    pub(crate) username: String,
    /// The password provided in the credentials.
    #[serde(rename = "password")]
    pub(crate) password: String,
    /// The role ID associated with the credentials.
    #[serde(rename = "roleId")]
    pub(crate) role_id: i8,
}

/// Handles WebSocket connections from clients.
///
/// Accepts a TCP stream representing a WebSocket connection and processes incoming messages.
///
/// # Arguments
///
/// * `stream` - The TCP stream representing the WebSocket connection.
pub(crate) async fn handle_connection(stream: tokio::net::TcpStream) {
    // Accept the websocket handshake
    let ws_stream = accept_async(stream)
        .await
        .expect("Failed to accept WebSocket handshake");

    println!("Client connected");

    // Split the websocket stream into sender and receiver
    let (mut sender, mut receiver) = ws_stream.split();

    // Receive and handle messages from the client
    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            Message::Text(text) => handle_text_message(&mut sender, &text).await,
            Message::Binary(data) => handle_binary_message(&data),
            Message::Ping(_) => handle_ping_message(&mut sender).await,
            Message::Close(_) => {
                println!("Client disconnected");
                return;
            }
            _ => (),
        }
    }
}

/// Handles incoming text messages from clients.
///
/// Parses JSON-formatted text messages representing credentials, routes them to appropriate
/// handlers, and sends back responses.
///
/// # Arguments
///
/// * `sender` - The sender part of the WebSocket stream.
/// * `text` - The text message received from the client.
async fn handle_text_message(sender:&mut SplitSink<WebSocketStream<tokio::net::TcpStream>, Message>, text: &str) {
    println!("Received message from client: {}", text);
    let trimmed_text = text.trim_matches('"');
    match serde_json::from_str::<Credentials>(&trimmed_text) {
        Ok(credentials) => {
            let response = route(credentials);
            sender.send(tokio_tungstenite::tungstenite::Message::Text(response));
        }
        Err(err) => {
            println!("Failed to deserialize JSON data: {}", err);
            sender.send(tokio_tungstenite::tungstenite::Message::Text(err.to_string())).await;
        }
    }
}

/// Handles incoming binary messages from clients.
///
/// # Arguments
///
/// * `data` - The binary data received from the client.
fn handle_binary_message(data: &[u8]) {
    println!("Received binary data: {:?}", data);
    // Handle binary data if necessary
}

/// Handles incoming ping messages from clients.
///
/// Responds to ping messages with pong messages to keep the connection alive.
///
/// # Arguments
///
/// * `sender` - The sender part of the WebSocket stream.
async fn handle_ping_message(sender: &mut SplitSink<WebSocketStream<tokio::net::TcpStream>, Message>) {
    // Respond to ping messages to keep the connection alive
    if let Err(err) = sender.send(Message::Pong(vec![])).await {
        println!("Failed to send message: {}", err);
    }
}
