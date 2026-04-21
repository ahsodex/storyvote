use crate::messages::{ClientEvent, ServerEvent};
use crate::state::SharedState;
use axum::extract::ws::{Message, WebSocket};
use std::sync::Arc;
use tracing::warn;

pub async fn handle_socket(mut socket: WebSocket, shared: Arc<SharedState>, name: String) {
    let (session_id, is_host) = match shared.add_participant(&name).await {
        Ok(result) => result,
        Err(message) => {
            send_event(&mut socket, &ServerEvent::Error { message }).await;
            let _ = socket.close().await;
            return;
        }
    };

    let participant_name = shared
        .participant_name(&session_id)
        .await
        .unwrap_or_else(|| name.clone());

    send_event(
        &mut socket,
        &ServerEvent::Connected {
            session_id: session_id.clone(),
            is_host,
            name: participant_name,
        },
    )
    .await;

    let snapshot = shared.snapshot().await;
    send_event(&mut socket, &snapshot).await;

    let mut subscription = shared.subscribe();

    loop {
        tokio::select! {
            recv_result = socket.recv() => {
                match recv_result {
                    Some(Ok(Message::Text(text))) => {
                        if let Err(message) = handle_client_event(&shared, &session_id, &text).await {
                            if !send_event(&mut socket, &ServerEvent::Error { message }).await {
                                break;
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => {
                        break;
                    }
                    Some(Ok(Message::Ping(payload))) => {
                        if socket.send(Message::Pong(payload)).await.is_err() {
                            break;
                        }
                    }
                    Some(Ok(_)) => {}
                    Some(Err(error)) => {
                        warn!("websocket receive error: {error}");
                        break;
                    }
                }
            }
            broadcast_result = subscription.recv() => {
                match broadcast_result {
                    Ok(event) => {
                        if !send_event(&mut socket, &event).await {
                            break;
                        }
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
        }
    }

    shared.remove_participant(&session_id).await;
}

async fn handle_client_event(
    shared: &Arc<SharedState>,
    session_id: &str,
    text: &str,
) -> Result<(), String> {
    let parsed = serde_json::from_str::<ClientEvent>(text);
    let event = match parsed {
        Ok(event) => event,
        Err(_) => {
            return Err("Invalid message format.".to_string());
        }
    };

    match event {
        ClientEvent::Vote { value } => shared.set_vote(session_id, &value).await,
        ClientEvent::SetTopic { value } => shared.set_topic(session_id, &value).await,
        ClientEvent::Reveal => shared.reveal(session_id).await,
        ClientEvent::Reset => shared.reset(session_id).await,
    }
}

async fn send_event(socket: &mut WebSocket, event: &ServerEvent) -> bool {
    let payload = match serde_json::to_string(event) {
        Ok(payload) => payload,
        Err(_) => return false,
    };

    socket.send(Message::Text(payload)).await.is_ok()
}

#[cfg(test)]
mod tests {
    use crate::http;
    use crate::messages::{ClientEvent, ServerEvent};
    use crate::state::SharedState;
    use futures_util::{SinkExt, StreamExt};
    use std::sync::Arc;
    use tokio::net::TcpListener;
    use tokio::task::JoinHandle;
    use tokio_tungstenite::{connect_async, tungstenite::protocol::Message as ClientMessage};

    #[tokio::test]
    async fn websocket_join_vote_reveal_reset_flow_works() {
        let (base_url, server_handle) = spawn_test_server().await;
        let (mut host_socket, _, host_state) = connect_client(&base_url, "Host").await;
        let (mut guest_socket, _, guest_state) = connect_client(&base_url, "Guest").await;

        assert!(!host_state.revealed);
        assert_eq!(host_state.participants.len(), 1);
        assert!(host_state.participants.iter().any(|participant| participant.is_host));

        assert_eq!(guest_state.participants.len(), 2);
    let host_join_sync = next_state(&mut host_socket).await;
    assert_eq!(host_join_sync.participants.len(), 2);

        send_client_event(&mut host_socket, &ClientEvent::Vote { value: "3".to_string() }).await;
        let host_vote_state = next_state(&mut host_socket).await;
        let guest_vote_state = next_state(&mut guest_socket).await;
        assert!(host_vote_state.participants.iter().any(|participant| participant.name == "Host" && participant.voted));
        assert!(guest_vote_state.participants.iter().any(|participant| participant.name == "Host" && participant.voted));
        assert!(host_vote_state.votes.is_empty());

        send_client_event(&mut guest_socket, &ClientEvent::Vote { value: "5".to_string() }).await;
        let _ = next_state(&mut host_socket).await;
        let _ = next_state(&mut guest_socket).await;

        send_client_event(&mut host_socket, &ClientEvent::Reveal).await;
        let revealed_for_host = next_state(&mut host_socket).await;
        let revealed_for_guest = next_state(&mut guest_socket).await;
        assert!(revealed_for_host.revealed);
        assert_eq!(revealed_for_host.votes.get("Host").map(String::as_str), Some("3"));
        assert_eq!(revealed_for_host.votes.get("Guest").map(String::as_str), Some("5"));
        assert_eq!(revealed_for_guest.votes.get("Host").map(String::as_str), Some("3"));

        send_client_event(&mut host_socket, &ClientEvent::Reset).await;
        let reset_for_host = next_state(&mut host_socket).await;
        let reset_for_guest = next_state(&mut guest_socket).await;
        assert!(!reset_for_host.revealed);
        assert!(reset_for_host.votes.is_empty());
        assert!(!reset_for_guest.revealed);

        host_socket.close(None).await.unwrap();
        guest_socket.close(None).await.unwrap();
        server_handle.abort();
    }

    #[tokio::test]
    async fn duplicate_name_connection_returns_error() {
        let (base_url, server_handle) = spawn_test_server().await;
        let (mut first_socket, _, _) = connect_client(&base_url, "Host").await;

        let duplicate_url = ws_url(&base_url, "Host");
        let (mut duplicate_socket, _) = connect_async(duplicate_url).await.unwrap();
        let duplicate_error = next_message(&mut duplicate_socket).await;

        match duplicate_error {
            ServerEvent::Error { message } => {
                assert_eq!(message, "That name is already in use for this session.");
            }
            other => panic!("expected error event, got {other:?}"),
        }

        first_socket.close(None).await.unwrap();
        let _ = duplicate_socket.close(None).await;
        server_handle.abort();
    }

    #[tokio::test]
    async fn non_host_reveal_returns_error_without_broadcasting_state_change() {
        let (base_url, server_handle) = spawn_test_server().await;
        let (mut host_socket, _, _) = connect_client(&base_url, "Host").await;
        let (mut guest_socket, _, _) = connect_client(&base_url, "Guest").await;
        let _ = next_state(&mut host_socket).await;

        send_client_event(&mut guest_socket, &ClientEvent::Reveal).await;
        let error = next_message(&mut guest_socket).await;
        match error {
            ServerEvent::Error { message } => {
                assert_eq!(message, "Only the host can reveal votes.");
            }
            other => panic!("expected error event, got {other:?}"),
        }

        send_client_event(&mut host_socket, &ClientEvent::Vote { value: "8".to_string() }).await;
        let host_state = next_state(&mut host_socket).await;
        let guest_state = next_state(&mut guest_socket).await;
        assert!(!host_state.revealed);
        assert!(!guest_state.revealed);

        host_socket.close(None).await.unwrap();
        guest_socket.close(None).await.unwrap();
        server_handle.abort();
    }

    #[tokio::test]
    async fn non_host_reset_returns_error() {
        let (base_url, server_handle) = spawn_test_server().await;
        let (mut host_socket, _, _) = connect_client(&base_url, "Host").await;
        let (mut guest_socket, _, _) = connect_client(&base_url, "Guest").await;
        let _ = next_state(&mut host_socket).await;

        send_client_event(&mut guest_socket, &ClientEvent::Reset).await;
        let error = next_message(&mut guest_socket).await;
        match error {
            ServerEvent::Error { message } => {
                assert_eq!(message, "Only the host can reset the round.");
            }
            other => panic!("expected error event, got {other:?}"),
        }

        host_socket.close(None).await.unwrap();
        guest_socket.close(None).await.unwrap();
        server_handle.abort();
    }

    #[tokio::test]
    async fn host_reassignment_is_broadcast_after_host_disconnect() {
        let (base_url, server_handle) = spawn_test_server().await;
        let (mut host_socket, _, _) = connect_client(&base_url, "Host").await;
        let (mut guest_socket, _, guest_state) = connect_client(&base_url, "Guest").await;

        assert_eq!(guest_state.participants.len(), 2);
        let _ = next_state(&mut host_socket).await;

        host_socket.close(None).await.unwrap();

        let reassigned_state = next_state(&mut guest_socket).await;
        assert_eq!(reassigned_state.participants.len(), 1);
        assert_eq!(reassigned_state.participants[0].name, "Guest");
        assert!(reassigned_state.participants[0].is_host);

        send_client_event(&mut guest_socket, &ClientEvent::Reveal).await;
        let revealed_state = next_state(&mut guest_socket).await;
        assert!(revealed_state.revealed);

        guest_socket.close(None).await.unwrap();
        server_handle.abort();
    }

    #[tokio::test]
    async fn name_can_rejoin_after_disconnect() {
        let (base_url, server_handle) = spawn_test_server().await;
        let (mut first_socket, connected, _) = connect_client(&base_url, "Alex").await;
        let first_session_id = match connected {
            ServerEvent::Connected { session_id, .. } => session_id,
            other => panic!("expected connected event, got {other:?}"),
        };

        first_socket.close(None).await.unwrap();

        let (mut second_socket, connected_again, state_again) = connect_client(&base_url, "Alex").await;
        let second_session_id = match connected_again {
            ServerEvent::Connected {
                session_id,
                is_host,
                name,
            } => {
                assert!(is_host);
                assert_eq!(name, "Alex");
                session_id
            }
            other => panic!("expected connected event, got {other:?}"),
        };

        assert_ne!(first_session_id, second_session_id);
        assert_eq!(state_again.participants.len(), 1);
        assert_eq!(state_again.participants[0].name, "Alex");

        second_socket.close(None).await.unwrap();
        server_handle.abort();
    }

    async fn spawn_test_server() -> (String, JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let app = http::router(Arc::new(SharedState::new()), false);
        let handle = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        (format!("http://{}", addr), handle)
    }

    async fn connect_client(
        base_url: &str,
        name: &str,
    ) -> (
        tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        ServerEvent,
        StateEvent,
    ) {
        let url = ws_url(base_url, name);
        let (mut socket, _) = connect_async(url).await.unwrap();
        let connected = next_message(&mut socket).await;
        let state = next_state(&mut socket).await;
        (socket, connected, state)
    }

    async fn send_client_event(
        socket: &mut tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        event: &ClientEvent,
    ) {
        let payload = serde_json::to_string(event).unwrap();
        socket.send(ClientMessage::Text(payload)).await.unwrap();
    }

    async fn next_state(
        socket: &mut tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    ) -> StateEvent {
        match next_message(socket).await {
            ServerEvent::State {
                participants,
                revealed,
                votes,
                topic: _,
                host_session_id: _,
            } => StateEvent {
                participants,
                revealed,
                votes,
            },
            other => panic!("expected state event, got {other:?}"),
        }
    }

    async fn next_message(
        socket: &mut tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    ) -> ServerEvent {
        let frame = socket.next().await.unwrap().unwrap();
        match frame {
            ClientMessage::Text(text) => serde_json::from_str(&text).unwrap(),
            other => panic!("expected text frame, got {other:?}"),
        }
    }

    fn ws_url(base_url: &str, name: &str) -> String {
        let host = base_url.strip_prefix("http://").unwrap_or(base_url);
        format!("ws://{host}/ws?name={name}")
    }

    #[derive(Debug)]
    struct StateEvent {
        participants: Vec<crate::messages::ParticipantView>,
        revealed: bool,
        votes: std::collections::HashMap<String, String>,
    }
}
