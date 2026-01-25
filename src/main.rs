use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, _rx) = broadcast::channel::<String>(100);

    let app = Router::new()
        .route("/ws", get(move |ws| websocket_handler(ws, tx.clone())));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("[status] running -> http://127.0.0.1:3000");
    
    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    tx: broadcast::Sender<String>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, tx))
}

async fn handle_socket(socket: WebSocket, tx: broadcast::Sender<String>) {
    let (mut sender, mut receiver) = socket.split();

    let mut rx = tx.subscribe();
    
    println!("[status] client connected");

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            println!("received: {}", text);
            
            let _ = tx.send(format!("[message] {}", text));
        }
    });

    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    };

    println!("[status] client disconnected");
}
