use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use warp::{ws::Message, Filter};
use serde::{Deserialize, Serialize};
use futures_util::stream::StreamExt;
use tokio_stream::wrappers::UnboundedReceiverStream;

// User session information
type Users = Arc<RwLock<HashMap<String, mpsc::UnboundedSender<Message>>>>;

#[derive(Deserialize, Serialize)]
struct ChatMessage {
    user: String,
    message: String,
    timestamp: u64,
}

#[tokio::main]
async fn main() {
    let users = Users::default();
    
    let chat = warp::path("chat")
        .and(warp::ws())
        .and(warp::path::param())
        .and(with_users(users.clone()))
        .map(|ws: warp::ws::Ws, user_id: String, users| {
            ws.on_upgrade(move |socket| handle_connection(socket, user_id, users))
        });
    
    let health = warp::path!("health").map(|| "OK");
    let routes = health.or(chat);
    
    println!("Chat server starting on ws://127.0.0.1:8080/chat/<user_id>");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}


fn with_users(users: Users) -> impl Filter<Extract = (Users,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || users.clone())
}

async fn handle_connection(ws: warp::ws::WebSocket, user_id: String, users: Users) {
    println!("New user connected: {}", user_id);
    
    let (user_ws_tx, mut user_ws_rx) = ws.split();
    let (tx, rx) = mpsc::unbounded_channel();
    
    let rx_stream = UnboundedReceiverStream::new(rx);
    
    tokio::task::spawn(async move {
        rx_stream
            .map(|msg| Ok(msg))
            .forward(user_ws_tx)
            .await
            .unwrap_or_else(|e| eprintln!("Error forwarding messages: {}", e));
    });
    
    users.write().await.insert(user_id.clone(), tx);
    
    broadcast_message(
        &users,
        ChatMessage {
            user: "System".to_string(),
            message: format!("{} joined the chat", user_id),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        },
    )
    .await;
    
    while let Some(result) = user_ws_rx.next().await {
        match result {
            Ok(msg) => {
                if !msg.is_text() {
                    continue;
                }
                
                let msg = if let Ok(s) = msg.to_str() {
                    s
                } else {
                    continue;
                };
                
                let chat_msg = ChatMessage {
                    user: user_id.clone(),
                    message: msg.to_string(),
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                };
                
                broadcast_message(&users, chat_msg).await;
            }
            Err(e) => {
                eprintln!("Error receiving message from {}: {}", user_id, e);
                break;
            }
        }
    }
    
    user_disconnected(&user_id, &users).await;
}

async fn broadcast_message(users: &Users, msg: ChatMessage) {
    let msg_json = serde_json::to_string(&msg).unwrap();
    
    let users_lock = users.read().await;
    
    for (&_, tx) in users_lock.iter() {
        if let Err(_) = tx.send(Message::text(&msg_json)) {
            continue;
        }
    }
}

async fn user_disconnected(user_id: &str, users: &Users) {
    println!("User disconnected: {}", user_id);
    
    users.write().await.remove(user_id);
    
    broadcast_message(
        users,
        ChatMessage {
            user: "System".to_string(),
            message: format!("{} left the chat", user_id),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        },
    )
    .await;
}