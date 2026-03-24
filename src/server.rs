use crate::core::IvyApp;
use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State, Query},
    response::{Html, IntoResponse, Response},
    routing::{get, post, patch},
    Router, Json, http::StatusCode,
};
use serde::Serialize;
use tower_http::cors::{CorsLayer, Any};
use tower_http::services::ServeDir;
use std::sync::Arc;
use std::collections::HashMap;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(non_snake_case)]
struct NegotiateResponse {
    connectionId: String,
    connectionToken: String,
    negotiateVersion: i32,
    availableTransports: Vec<TransportInfo>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(non_snake_case)]
struct TransportInfo {
    transport: String,
    transferFormats: Vec<String>,
}

pub trait AppProvider: Send + Sync {
    fn get_app(&self, app_id: &str) -> Arc<dyn IvyApp>;
}

pub async fn start_server(app_instance: Arc<dyn IvyApp>) {
    struct SingleApp(Arc<dyn IvyApp>);
    impl AppProvider for SingleApp {
        fn get_app(&self, _: &str) -> Arc<dyn IvyApp> { self.0.clone() }
    }
    start_server_with_provider(Arc::new(SingleApp(app_instance))).await;
}

pub async fn start_server_with_provider(provider: Arc<dyn AppProvider>) {
    let dist_path = std::env::current_dir().unwrap().join("frontend/dist");
    
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(index))
        .route("/ivy/messages", get(ws_handler))
        .route("/ivy/messages/negotiate", post(negotiate))
        .route("/ivy/auth/set-auth-cookies", patch(dummy_ok))
        .route("/ivy/auth/refresh-session", post(dummy_ok))
        .fallback_service(ServeDir::new(&dist_path))
        .with_state(provider)
        .layer(cors);

    println!("--- RUSTY NATIVE SERVER ---");
    println!("Serving Ivy frontend on http://localhost:5010");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5010").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> impl IntoResponse {
    let dist_path = std::env::current_dir().unwrap().join("frontend/dist");
    let index_path = dist_path.join("index.html");
    match tokio::fs::read_to_string(index_path).await {
        Ok(content) => Html(content).into_response(),
        Err(_) => "Frontend not found. Please build the frontend first.".into_response(),
    }
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<HashMap<String, String>>,
    State(provider): State<Arc<dyn AppProvider>>,
) -> Response {
    let app_id = params.get("appId").map(|s| s.as_str()).unwrap_or("HelloApp");
    let app = provider.get_app(app_id);
    println!("Websocket upgrade request received for appId: {}", app_id);
    ws.on_upgrade(move |socket| handle_socket(socket, app))
}

async fn handle_socket(mut socket: WebSocket, app: Arc<dyn IvyApp>) {
    // Handshake
    if let Some(Ok(Message::Text(text))) = socket.recv().await {
        if text.contains("protocol") {
            let _ = socket.send(Message::Text("{}\u{1e}".to_string())).await;
        }
    }

    // First refresh
    send_refresh(&mut socket, &app).await;

    // Message loop
    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(text) = msg {
            // println!("MSG: {}", text);
            let clean_text = text.trim_end_matches('\u{1e}');
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(clean_text) {
                if val["type"] == 6 { // Ping
                    let _ = socket.send(Message::Text("{\"type\":6}\u{1e}".to_string())).await;
                } else if val["type"] == 1 { // Invocation
                    let target = val["target"].as_str().unwrap_or("");
                    if target == "Event" {
                        if let Some(args) = val["arguments"].as_array() {
                            if args.len() >= 3 {
                                let event_name = args[0].as_str().unwrap_or("");
                                let id = args[1].as_str().unwrap_or("");
                                if let Some(event_args) = args[2].as_array() {
                                    if event_name == "OnChange" {
                                        let new_val = match &event_args[0] {
                                            serde_json::Value::String(s) => s.clone(),
                                            serde_json::Value::Bool(b) => b.to_string(),
                                            serde_json::Value::Number(n) => n.to_string(),
                                            _ => "".to_string(),
                                        };
                                        println!("ON_CHANGE: id={}, val={}", id, new_val);
                                        app.update_state(id, &new_val);
                                        send_refresh(&mut socket, &app).await;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn send_refresh(socket: &mut WebSocket, app: &Arc<dyn IvyApp>) {
    let tree = app.build();
    let msg = serde_json::json!({
        "type": 1,
        "target": "Refresh",
        "arguments": [
            { "widgets": tree.serialize() }
        ]
    });
    let _ = socket.send(Message::Text(msg.to_string() + "\u{1e}")).await;
}

async fn dummy_ok() -> impl IntoResponse {
    StatusCode::OK
}

async fn negotiate() -> impl IntoResponse {
    // println!("Negotiate request received");
    let id = uuid::Uuid::new_v4().to_string();
    let resp = NegotiateResponse {
        connectionId: id.clone(),
        connectionToken: id,
        negotiateVersion: 1,
        availableTransports: vec![
            TransportInfo {
                transport: "WebSockets".to_string(),
                transferFormats: vec!["Text".to_string()],
            }
        ],
    };
    Json(resp)
}
