use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ClientEvent {
    Vote { value: String },
    SetTopic { value: String },
    Reveal,
    Reset,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ServerEvent {
    Connected {
        session_id: String,
        is_host: bool,
        name: String,
    },
    State {
        participants: Vec<ParticipantView>,
        revealed: bool,
        votes: HashMap<String, String>,
        topic: String,
        host_session_id: Option<String>,
    },
    Error {
        message: String,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantView {
    pub name: String,
    pub voted: bool,
    pub is_host: bool,
}
