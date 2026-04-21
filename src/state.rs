use crate::messages::{ParticipantView, ServerEvent};
use std::collections::HashMap;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

const ALLOWED_CARDS: [&str; 11] = ["0", "1", "2", "3", "5", "8", "13", "21", "34", "55", "?"];

#[derive(Debug, Clone)]
struct Participant {
    name: String,
}

#[derive(Debug)]
struct PlanningState {
    participants: HashMap<String, Participant>,
    votes: HashMap<String, String>,
    revealed: bool,
    topic: String,
    host_session_id: Option<String>,
}

#[derive(Debug)]
pub struct SharedState {
    inner: RwLock<PlanningState>,
    broadcaster: broadcast::Sender<ServerEvent>,
}

impl SharedState {
    pub fn new() -> Self {
        let (broadcaster, _) = broadcast::channel(256);
        Self {
            inner: RwLock::new(PlanningState {
                participants: HashMap::new(),
                votes: HashMap::new(),
                revealed: false,
                topic: String::new(),
                host_session_id: None,
            }),
            broadcaster,
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ServerEvent> {
        self.broadcaster.subscribe()
    }

    pub async fn add_participant(&self, name: &str) -> Result<(String, bool), String> {
        let normalized = normalize_name(name)?;
        let session_id = Uuid::new_v4().to_string();

        {
            let mut state = self.inner.write().await;
            let duplicate = state
                .participants
                .values()
                .any(|participant| participant.name.eq_ignore_ascii_case(&normalized));
            if duplicate {
                return Err("That name is already in use for this session.".to_string());
            }

            state
                .participants
                .insert(session_id.clone(), Participant { name: normalized.clone() });

            let is_host = if state.host_session_id.is_none() {
                state.host_session_id = Some(session_id.clone());
                true
            } else {
                false
            };

            drop(state);
            self.broadcast_snapshot().await;
            return Ok((session_id, is_host));
        }
    }

    pub async fn remove_participant(&self, session_id: &str) {
        let mut should_broadcast = false;
        {
            let mut state = self.inner.write().await;
            if state.participants.remove(session_id).is_some() {
                state.votes.remove(session_id);
                should_broadcast = true;

                if state.host_session_id.as_deref() == Some(session_id) {
                    state.host_session_id = state.participants.keys().next().cloned();
                }
            }
        }

        if should_broadcast {
            self.broadcast_snapshot().await;
        }
    }

    pub async fn set_vote(&self, session_id: &str, value: &str) -> Result<(), String> {
        if !ALLOWED_CARDS.contains(&value) {
            return Err("Invalid card value.".to_string());
        }

        {
            let mut state = self.inner.write().await;
            if !state.participants.contains_key(session_id) {
                return Err("Participant not found.".to_string());
            }
            state.votes.insert(session_id.to_string(), value.to_string());
        }

        self.broadcast_snapshot().await;
        Ok(())
    }

    pub async fn reveal(&self, session_id: &str) -> Result<(), String> {
        {
            let mut state = self.inner.write().await;
            if state.host_session_id.as_deref() != Some(session_id) {
                return Err("Only the host can reveal votes.".to_string());
            }
            state.revealed = true;
        }

        self.broadcast_snapshot().await;
        Ok(())
    }

    pub async fn set_topic(&self, session_id: &str, value: &str) -> Result<(), String> {
        let normalized = normalize_topic(value)?;

        {
            let mut state = self.inner.write().await;
            if state.host_session_id.as_deref() != Some(session_id) {
                return Err("Only the host can set the topic.".to_string());
            }
            state.topic = normalized;
        }

        self.broadcast_snapshot().await;
        Ok(())
    }

    pub async fn reset(&self, session_id: &str) -> Result<(), String> {
        {
            let mut state = self.inner.write().await;
            if state.host_session_id.as_deref() != Some(session_id) {
                return Err("Only the host can reset the round.".to_string());
            }
            state.votes.clear();
            state.revealed = false;
        }

        self.broadcast_snapshot().await;
        Ok(())
    }

    pub async fn participant_name(&self, session_id: &str) -> Option<String> {
        let state = self.inner.read().await;
        state
            .participants
            .get(session_id)
            .map(|participant| participant.name.clone())
    }

    pub async fn snapshot(&self) -> ServerEvent {
        let state = self.inner.read().await;

        let participants = state
            .participants
            .iter()
            .map(|(session_id, participant)| ParticipantView {
                name: participant.name.clone(),
                voted: state.votes.contains_key(session_id),
                is_host: state.host_session_id.as_deref() == Some(session_id),
            })
            .collect::<Vec<_>>();

        let votes = if state.revealed {
            state
                .votes
                .iter()
                .filter_map(|(session_id, value)| {
                    state
                        .participants
                        .get(session_id)
                        .map(|participant| (participant.name.clone(), value.clone()))
                })
                .collect::<HashMap<_, _>>()
        } else {
            HashMap::new()
        };

        ServerEvent::State {
            participants,
            revealed: state.revealed,
            votes,
            topic: state.topic.clone(),
            host_session_id: state.host_session_id.clone(),
        }
    }

    pub async fn broadcast_snapshot(&self) {
        let snapshot = self.snapshot().await;
        let _ = self.broadcaster.send(snapshot);
    }
}

fn normalize_name(name: &str) -> Result<String, String> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err("Name is required.".to_string());
    }
    if trimmed.len() > 32 {
        return Err("Name must be 32 characters or less.".to_string());
    }
    Ok(trimmed.to_string())
}

fn normalize_topic(value: &str) -> Result<String, String> {
    let trimmed = value.trim();
    if trimmed.len() > 120 {
        return Err("Topic must be 120 characters or less.".to_string());
    }
    Ok(trimmed.to_string())
}

#[cfg(test)]
mod tests {
    use super::SharedState;
    use crate::messages::ServerEvent;

    #[tokio::test]
    async fn first_participant_becomes_host() {
        let state = SharedState::new();

        let (session_id, is_host) = state.add_participant("Alice").await.unwrap();

        assert!(is_host);
        assert_eq!(state.participant_name(&session_id).await.as_deref(), Some("Alice"));
    }

    #[tokio::test]
    async fn duplicate_names_are_rejected_case_insensitively() {
        let state = SharedState::new();
        state.add_participant("Alice").await.unwrap();

        let error = state.add_participant("alice").await.unwrap_err();

        assert_eq!(error, "That name is already in use for this session.");
    }

    #[tokio::test]
    async fn votes_are_hidden_until_reveal_and_exposed_after() {
        let state = SharedState::new();
        let (alice_id, _) = state.add_participant("Alice").await.unwrap();
        let (bob_id, _) = state.add_participant("Bob").await.unwrap();

        state.set_vote(&alice_id, "5").await.unwrap();
        state.set_vote(&bob_id, "8").await.unwrap();

        let hidden_snapshot = state.snapshot().await;
        match hidden_snapshot {
            ServerEvent::State { revealed, votes, .. } => {
                assert!(!revealed);
                assert!(votes.is_empty());
            }
            _ => panic!("expected state snapshot"),
        }

        state.reveal(&alice_id).await.unwrap();

        let revealed_snapshot = state.snapshot().await;
        match revealed_snapshot {
            ServerEvent::State { revealed, votes, .. } => {
                assert!(revealed);
                assert_eq!(votes.get("Alice").map(String::as_str), Some("5"));
                assert_eq!(votes.get("Bob").map(String::as_str), Some("8"));
            }
            _ => panic!("expected state snapshot"),
        }
    }

    #[tokio::test]
    async fn only_host_can_reveal_or_reset() {
        let state = SharedState::new();
        let (host_id, _) = state.add_participant("Host").await.unwrap();
        let (guest_id, _) = state.add_participant("Guest").await.unwrap();

        let reveal_error = state.reveal(&guest_id).await.unwrap_err();
        let reset_error = state.reset(&guest_id).await.unwrap_err();

        assert_eq!(reveal_error, "Only the host can reveal votes.");
        assert_eq!(reset_error, "Only the host can reset the round.");

        state.set_vote(&host_id, "3").await.unwrap();
        state.reveal(&host_id).await.unwrap();
        state.reset(&host_id).await.unwrap();

        let snapshot = state.snapshot().await;
        match snapshot {
            ServerEvent::State { revealed, votes, .. } => {
                assert!(!revealed);
                assert!(votes.is_empty());
            }
            _ => panic!("expected state snapshot"),
        }
    }

    #[tokio::test]
    async fn host_reassigns_when_current_host_leaves() {
        let state = SharedState::new();
        let (host_id, _) = state.add_participant("Alice").await.unwrap();
        let (_, _) = state.add_participant("Bob").await.unwrap();

        state.remove_participant(&host_id).await;

        let snapshot = state.snapshot().await;
        match snapshot {
            ServerEvent::State { participants, .. } => {
                assert_eq!(participants.len(), 1);
                assert_eq!(participants[0].name, "Bob");
                assert!(participants[0].is_host);
            }
            _ => panic!("expected state snapshot"),
        }
    }

    #[tokio::test]
    async fn only_host_can_set_topic() {
        let state = SharedState::new();
        let (host_id, _) = state.add_participant("Host").await.unwrap();
        let (guest_id, _) = state.add_participant("Guest").await.unwrap();

        let error = state.set_topic(&guest_id, "Story 123").await.unwrap_err();
        assert_eq!(error, "Only the host can set the topic.");

        state.set_topic(&host_id, "Story 123").await.unwrap();
        let snapshot = state.snapshot().await;
        match snapshot {
            ServerEvent::State { topic, .. } => {
                assert_eq!(topic, "Story 123");
            }
            _ => panic!("expected state snapshot"),
        }
    }
}
